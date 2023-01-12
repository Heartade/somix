use std::ops::Deref;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::text_input::TextInput;

use crate::{client, Route};

#[function_component(Login)]
pub fn login() -> Html {
    let navigator = use_navigator().unwrap();
    let user_id_state = use_state(|| "".to_owned());
    let password_state = use_state(|| "".to_owned());
    let loading_state = use_state(|| false);

    let user_id_onchange = {
        let user_id_state = user_id_state.clone();
        Callback::from(move |value| {
            user_id_state.set(value);
        })
    };
    let password_onchange = {
        let password_state = password_state.clone();
        Callback::from(move |value| {
            password_state.set(value);
        })
    };

    let login_onclick = {
        let user_id_state = user_id_state.clone();
        let password_state = password_state.clone();
        let loading_state = loading_state.clone();
        Callback::from(move |_| {
            let user_id_state = user_id_state.clone();
            let password_state = password_state.clone();
            let loading_state = loading_state.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                loading_state.set(true);
                client::login(
                    user_id_state.deref().to_string(),
                    password_state.deref().to_string(),
                )
                .await
                .unwrap();
                client::get_posts().await.unwrap();
                navigator.push(&Route::Home);
            });
        })
    };

    html! {
    <div class="flex w-screen justify-center py-5">
      <div class="flex flex-col space-y-10 min-w-fit max-w-md w-1/2 p-8 border border-tuatara-400 bg-tuatara-700 text-charm-400">
        <span class="text-4xl text-bold text-center pb-4">{"Login"}</span>
        <div class="flex flex-col space-y-4">
          <span>{"User Id"}</span>
          <TextInput class="bg-tuatara-500 border border-tuatara-400 text-sm rounded-md focus:ring-charm-400 focus:border-charm-400 block w-full p-2.5"
                     type_="text" placeholder="@user:example.com" onchange={user_id_onchange.clone()} />
        </div>
        <div class="flex flex-col space-y-4">
          <span>{"Password"}</span>
          <TextInput class="bg-tuatara-500 border border-tuatara-400 text-sm rounded-md focus:ring-charm-400 focus:border-charm-400 block w-full p-2.5"
                     type_="password" onchange={password_onchange.clone()} />
        </div>
        <div class="flex flex-col space-y-4">
          {
            if *loading_state {
              html! {
                <button class="flex justify-center space-x-2 border rounded py-2 border-charm-400 bg-tuatara-600"
                        disabled=true>
                  <span>{"Loading"}</span>
                  <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"
                      class="animate-spin w-6 h-6 stroke-charm-400">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99" />
                  </svg>
                </button>
              }
            } else {
              html! {
                <button class="border rounded py-2 border-charm-400 hover:text-charm-300 hover:border-charm-300 bg-tuatara-600 hover:bg-tuatara-500"
                        onclick={login_onclick.clone()}>
                  {"Login"}
                </button>
              }
            }
          }
          <a href="https://joinmatrix.org/" class="text-center hover:text-charm-300">{"New to Matrix?"}</a>
        </div>
      </div>
    </div>
    }
}
