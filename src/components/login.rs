use std::ops::Deref;

use gloo_console::log;
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
    <div>
      <div class="columns has-text-centered is-centered">
        <div class="column is-two-fifths">
          <br />
          <div>
            <p class="title has-text-primary">{"Login"}</p>
            <div>
              <div class="field">
                <label class="label has-text-primary has-text-left">{"User Id:"}</label>
                <div class="control">
                  <TextInput class="input is-primary" type_="text" placeholder="@user:example.com" onchange={user_id_onchange} />
                </div>
              </div>

              <div class="field">
                <label class="label has-text-primary has-text-left">{"Password:"}</label>
                <div class="control">
                  <TextInput class="input is-primary" type_="password" onchange={password_onchange} />
                </div>
              </div>

              <div class="field is-grouped">
                <div class="control">
                {
                  if  *loading_state {
                    html! {<button class="button is-primary is-loading" >{"Loading"}</button>}
                  }
                  else {
                    html! {<button class="button is-primary" onclick={login_onclick}>{"Login"}</button>}
                  }
                }
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
    }
}
