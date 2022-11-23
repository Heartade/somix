use gloo_console::log;
use reqwest_wasm::{self};
use yew::prelude::*;
use yew_hooks::prelude::*;

async fn get_guest(
    homeserver: String,
    storage: &UseLocalStorageHandle<String>,
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
    storage.set(res.clone());
    Ok(res)
}

#[function_component(Guest)]
pub fn guest() -> Html {
    let storage = use_local_storage::<String>("user".to_string());

    let state =
        use_async(async move { get_guest("https://matrix.org".to_owned(), &storage).await });
    
    if use_is_first_mount() {
        let state = state.clone();
        state.run();
    }
    html! {}
}
