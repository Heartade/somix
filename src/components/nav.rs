use gloo_console::log;
use gloo_storage::{LocalStorage, Storage};
use matrix_sdk::Session;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Nav)]
pub fn nav() -> Html {
    let navigator = use_navigator().unwrap();
    let _location = use_location().unwrap().query_str();
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
                <Link<Route> to={Route::Login} classes={classes!("button", "is-primary", "has-text-dark")}>{ "Login" }</Link<Route>>
            </div>
            </>
        },
    };

    html! {
        <div>
            <nav class="navbar is-fixed-top is-dark">
                <div class="navbar-brand">
                    <div class="navbar-item">
                        <Link<Route> to={Route::Home} classes={classes!("title", "has-text-primary")}>{ "matrix-social" }</Link<Route>>
                    </div>
                </div>
                <div class="navbar-menu is-active">
                    <div class="navbar-start">
                        <Link<Route> to={Route::Feed} classes={classes!("navbar-item", "has-text-primary")}>{ "Feed" }</Link<Route>>
                    </div>
                    <div class="navbar-end">
                        { logged_in }
                    </div>
                </div>
            </nav>
        </div>
    }
}
