mod client;

use crate::client::MatrixSocialClient;
use gloo_console::log;
use yew::prelude::*;
use yew_hooks::prelude::*;

use matrix_sdk::{
    config::SyncSettings,
    room::MessagesOptions,
    ruma::{events::room::message::SyncRoomMessageEvent, UserId},
    Client,
};

async fn async_func() -> Result<String, String> {
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

    let ms_client = MatrixSocialClient(client.clone());

    let response = client.sync_once(SyncSettings::default()).await.unwrap();
    let settings = SyncSettings::default().token(response.next_batch);

    //client
    //    .sync_with_callback(settings, |response| ms_client.on_sync_response(response))
    //    .await
    //    .unwrap();
    client.add_event_handler(|ev: SyncRoomMessageEvent| async move {
        println!("Received a message {:?}", ev);
    });
    client.sync_once(settings).await.unwrap();
    for room in client.joined_rooms() {
        let room_name = room.name().unwrap();
        log!("room:", room_name);
        let options = MessagesOptions::backward();
        let messages = room.messages(options).await;
        match messages {
            Ok(messages) => {
                for message in messages.chunk.iter() {
                    let message = format!("{:#?}", message.event.json());
                    log!("message:", message);
                }
            }
            Err(e) => {
                log!("Error during fetching of messages {}", e.to_string());
            }
        }
    }
    let a = client.joined_rooms().get(0).unwrap().name().unwrap();
    log!(a);
    Ok("synced".to_owned())
}

#[function_component(App)]
pub fn app() -> Html {
    log!("Rendering App");
    let state = use_async(async move { async_func().await });
    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            log!("Starting sync...");
            state.run();
        })
    };
    html! {
        <div>
            <button {onclick} class={classes!("button")}>{"Click"}</button>
            <p class={classes!("has-text-white")}>
            {
                if let Some(data) = &state.data {
                    html! { data }
                }
                else {
                    html! { }
                }
            }
            {
                if state.loading {
                    html! {
                        <div>
                            <p>{"Loading..."}</p>
                            <progress class="progress is-small is-primary" max="100">{"30%"}</progress>
                        </div>
                     }
                } else {
                    html! { }
                }
            }
            </p>
        </div>
    }
}
