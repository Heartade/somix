use gloo_console::log;
use matrix_sdk::{
    config::SyncSettings,
    room::MessagesOptions,
    ruma::{
        events::{room::message::SyncRoomMessageEvent, AnyMessageLikeEvent, AnyTimelineEvent},
        UserId,
    },
    Client,
};
use serde_json::Value;

use gloo_storage::{LocalStorage, Storage};

pub async fn matrix_social_client() -> Result<String, String> {
    let username: &'static str = env!("MATRIX_SOCIAL_USER");
    let password: &'static str = env!("MATRIX_SOCIAL_PASS");
    let myuser = <&UserId>::try_from(username).unwrap();
    let client = match Client::builder().user_id(myuser).build().await {
        Ok(client) => client,
        Err(e) => {
            panic!("Error during client build: {e}");
        }
    };

    match client.login_username(myuser, password).send().await {
        Ok(_) => {}
        Err(e) => {
            panic!("Error during client login: {e}");
        }
    };

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
