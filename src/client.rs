use std::ops::Deref;

use gloo_console::log;
use matrix_sdk::{
    config::SyncSettings,
    room::MessagesOptions,
    ruma::{
        events::{
            room::message::{sanitize::HtmlSanitizerMode, Relation, RoomMessageEventContent},
            AnyMessageLikeEvent, AnyTimelineEvent,
        },
        UserId,
    },
    Client, Session,
};
use ruma::{
    api::client::{
        filter::{FilterDefinition, LazyLoadOptions, RoomEventFilter, RoomFilter},
        sync::sync_events::v3::Filter,
    },
    events::{reaction::ReactionEventContent, room::message::sanitize::RemoveReplyFallback},
    EventId, OwnedEventId, RoomId, UInt,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use gloo_storage::{errors::StorageError, LocalStorage, Storage};

use crate::{round_robin_vec_merge, MatrixSocialError};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Post {
    pub sender_id: String,
    pub room_name: String,
    pub room_id: String,
    pub content: String,
    pub event_id: String,
    pub reply_to: Option<String>,
    pub reply_ids: Vec<String>,
    pub score: i32,
}

impl Post {
    fn increment_score(&mut self) {
        self.score = self.score + 1;
    }

    fn decrement_score(&mut self) {
        self.score = self.score - 1;
    }

    fn add_reply_id(&mut self, event_id: String) {
        self.reply_ids.push(event_id);
    }
}

pub async fn login(user_id: String, password: String) -> Result<String, String> {
    let user: &UserId = &UserId::parse(user_id.clone()).unwrap();
    let client: Client = Client::builder().user_id(user).build().await.unwrap();
    log!("Logging in with", &user_id);
    client.login_username(user, &password).send().await.unwrap();
    log!("Successfully logged in!");
    log!("syncing...");

    client.sync_once(get_sync_settings()).await.unwrap();
    log!("Successfully synced!");
    let access_token = client.access_token().unwrap();
    let device_id = client.device_id().unwrap();
    let session = client.session().unwrap();
    LocalStorage::set("matrix-social:session", session).unwrap();
    Ok(access_token)
}

pub async fn get_client() -> Result<Client, StorageError> {
    let stored_session: Value = LocalStorage::get("matrix-social:session")?;
    let session: Session = serde_json::from_value(stored_session).unwrap();
    let client: Client = Client::builder()
        .user_id(&session.user_id)
        .build()
        .await
        .unwrap();
    client.restore_login(session).await.unwrap();
    Ok(client)
}

pub async fn get_posts() -> Result<Vec<Post>, StorageError> {
    let client = get_client().await?;

    log!("Syncing...");

    let response = client.sync_once(get_sync_settings().clone()).await.unwrap();
    log!("Synced!");

    log!("Getting posts...");
    let mut posts: Vec<Vec<Post>> = vec![];
    for room in client.joined_rooms() {
        let room_name = room.display_name().await.unwrap().to_string();
        let room_id = room.room_id().to_string();
        log!(format!("Getting posts from \"{room_name}\" ({room_id})...",));

        let mut messages_options = MessagesOptions::backward();
        messages_options.limit = UInt::from(100u32);
        let messages = room.messages(messages_options).await.unwrap();
        let mut room_posts: Vec<Post> = vec![];

        for message in messages.chunk.iter().rev() {
            let event = match message.event.deserialize(){
                Ok(event) => {
                    event
                },
                Err(error) => {
                    log!("Error deserializing event: ", error.to_string(), "\n\n", message.event.json().to_string());
                    continue;
                }
           };
            let sender_name = event.sender().to_string();
            match event {
                AnyTimelineEvent::MessageLike(event) => match event {
                    AnyMessageLikeEvent::RoomMessage(event) => match event {
                        matrix_sdk::ruma::events::MessageLikeEvent::Original(event) => {
                            let content = event.content.body().to_string();
                            let (reply_to, content) = match event.clone().content.relates_to {
                                Some(relation) => match relation {
                                    Relation::Reply { in_reply_to } => {
                                        let mut event = event.clone();
                                        event.content.sanitize(
                                            HtmlSanitizerMode::Strict,
                                            RemoveReplyFallback::Yes,
                                        );
                                        (
                                            Some(in_reply_to.event_id.to_string()),
                                            event.content.body().to_string(),
                                        )
                                    }
                                    _ => (None, content),
                                },
                                None => (None, content),
                            };
                            room_posts.push(Post {
                                sender_id: sender_name,
                                room_name: room_name.clone(),
                                room_id: room_id.clone(),
                                content: content,
                                event_id: event.event_id.to_string(),
                                reply_to: reply_to,
                                reply_ids: vec![],
                                score: 0,
                            });
                        }
                        matrix_sdk::ruma::events::MessageLikeEvent::Redacted(_) => {}
                    },
                    AnyMessageLikeEvent::Reaction(event) => match event {
                        ruma::events::MessageLikeEvent::Original(event) => {
                            let reaction = event.content.relates_to.key;
                            for post in &mut room_posts {
                                if post.event_id == event.content.relates_to.event_id {
                                    if reaction == "👍️".to_string() {
                                        post.increment_score();
                                    } else if reaction == "👎️".to_string() {
                                        post.decrement_score();
                                    }
                                }
                            }
                        }
                        ruma::events::MessageLikeEvent::Redacted(_) => {}
                    },
                    _ => {}
                },
                AnyTimelineEvent::State(_) => {}
            }
        }

        let room_posts_clone = room_posts.clone();

        for original_post in &mut room_posts {
            for post in &room_posts_clone {
                match post.reply_to.clone() {
                    Some(reply_to) => {
                        if reply_to == original_post.event_id {
                            original_post.add_reply_id(post.event_id.clone());
                        }
                    }
                    None => {}
                }
            }
        }

        room_posts.reverse();
        posts.push(room_posts);
        log!(format!("Got posts from \"{room_name}\"!"));
    }
    let mixed_posts = round_robin_vec_merge(posts);
    LocalStorage::set("matrix-social:posts".to_string(), mixed_posts).unwrap();
    log!("Got posts!");
    Ok(vec![])
}

pub async fn react_to_event(
    room_id: String,
    event_id: String,
    reaction: String,
) -> Result<String, MatrixSocialError> {
    let client = get_client().await?;
    client.sync_once(get_sync_settings()).await.unwrap();
    let room_id = RoomId::parse(room_id).unwrap();
    let room = client.get_joined_room(&room_id).unwrap();
    let posts: Vec<Post> = LocalStorage::get("matrix-social:posts")?;

    for post in posts {
        if &post.event_id == &event_id {
            let event_id: OwnedEventId = EventId::parse(event_id.clone()).unwrap();
            let relation = ruma::events::reaction::Relation::new(event_id, reaction.clone());
            let reaction_content = ReactionEventContent::new(relation);

            room.send(reaction_content, None).await?;
            break;
        }
    }
    client.sync_once(get_sync_settings()).await.unwrap();
    Ok(String::from(""))
}

pub async fn send_message(room_id: String, body: String) -> Result<String, MatrixSocialError> {
    let client = get_client().await?;
    client.sync_once(get_sync_settings()).await.unwrap();
    let room_id = RoomId::parse(room_id).unwrap();
    let room = client.get_joined_room(&room_id).unwrap();
    let content = RoomMessageEventContent::text_plain(body);
    let resp = room.send(content, None).await?;
    Ok(resp.event_id.to_string())
}

pub async fn redact_event(room_id: String, event_id: String) {
    let client = get_client().await.unwrap();
    let event_id = EventId::parse(event_id).unwrap();
    let room_id = RoomId::parse(room_id).unwrap();
    client.sync_once(get_sync_settings()).await.unwrap();
    let room = client.get_joined_room(&room_id).unwrap();
    room.redact(&event_id, None, None).await.unwrap();
}

pub fn get_sync_settings() -> SyncSettings<'static> {
    let mut sync_settings_filter_definition = FilterDefinition::default();
    let mut room_filter = RoomFilter::default();
    let mut room_filter_timeline = RoomEventFilter::default();

    room_filter_timeline.limit = Some(UInt::from(100u32));
    room_filter.timeline = room_filter_timeline;
    sync_settings_filter_definition.room = room_filter;

    sync_settings_filter_definition
        .room
        .timeline
        .lazy_load_options = LazyLoadOptions::Enabled {
        include_redundant_members: false,
    };

    SyncSettings::new().filter(Filter::from(sync_settings_filter_definition))
}
