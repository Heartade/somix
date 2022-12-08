mod client;
mod components;

use crate::components::{feed::Feed, login::Login, nav::Nav};
use gloo_console::log;
use yew::prelude::*;
use yew_router::prelude::*;

const BASE_URL: &str = "http://0.0.0.0:8080";

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/feed")]
    Feed,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <p class="title has-text-primary has-text-centered">{"Welcome to matrix-social!"}</p> }
        }
        Route::Feed => html! { <Feed /> },
        Route::Login => html! { <Login /> },
        Route::NotFound => {
            html! { <p class="title has-text-primary has-text-centered">{"404 Not Found"}</p> }
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    log!("Rendering App");
    html! {
        <div>
            <Nav></Nav>
            <br/>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </div>
    }
}
