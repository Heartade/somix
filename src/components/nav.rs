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
        LocalStorage::delete("somix:session");
        LocalStorage::delete("somix:posts");
        log!("Logged out");
        navigator.push(&Route::Login);
    });

    let logged_in = match LocalStorage::get("somix:session") {
        Ok(session) => {
            let session: Session = session;
            Some(session.user_id.to_string())
        }
        Err(_) => None,
    };

    html! {
        <nav class="top-0 sticky bg-tuatara-600 z-50">
            <div class="px-8 mx-auto">
                <div class="flex justify-between items-center">
                    <div class="flex space-x-4">
                        <div>
                            <Link<Route> to={Route::Home} classes={classes!(String::from("flex items-center py-4 px-2 text-charm-400 hover:text-charm-300"))}>
                                <svg id="SVGRoot" aria-label="Somix wordmark" class="h-12 fill-charm-400" version="1.1" viewBox="0 0 64 24" xmlns="http://www.w3.org/2000/svg">
                                    <g>
                                        <path d="m7 4h-3v16h3v-1h-2v-14h2z"/>
                                        <path d="m57 4v1h2v14h-2v1h3v-16z"/>
                                        <path d="m15.141 10.99q-0.078125-0.44531-0.44531-0.71094-0.36719-0.26562-0.94531-0.27344-0.5625 0.0078-0.9375 0.25-0.375 0.23438-0.375 0.61719 0 0.30469 0.25 0.52344 0.25781 0.21094 0.84375 0.32812l1.5156 0.3125q1.2422 0.25781 1.8438 0.82031 0.60156 0.5625 0.60938 1.4766 0 0.82812-0.47656 1.4609-0.46875 0.63281-1.3203 0.98438-0.85156 0.35156-1.9688 0.35156-1.1328 0-1.9609-0.32031-0.82031-0.32812-1.2969-0.92969-0.46875-0.60938-0.57031-1.4375h2.3281q0.07813 0.49219 0.46094 0.75 0.39062 0.25781 1.0391 0.26562 0.64062-0.0078 1.0234-0.24219 0.38281-0.24219 0.38281-0.64844 0-0.32812-0.28125-0.53906-0.28125-0.21094-0.875-0.33594l-1.4062-0.28125q-1.2109-0.24219-1.8281-0.85156-0.61719-0.60938-0.60938-1.5547 0-0.80469 0.4375-1.3984 0.4375-0.60156 1.2422-0.92188 0.8125-0.32031 1.8984-0.32031 1.0547 0 1.8438 0.32031 0.78906 0.32031 1.2344 0.91406 0.45312 0.58594 0.51562 1.3906z"/>
                                        <path d="m22.734 17.131q-1.2656 0-2.2109-0.54688-0.9375-0.55469-1.4453-1.5469-0.5-0.99219-0.5-2.2969 0-1.3047 0.5-2.2891 0.50781-0.99219 1.4453-1.5391 0.94531-0.54688 2.2109-0.54688t2.2031 0.54688q0.94531 0.54688 1.4531 1.5391 0.51562 0.98438 0.51562 2.2891 0 1.3047-0.51562 2.2969-0.50781 0.99219-1.4531 1.5469-0.9375 0.54688-2.2031 0.54688zm-1.8125-4.3906q0 0.75781 0.20312 1.3438 0.21094 0.57812 0.625 0.91406 0.41406 0.33594 1 0.33594 0.57031 0 0.97656-0.32812 0.40625-0.33594 0.60938-0.91406 0.21094-0.57812 0.21094-1.3203 0-0.76562-0.20312-1.3594-0.20312-0.60156-0.60938-0.9375-0.40625-0.34375-0.98438-0.34375-0.58594 0-1 0.33594-0.41406 0.33594-0.625 0.92969-0.20312 0.58594-0.20312 1.3438z"/>
                                        <path d="m28.359 8.4746h2.2031v1.4844h0.09375q0.27344-0.75 0.89062-1.1719 0.61719-0.42188 1.4688-0.42188 0.86719 0 1.4844 0.42969 0.61719 0.42969 0.82812 1.1641h0.07813q0.26562-0.72656 0.95312-1.1562 0.6875-0.4375 1.6094-0.4375 0.78906 0 1.3906 0.34375 0.60938 0.33594 0.94531 0.99219 0.33594 0.64844 0.33594 1.5547v5.7031h-2.3125v-5.25q0.0078-0.70312-0.36719-1.0703-0.36719-0.36719-0.96094-0.36719-0.64062 0-1.0156 0.41406-0.36719 0.40625-0.35938 1.0859v5.1875h-2.25v-5.3125q0.0078-0.625-0.35938-1-0.35938-0.375-0.95312-0.375-0.39062 0-0.71094 0.20312-0.32031 0.19531-0.5 0.5625-0.17969 0.35938-0.17969 0.82812v5.0938h-2.3125z"/>
                                        <path d="m42.422 8.4746h2.3125v8.4844h-2.3125zm1.1562-1.1094q-0.34375 0-0.63281-0.15625-0.28906-0.16406-0.46094-0.42969-0.17188-0.27344-0.17188-0.58594 0-0.32031 0.17188-0.59375t0.46094-0.42969q0.28906-0.16406 0.63281-0.16406 0.33594 0 0.625 0.16406 0.28906 0.15625 0.45312 0.42969 0.17188 0.27344 0.17188 0.59375 0 0.3125-0.17188 0.58594-0.16406 0.26562-0.45312 0.42969-0.28906 0.15625-0.625 0.15625z"/>
                                        <path d="m50.031 11.459 1.6094-2.9844h2.3906l-2.4844 4.2344 2.5469 4.25h-2.375l-1.6875-2.9531-1.6562 2.9531h-2.4062l2.5312-4.25-2.4375-4.2344h2.3906z"/>
                                    </g>
                                </svg>
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
                                    <div>
                                        <Link<Route> to={Route::New}>
                                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"
                                                 class="w-6 h-6 stroke-charm-400 hover:stroke-charm-300">
                                              <path stroke-linecap="round" stroke-linejoin="round" d="M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L10.582 16.07a4.5 4.5 0 01-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 011.13-1.897l8.932-8.931zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0115.75 21H5.25A2.25 2.25 0 013 18.75V8.25A2.25 2.25 0 015.25 6H10" />
                                            </svg>
                                        </Link<Route>>
                                    </div>
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
                                <Link<Route> to={Route::New} classes={classes!(String::from("py-4 px-4 block font-bold hover:bg-tuatara-700"))}>
                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"
                                         class="w-6 h-6 stroke-charm-400 hover:stroke-charm-300">
                                      <path stroke-linecap="round" stroke-linejoin="round" d="M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L10.582 16.07a4.5 4.5 0 01-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 011.13-1.897l8.932-8.931zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0115.75 21H5.25A2.25 2.25 0 013 18.75V8.25A2.25 2.25 0 015.25 6H10" />
                                    </svg>
                                </Link<Route>>
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
