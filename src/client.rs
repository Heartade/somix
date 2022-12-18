use gloo_console::log;
use matrix_sdk::{
    config::SyncSettings,
    room::MessagesOptions,
    ruma::{
        events::{
            room::message::{Relation, RoomMessageEventContent},
            AnyMessageLikeEvent, AnyTimelineEvent,
        },
        UserId,
    },
    Client, Session,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use gloo_storage::{errors::StorageError, LocalStorage, Storage};

use crate::round_robin_vec_merge;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Post {
    pub sender_id: String,
    pub room_name: String,
    pub room_id: String,
    pub content: RoomMessageEventContent,
    pub event_id: String,
    pub reply_to: Option<String>,
}

pub async fn login(user_id: String, password: String) -> Result<String, String> {
    let user: &UserId = &UserId::parse(user_id.clone()).unwrap();
    let client: Client = Client::builder().user_id(user).build().await.unwrap();
    log!("Logging in with", &user_id);
    client.login_username(user, &password).send().await.unwrap();
    log!("Successfully logged in!");
    log!("syncing...");
    client.sync_once(SyncSettings::default()).await.unwrap();
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
        for message in messages.chunk.iter() {
            let event = message.event.deserialize().unwrap();
            let sender_name = event.sender().to_string();
            match event {
                AnyTimelineEvent::MessageLike(event) => match event {
                    AnyMessageLikeEvent::RoomMessage(event) => match event {
                        matrix_sdk::ruma::events::MessageLikeEvent::Original(event) => {
                            let reply_to: Option<String> = match event.clone().content.relates_to {
                                Some(relation) => match relation {
                                    Relation::Reply { in_reply_to } => {
                                        Some(in_reply_to.event_id.to_string())
                                    }
                                    _ => None,
                                },
                                None => None,
                            };
                            room_posts.push(Post {
                                sender_id: sender_name,
                                room_name: room_name.clone(),
                                room_id: room_id.clone(),
                                content: event.content,
                                event_id: event.event_id.to_string(),
                                reply_to: reply_to,
                            });
                        }
                        matrix_sdk::ruma::events::MessageLikeEvent::Redacted(_) => {}
                    },
                    _ => {}
                },
                AnyTimelineEvent::State(_) => {}
            }
        }
        posts.push(room_posts);
        log!(format!("Got posts from \"{room_name}\"!"));
    }
    let mixed_posts = round_robin_vec_merge(posts);
    LocalStorage::set("matrix-social:posts".to_string(), mixed_posts).unwrap();
    log!("Got posts!");
    Ok(vec![])
}
