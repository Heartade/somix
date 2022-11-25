use std::rc::Rc;

use gloo_console::log;
use reqwest_wasm::{self};
use yew::prelude::*;
use yew_hooks::prelude::*;
use yewdux::{prelude::*, dispatch};

use crate::store::user::UserStore;

async fn get_guest(
    homeserver: String,
    user: Rc<UserStore>,
    dispatch: Dispatch<UserStore>
) -> Result<String, String> {
    log!(format!("Getting guest user from {homeserver}"));
    let client = reqwest_wasm::Client::new();
    let res = client
        .post(format!(
            "{homeserver}/_matrix/client/r0/register?kind=guest"
        ))
        .body("{}")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    log!(format!("Got user: {res}"));
    let res2 = res.clone();
    dispatch.reduce_mut_callback(
        move |user| *user = serde_json::from_str(&res2).unwrap()
    ).emit("");
    Ok(res)
}

#[function_component(Guest)]
pub fn guest() -> Html {
    let (user, dispatch) = use_store::<UserStore>();
    let state =
        use_async(async move { get_guest("https://matrix.org".to_owned(), user, dispatch).await });
    
    if use_is_first_mount() {
        let state = state.clone();
        state.run();
    }
    html! {}
}
