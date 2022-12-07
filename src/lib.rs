mod client;

use crate::client::MatrixSocialClient;
use gloo_console::log;
use serde_json::Value;
use yew::prelude::*;
use yew_hooks::prelude::*;

use matrix_sdk::{
    config::SyncSettings,
    room::MessagesOptions,
    ruma::{events::{room::message::{SyncRoomMessageEvent, self}, MessageLikeEvent, AnyTimelineEvent, AnyMessageLikeEvent}, UserId},
    Client,
};

use gloo_storage::{LocalStorage, Storage};

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
                        AnyTimelineEvent::MessageLike(event) => {
                            match event {
                                AnyMessageLikeEvent::RoomMessage(event) => {
                                    events_.push(message);
                                    //LocalStorage::set("matrix-social:posts", message).ok();
                                },
                                _ => todo!(),
                            }
                        },
                        AnyTimelineEvent::State(_) => {},
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

#[function_component(App)]
pub fn app() -> Html {
    log!("Rendering App");
    let message_history = use_local_storage::<String>("matrix-social:posts".to_string());
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
            <Nav></Nav>
            <br/>
            <div class="columns has-text-centered is-centered">
                <div class="column is-two-fifths">
                <br/>
                    <div>
            <button {onclick} class="button is-primary has-text-dark">{"Load Messages"}</button>
            <div /><br />
            <p class={classes!("has-text-white")}>
            {
                if let Some(data) = &state.data {
                    let data: Vec<Value> = serde_json::from_str(data).unwrap();
                    data.into_iter().map(|event| {
                            html!{
                                <article class="message">
                                <div class="message-header is-dark has-text-primary">
                                <p>{
                                    html!{
                                        &event["sender"].to_string()
                                    }
                                }</p>
                                </div>
                                <div class="message-body has-text-primary has-background-dark">
                                {
                                    html!{
                                        &event["content"]["body"].to_string()
                                    }
                                }
                                </div>
                                </article>
                            }
                        }).collect::<Html>()
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
                </div>
                <div class="column is-one-fifth">
                    <p class="box has-background-dark has-text-primary">{"Lorem ipsum"}</p>
                </div>
            </div>
        </div>
    }
}


#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <div>
            <nav class="navbar is-fixed-top is-dark">
                <div class="navbar-brand">
                    <div class="navbar-item">
                        <a class="title has-text-primary">{"matrix-social"}</a>
                    </div>
                </div>
                <div class="navbar-menu is-active">
                    <div class="navbar-start">
                        <a class="navbar-item has-text-primary">{"Feed"}</a>
                    </div>
                    <div class="navbar-end">
                        <div class="navbar-item has-text-primary">
                            { "Username Placeholder" }
                        </div>
                        <div class="navbar-item">
                            <a class="button is-primary has-text-dark">{"Login"}</a>
                        </div>
                    </div>
                </div>
            </nav>
        </div>
    }
}
