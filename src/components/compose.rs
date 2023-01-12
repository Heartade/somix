use std::ops::Deref;

use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    client::{get_posts, send_message},
    Route,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub room_id: String,
}

#[function_component(Compose)]
pub fn compose(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let compose_textarea_state = use_state(|| "".to_string());
    let loading_state = use_state(|| false);
    let compose_onchange = {
        let compose_textarea_state = compose_textarea_state.clone();
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            compose_textarea_state.set(value);
        })
    };
    let send_onclick = {
        let compose_textarea_state = compose_textarea_state.clone();
        let loading_state = loading_state.clone();
        let room_id = props.room_id.clone();
        let navigator = navigator.clone();
        Callback::from(move |_| {
            let compose_textarea_state = compose_textarea_state.clone();
            let loading_state = loading_state.clone();
            let room_id = room_id.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                loading_state.set(true);
                let event_id = send_message(room_id, compose_textarea_state.deref().to_string())
                    .await
                    .unwrap();
                get_posts().await.unwrap();
                navigator.push(&Route::Event { event_id })
            });
        })
    };
    html! {
        <div class="flex justify-center w-full">
            <div class="w-2/3 md:w-3/5 lg:w-1/2 xl:w-2/5 flex flex-col gap-2 border border-tuatara-400 bg-tuatara-600 rounded p-2">
                <textarea class="w-full h-full bg-tuatara-700 text-charm-400 placeholder:text-charm-500 p-2 rounded border border-tuatara-300"
                          placeholder="Say something..." rows=10 onchange={compose_onchange}>
                </textarea>
                <div class="flex justify-end bg-tuatara-600">
                {
                    if *loading_state {
                        html! {
                            <button class="flex gap-2 bg-tuatara-500 border rounded-2xl border-charm-400 px-6 py-2 group">
                                <span class="text-charm-400 group-hover:text-charm-300">{"Loading"}</span>
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"
                                    class="animate-spin w-6 h-6 stroke-charm-400">
                                  <path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99" />
                                </svg>
                            </button>
                        }
                    } else {
                        html! {
                            <button onclick={send_onclick} class="flex gap-2 bg-tuatara-500 border rounded-2xl border-charm-400 px-6 py-2 group hover:border-charm-300">
                                <span class="text-charm-400 group-hover:text-charm-300">{"Send"}</span>
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"
                                     class="w-6 h-6 stroke-charm-400 group-hover:stroke-charm-300">
                                  <path stroke-linecap="round" stroke-linejoin="round" d="M6 12L3.269 3.126A59.768 59.768 0 0121.485 12 59.77 59.77 0 013.27 20.876L5.999 12zm0 0h7.5" />
                                </svg>
                            </button>
                        }
                    }
                }
                </div>
            </div>
        </div>
    }
}
