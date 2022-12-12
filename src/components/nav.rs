use gloo_console::log;
use gloo_storage::{LocalStorage, Storage};
use matrix_sdk::Session;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{Route, BASE_URL};

#[function_component(Nav)]
pub fn nav() -> Html {
    let navigator = use_navigator().unwrap();
    let logout = Callback::from(move |_| {
        LocalStorage::delete("matrix-social:session");
        LocalStorage::delete("matrix-social:posts");
        log!("Logged out");
        navigator.push(&Route::Login);
    });

    let logged_in: Html = match LocalStorage::get("matrix-social:session") {
        Ok(session) => {
            let session: Session = session;
            html! {
                <>
                <div class="navbar-item has-text-primary">{ session.user_id.to_string() }</div>
                <div class="navbar-item">
                    <button class="button is-danger has-text-dark" onclick={logout}>{"Logout"}</button>
                </div>
                </>
            }
        }
        Err(_) => html! {
            <>
            <div class="navbar-item has-text-primary">{ "Not Logged in" }</div>
            <div class="navbar-item">
                <a class="button is-primary has-text-dark" href={BASE_URL.to_owned()+"/login"}>{"Login"}</a>
            </div>
            </>
        },
    };

    html! {
        <div>
            <nav class="navbar is-fixed-top is-dark">
                <div class="navbar-brand">
                    <div class="navbar-item">
                        <a class="title has-text-primary" href={BASE_URL.to_owned()+"/"}>{"matrix-social"}</a>
                    </div>
                </div>
                <div class="navbar-menu is-active">
                    <div class="navbar-start">
                        <a class="navbar-item has-text-primary" href={BASE_URL.to_owned()+"/feed"}>{"Feed"}</a>
                    </div>
                    <div class="navbar-end">
                        { logged_in }
                    </div>
                </div>
            </nav>
        </div>
    }
}
