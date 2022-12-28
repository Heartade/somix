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

    let logged_in = match LocalStorage::get("matrix-social:session") {
        Ok(session) => {
            let session: Session = session;
            Some(session.user_id.to_string())
        }
        Err(_) => None,
    };

    html! {
        <nav class="top-0 sticky bg-tuatara-600">
            <div class="px-8 mx-auto">
                <div class="flex justify-between items-center">
                    <div class="flex space-x-4">
                        <div>
                            <Link<Route> to={Route::Home} classes={classes!(String::from("flex items-center py-4 px-2 text-charm-400 hover:text-charm-300"))}>
                                <img src="assets/logo_128x128.webp" class="h-7 w-7 mr-1"/>
                                <span class="font-bold">{"matrix-social"}</span>
                            </Link<Route>>
                        </div>
                        <div class="hidden md:flex items-center space-x-1">
                            <Link<Route> to={Route::Feed} classes={classes!(String::from("py-4 px-3 text-charm-400 hover:text-charm-300"))}>
                                {"Feed"}
                            </Link<Route>>
                        </div>
                    </div>
                    <div class="hidden md:flex space-x-4 items-center">
                    {
                        match logged_in.clone() {
                            Some(user_id) => html! {
                                <>
                                    <div class="items-center space-x-1">
                                        <span class="py-4 px-3 text-charm-400">{user_id}</span>
                                    </div>
                                    <div class="items-center space-x-1">
                                        <button class="py-4 px-3 font-bold text-stiletto-500 hover:text-stiletto-400" onclick={logout.clone()}>{"Logout"}</button>
                                    </div>
                                </>
                            },
                            None => html! {
                                <div class="items-center space-x-1">
                                    <Link<Route> to={Route::Login} classes={classes!(String::from("py-4 px-3 font-bold text-charm-400 hover:text-charm-300"))}>
                                        {"Login"}
                                    </Link<Route>>
                                </div>
                            }
                        }
                    }
                    </div>
                    <div class="md:hidden flex items-center">
                        <button class="mobile-menu-button">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"
                                class="w-6 h-6 stroke-charm-400 hover:stroke-charm-300">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
            <div class="mobile-menu hidden md:hidden">
                <Link<Route> to={Route::Feed} classes={classes!(String::from("py-4 px-4 block text-charm-400 hover:text-charm-300 hover:bg-tuatara-700"))}>
                    {"Feed"}
                </Link<Route>>
                {
                    match logged_in {
                        Some(user_id) => html! {
                            <>
                                <button class="py-4 px-4 block font-bold hover:bg-tuatara-700 text-stiletto-500 hover:text-stiletto-400" onclick={logout}>{"Logout"}</button>
                                <div class="flex justify-center items-center">
                                    <span class="py-4 px-3 text-charm-400">{user_id}</span>
                                </div>
                            </>
                        },
                        None => html! {
                            <Link<Route> to={Route::Login} classes={classes!(String::from("py-4 px-4 block font-bold text-charm-400 hover:text-charm-300 hover:bg-tuatara-700"))}>
                                {"Login"}
                            </Link<Route>>
                        }
                    }
                }
            </div>
            <script type="text/javascript" src="assets/mobile-menu.js"/>
        </nav>
    }
}