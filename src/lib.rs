mod client;
mod components;

use std::collections::VecDeque;

use crate::components::{event::Event, feed::Feed, login::Login, nav::Nav};
use gloo_console::log;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/feed")]
    Feed,
    #[at("/login")]
    Login,
    #[at("/$/:event_id")]
    Event { event_id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <p class="text-4xl text-center text-bold text-charm-400">{"Welcome to matrix-social!"}</p> }
        }
        Route::Feed => html! { <Feed /> },
        Route::Login => html! { <Login /> },
        Route::Event { event_id } => html! { <Event event_id={ event_id } /> },
        Route::NotFound => {
            html! { <p class="text-4xl text-center text-bold text-charm-400">{"404 Not Found"}</p> }
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    log!("Rendering App");
    html! {
        <BrowserRouter>
            <Nav></Nav>
            <br/>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn round_robin_vec_merge<T: Clone>(mut vecs: Vec<Vec<T>>) -> Vec<T> {
    let mut vecs: VecDeque<_> = vecs.iter_mut().map(|v| v.drain(..)).collect();

    let mut result = Vec::new();

    while let Some(front) = vecs.front_mut() {
        if let Some(elem) = front.next() {
            result.push(elem);
            vecs.rotate_left(1);
        } else {
            vecs.pop_front();
        }
    }

    result
}
