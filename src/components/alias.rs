use ruma::RoomAliasId;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{client::get_client, Route};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub room_alias: String,
}

#[function_component(Alias)]
pub fn alias(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let room_alias = props.room_alias.clone();
    wasm_bindgen_futures::spawn_local(async move {
        let room_alias = format!("#{}", room_alias.clone());
        let client = get_client().await.unwrap();
        let room_alias = RoomAliasId::parse(room_alias).unwrap();
        let room_id = client
            .resolve_room_alias(&room_alias)
            .await
            .unwrap()
            .room_id
            .to_string();
        navigator.push(&Route::Room { room_id });
    });
    html! {
        <div class="flex justify-center">
            <span class="text-charm-400">{"Redirecting..."}</span>
        </div>
    }
}
