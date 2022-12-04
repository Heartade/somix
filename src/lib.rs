use gloo_console::log;
use yew::prelude::*;
use yew_hooks::prelude::*;

use matrix_sdk::{
    config::SyncSettings,
    deserialized_responses::SyncResponse,
    ruma::{
        events::{
            room::message::{MessageType, OriginalSyncRoomMessageEvent, SyncRoomMessageEvent},
            AnySyncMessageLikeEvent, AnySyncTimelineEvent, SyncMessageLikeEvent,
        },
        user_id, RoomId,
    },
    Client, LoopCtrl,
};

struct MatrixSocialClient(Client);

impl MatrixSocialClient {
    async fn on_room_message(&self, room_id: &RoomId, event: &OriginalSyncRoomMessageEvent) {
        let MessageType::Text(text_content) = &event.content.msgtype else { return };

        log!(&format!("received event {:?}", &text_content.body).to_string());
    }

    async fn on_sync_response(&self, response: SyncResponse) -> LoopCtrl {
        log!("synced");

        for (room_id, room) in response.rooms.join {
            for event in room.timeline.events {
                if let Ok(AnySyncTimelineEvent::MessageLike(
                    AnySyncMessageLikeEvent::RoomMessage(SyncMessageLikeEvent::Original(ev)),
                )) = event.event.deserialize()
                {
                    self.on_room_message(&room_id, &ev).await
                }
            }
        }

        LoopCtrl::Continue
    }
}

async fn async_func() -> Result<String, String> {
    let myuser = user_id!("@user:example.org");
    let client = match Client::builder().user_id(myuser).build().await {
        Ok(client) => client,
        Err(e) => {
            panic!("Error building client: {e}");
        }
    };

    client.login_username(myuser, "password").send().await;

    let ms_client = MatrixSocialClient(client.clone());

    let response = client.sync_once(SyncSettings::default()).await.unwrap();
    let settings = SyncSettings::default().token(response.next_batch);

    client
        .sync_with_callback(settings, |response| ms_client.on_sync_response(response))
        .await
        .unwrap();
    Ok("something".to_owned())
}

#[function_component(App)]
pub fn app() -> Html {
    log!("Rendering App");
    let state = use_async(async move { async_func().await });
    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
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
                    html! { "nothing" }
                }
            }
            </p>
        </div>
    }
}
