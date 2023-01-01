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
        filter::{EventFormat, FilterDefinition, LazyLoadOptions, RoomEventFilter, RoomFilter},
        sync::sync_events::v3::Filter,
    },
    events::room::message::sanitize::RemoveReplyFallback,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use gloo_storage::{errors::StorageError, LocalStorage, Storage};

use crate::round_robin_vec_merge;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Post {
    pub sender_id: String,
    pub room_name: String,
    pub room_id: String,
    pub content: String,
    pub event_id: String,
    pub reply_to: Option<String>,
    pub score: i32,
}

impl Post {
    fn increment_score(&mut self) {
        self.score = self.score + 1;
    }

    fn decrement_score(&mut self) {
        self.score = self.score - 1;
    }
}

pub async fn login(user_id: String, password: String) -> Result<String, String> {
    let user: &UserId = &UserId::parse(user_id.clone()).unwrap();
    let client: Client = Client::builder().user_id(user).build().await.unwrap();
    log!("Logging in with", &user_id);
    client.login_username(user, &password).send().await.unwrap();
    log!("Successfully logged in!");
    log!("syncing...");

    let mut sync_settings_filter_definition = FilterDefinition::empty();
    sync_settings_filter_definition.room = RoomFilter::empty();
    sync_settings_filter_definition.room.timeline = RoomEventFilter::empty();
    sync_settings_filter_definition
        .room
        .timeline
        .lazy_load_options = LazyLoadOptions::Enabled {
        include_redundant_members: false,
    };
    let sync_settings = SyncSettings::new().filter(Filter::from(sync_settings_filter_definition));

    client.sync_once(sync_settings).await.unwrap();
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
    let response = client.sync_once(SyncSettings::default()).await.unwrap();
    client
        .sync_once(SyncSettings::default().token(response.next_batch))
        .await
        .unwrap();
    log!("Synced!");

    log!("Getting posts...");
    let mut posts: Vec<Vec<Post>> = vec![];
    for room in client.joined_rooms() {
        let room_name = room.name().unwrap();
        let room_id = room.room_id().to_string();
        log!(format!("Getting posts from \"{room_name}\"...",));
        let messages = room.messages(MessagesOptions::backward()).await.unwrap();
        let mut room_posts: Vec<Post> = vec![];
        for message in messages.chunk.iter().rev() {
            let event = message.event.deserialize().unwrap();
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
        room_posts.reverse();
        posts.push(room_posts);
        log!(format!("Got posts from \"{room_name}\"!"));
    }
    let mixed_posts = round_robin_vec_merge(posts);
    LocalStorage::set("matrix-social:posts".to_string(), mixed_posts).unwrap();
    log!("Got posts!");
    Ok(vec![])
}
