use std::ops::Deref;

use matrix_sdk::room::Room;
use yew::prelude::*;

use crate::components::{compose::Compose, room_selector::RoomSelector};

#[function_component(New)]
pub fn new() -> Html {
    let room_id_state = use_state(|| "".to_string());
    let room_selected = {
        let room_id_state = room_id_state.clone();
        Callback::from(move |room: Room| {
            let room_id_state = room_id_state.clone();
            room_id_state.set(room.room_id().to_string());
        })
    };
    html! {
        <div class="flex flex-wrap justify-center gap-4">
            <RoomSelector onchange={room_selected} />
            <Compose room_id={room_id_state.deref().clone()}/>
        </div>
    }
}
