use gloo_console::log;
use gloo_storage::{errors::StorageError, LocalStorage, Storage};
use matrix_sdk::{Client, ClientBuildError, config::SyncSettings, HttpError, room::MessagesOptions, ruma::{
    events::{
        AnyMessageLikeEvent,
        AnyTimelineEvent, room::message::{Relation, RoomMessageEventContent, sanitize::HtmlSanitizerMode},
    },
    UserId,
}, Session};
use ruma::{api::client::{
    filter::{FilterDefinition, LazyLoadOptions, RoomEventFilter, RoomFilter},
    sync::sync_events::v3::Filter,
}, EventId, events::{
    OriginalMessageLikeEvent,
    reaction::ReactionEventContent,
    room::message::{ForwardThread, sanitize::RemoveReplyFallback, TextMessageEventContent},
}, OwnedEventId, OwnedRoomId, OwnedUserId, RoomId, UInt};
use ruma::api::client::relations::get_relating_events;
use ruma::events::reaction::ReactionEvent;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{error_alert, round_robin_vec_merge, SomixError};

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
    pub source: OriginalMessageLikeEvent<RoomMessageEventContent>,
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
    let user_id: OwnedUserId = match UserId::parse(user_id.clone()) {
        Ok(user_id) => user_id,
        Err(e) => {
            error_alert(e.into());
            panic!();
        }
    };
    let user_id = &user_id;
    let server_name = user_id.server_name();
    let client: Client = match Client::builder()
        .server_name(server_name)
        .build()
        .await {
        Ok(client) => client,
        Err(e) => {
            match e {
                ClientBuildError::Http(e) => match e {
                    HttpError::Reqwest(e) => {
                        error_alert(e.into());
                        panic!();
                    }
                    _ => { panic!(); }
                },

                _ => { panic!(); }
            }
        }
    };
    log!("Logging in with", user_id.to_string());
    match client
        .login_username(user_id, &password)
        .initial_device_display_name("somix")
        .send()
        .await {
        Ok(_) => {}
        Err(e) => {
            error_alert(e.into());
        }
    }
    log!("Successfully logged in!");
    log!("syncing...");

    client.sync_once(get_sync_settings()).await.unwrap();
    log!("Successfully synced!");
    let access_token = client.access_token().unwrap();
    let session = client.session().unwrap();
    LocalStorage::set("somix:session", session).unwrap();
    Ok(access_token)
}

pub async fn get_client() -> Result<Client, StorageError> {
    let stored_session: Value = LocalStorage::get("somix:session")?;
    let session: Session = serde_json::from_value(stored_session).unwrap();
    let client: Client = Client::builder()
        .server_name(session.user_id.server_name())
        .build()
        .await
        .unwrap();
    client.restore_login(session).await.unwrap();
    Ok(client)
}

pub async fn get_posts() -> Result<Vec<Post>, StorageError> {
    let client = get_client().await?;

    log!("Syncing...");

    client.sync_once(get_sync_settings().clone()).await.unwrap();
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
            let event = match message.event.deserialize() {
                Ok(event) => event,
                Err(error) => {
                    log!(
                        "Error deserializing event: ",
                        error.to_string(),
                        "\n\n",
                        message.event.json().to_string()
                    );
                    continue;
                }
            };
            let sender_name = event.sender().to_string();
            match event {
                AnyTimelineEvent::MessageLike(event) => match event {
                    AnyMessageLikeEvent::RoomMessage(event) => match event {
                        ruma::events::MessageLikeEvent::Original(event) => {
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
                                content,
                                event_id: event.event_id.to_string(),
                                reply_to,
                                reply_ids: vec![],
                                score: 0,
                                source: event,
                            });
                        }
                        ruma::events::MessageLikeEvent::Redacted(_) => {}
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
    LocalStorage::set("somix:posts".to_string(), mixed_posts).unwrap();
    log!("Got posts!");
    Ok(vec![])
}

pub async fn react_to_event(
    room_id: String,
    event_id: String,
    reaction: String,
) -> Result<String, SomixError> {
    let client = get_client().await?;
    match client.sync_once(get_sync_settings()).await {
        Ok(_) => {}
        Err(e) => {
            error_alert(e.into());
        }
    }
    let room_id = RoomId::parse(room_id).unwrap();
    let room = client.get_joined_room(&room_id).unwrap();
    let posts: Vec<Post> = LocalStorage::get("somix:posts")?;

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

pub async fn send_message(room_id: String, body: String) -> Result<String, SomixError> {
    let client = get_client().await?;
    client.sync_once(get_sync_settings()).await.unwrap();
    let room_id = RoomId::parse(room_id).unwrap();
    let room = client.get_joined_room(&room_id).unwrap();
    let content = RoomMessageEventContent::text_plain(body);
    let resp = room.send(content, None).await?;
    Ok(resp.event_id.to_string())
}

pub async fn reply_to_message(post: Post, body: String) -> Result<String, SomixError> {
    let client = get_client().await?;
    client.sync_once(get_sync_settings()).await?;
    let room_id = RoomId::parse(post.room_id.clone()).unwrap();
    let room = client.get_joined_room(&room_id).unwrap();
    let message_content = TextMessageEventContent::plain(body);
    let message = ruma::events::room::message::MessageType::Text(message_content);
    let content = RoomMessageEventContent::reply(message, &post.source, ForwardThread::No);
    let resp = room.send(content, None).await.unwrap();
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

pub async fn get_room_info(room_id: String) -> (String, String, String) {
    let client = get_client().await.unwrap();
    client.sync_once(get_sync_settings()).await.unwrap();
    let room_id = RoomId::parse(room_id.clone()).unwrap();
    let room = client.get_room(&room_id).unwrap();
    let room_name = room.display_name().await.unwrap().to_string();
    let room_desc = match room.topic() {
        Some(desc) => desc,
        None => "".to_string(),
    };
    let room_avatar_url = match room.avatar_url() {
        Some(u) => {
            format!(
                "https://{}/_matrix/media/r0/thumbnail/{}/{}?width=256&height=256",
                u.server_name().unwrap(),
                u.server_name().unwrap(),
                u.media_id().unwrap().to_string()
            )
        }
        None => "assets/logo_128x128.webp".to_string().to_string(),
    };
    (room_name, room_desc, room_avatar_url)
}

pub async fn get_post_info(event_id: OwnedEventId, room_id: OwnedRoomId) -> Result<String, String> {
    let client = get_client().await.unwrap();
    let req = get_relating_events::v1::Request::new(&room_id, &event_id);
    let resp = client.send(req, None).await.unwrap();
    let mut score = 0;
    for event in resp.chunk {
        let event = event.deserialize().unwrap();
        match event {
            AnyMessageLikeEvent::Reaction(event) => {
                match event {
                    ReactionEvent::Original(event) => {
                        if event.content.relates_to.key.clone() == "👍️".to_string() {
                            score += 1;
                        } else if event.content.relates_to.key.clone() == "👎️".to_string() {
                            score += -1;
                        } else {}
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    Ok(score.to_string())
}
