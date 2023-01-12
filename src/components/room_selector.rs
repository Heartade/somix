use std::{collections::HashMap, ops::Deref};

use matrix_sdk::room::Room;
use yew::prelude::*;

use crate::client::{get_client, get_sync_settings};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub onchange: Callback<Room>,
}

#[function_component(RoomSelector)]
pub fn room_selector(props: &Props) -> Html {
    let list_open_state = use_state(|| false);
    let toggle_button = {
        let list_open_state = list_open_state.clone();
        Callback::from(move |_| list_open_state.set(!*list_open_state))
    };
    let loading_state = use_state(|| true);
    let rooms_state: UseStateHandle<Vec<Room>> = use_state(|| vec![]);
    let room_display_names_state = use_state(|| HashMap::new());
    let current_selected_state = use_state(|| "None".to_string());
    {
        let rooms_state = rooms_state.clone();
        let loading_state = loading_state.clone();
        let room_display_names_state = room_display_names_state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let client = get_client().await.unwrap();
            client.sync_once(get_sync_settings()).await.unwrap();
            let mut rooms = client.rooms();
            let mut room_names = HashMap::new();
            for room in rooms.clone() {
                let room_name = room.display_name().await.unwrap().to_string();
                room_names.insert(room.room_id().to_string(), room_name);
            }
            rooms.sort_by_key(|room| room_names.get(&room.room_id().to_string()).unwrap());

            rooms_state.set(rooms);
            loading_state.set(false);
            room_display_names_state.set(room_names);
        });
    }
    html! {
        <div class="text-charm-400 relative bg-tuatara-600 border border-tuatara-400 rounded p-4 w-96">
            <div class="">
            {
                {
                    let current_selected_state = current_selected_state.clone();
                    html! {
                        <span class="justify-self-end">{"Room: "}{current_selected_state.deref().clone()}</span>
                    }
                }
            }
                <button class="flex justify-between" onclick={let toggle_button = toggle_button.clone(); Callback::from(move |_| toggle_button.clone().emit("".to_string()))}>
                    <span>{"Choose Room"}</span>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"
                         class="w-6 h-6 stroke-charm-400">
                      <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 8.25l-7.5 7.5-7.5-7.5" />
                    </svg>
                </button>
            </div>
            {
                if *list_open_state {
                    html! {
                        <div class="relative bg-tuatara-500 flex flex-col drop-shadow rounded border border-tuatara-300">
                            {   if *loading_state {
                                    html! { <span>{"Loading..."}</span> }
                                } else {
                                    rooms_state.deref().clone().into_iter().map(|room| {
                                        let onclick = {
                                            let onchange = props.onchange.clone();
                                            let room = room.clone();
                                            let current_selected_state = current_selected_state.clone();
                                            let room_display_names_state = room_display_names_state.clone();
                                            let toggle_button = toggle_button.clone();
                                            Callback::from(move |_| {
                                                let room_id = room.room_id().to_string();
                                                current_selected_state.set(room_display_names_state.deref().clone().get(&room_id).unwrap().to_string());
                                                onchange.emit(room.clone());
                                                toggle_button.emit("".to_string());
                                            })
                                        };
                                        html! {
                                            <button class="hover:bg-tuatara-400 hover:text-charm-300" onclick={onclick}>
                                                <span>{room_display_names_state.deref().clone().get(&room.room_id().to_string()).unwrap().to_string()}</span>
                                            </button>
                                        }
                                    }).collect::<Html>()
                                }
                            }
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
