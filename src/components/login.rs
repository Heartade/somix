use std::ops::Deref;

use gloo_console::log;
use yew::prelude::*;

use crate::components::text_input::TextInput;

use crate::client;

#[function_component(Login)]
pub fn login() -> Html {
    let user_id_state = use_state(|| "".to_owned());
    let password_state = use_state(|| "".to_owned());

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
        Callback::from(move |_| {
            let user_id_state = user_id_state.clone();
            let password_state = password_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                client::login(
                    user_id_state.deref().to_string(),
                    password_state.deref().to_string(),
                )
                .await
                .unwrap();
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
                  <button class="button is-primary" onclick={login_onclick}>{"Login"}</button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
    }
}
