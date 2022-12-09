use gloo_console::log;
use matrix_sdk::{
    config::SyncSettings,
    room::MessagesOptions,
    ruma::{
        device_id,
        events::{room::message::SyncRoomMessageEvent, AnyMessageLikeEvent, AnyTimelineEvent},
        exports::serde::Deserialize,
        serde::test::serde_json_eq,
        DeviceId, OwnedDeviceId, UserId,
    },
    Client, Session,
};
use serde_json::Value;

use gloo_storage::{LocalStorage, Storage};

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

pub async fn get_client() -> Result<Client, Client> {
    let stored_session: Value = LocalStorage::get("matrix-social:session").unwrap();
    let session: Session = serde_json::from_value(stored_session).unwrap();
    let client: Client = Client::builder()
        .user_id(&session.user_id)
        .build()
        .await
        .unwrap();
    client.restore_login(session).await.unwrap();
    Ok(client)
}

pub async fn matrix_social_client() -> Result<String, String> {
    let client = get_client().await.unwrap();

    let response = client.sync_once(SyncSettings::default()).await.unwrap();
    let settings = SyncSettings::default().token(response.next_batch);
    client.add_event_handler(|ev: SyncRoomMessageEvent| async move {
        println!("Received a message {:?}", ev);
    });
    client.sync_once(settings).await.unwrap();
    for room in client.joined_rooms() {
        let room_name = room.name().unwrap();
        log!("room:", room_name);
        let options = MessagesOptions::backward();
        let messages = room.messages(options).await;
        let mut events_: Vec<Value> = vec![];
        match messages {
            Ok(messages) => {
                for message in messages.chunk.iter() {
                    let event = message.event.deserialize().unwrap();
                    let message = message.event.json().get();
                    let message: Value = serde_json::from_str(message).unwrap();
                    log!("message:", &message.to_string());
                    match event {
                        AnyTimelineEvent::MessageLike(event) => match event {
                            AnyMessageLikeEvent::RoomMessage(_event) => {
                                events_.push(message);
                            }
                            _ => todo!(),
                        },
                        AnyTimelineEvent::State(_) => {}
                    }
                }
            }
            Err(e) => {
                log!("Error during fetching of messages {}", e.to_string());
            }
        }
        LocalStorage::set("matrix-social:posts", events_).ok();
    }
    let a = client.joined_rooms().get(0).unwrap().name().unwrap();
    log!(a);
    let s: Value = LocalStorage::get("matrix-social:posts").unwrap();
    let s: String = s.to_string();
    Ok(s)
}
