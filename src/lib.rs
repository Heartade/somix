use std::collections::VecDeque;

use gloo_console::log;
use gloo_storage::errors::StorageError;
use yew::prelude::*;
use yew_router::prelude::*;

use components::{alias::Alias, event::Event, feed::Feed, login::Login, nav::Nav, new::New, room::Room};

mod client;
mod components;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/feed")]
    Feed,
    #[at("/new")]
    New,
    #[at("/login")]
    Login,
    #[at("/$/:event_id")]
    Event { event_id: String },
    #[at("/!/:room_id")]
    Room { room_id: String },
    #[at("/alias/:room_alias")]
    Alias { room_alias: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <><br /><p class="text-4xl text-center text-bold text-charm-400">{"Welcome to Somix!"}</p></> }
        }
        Route::Feed => html! { <Feed /> },
        Route::New => html! { <New />},
        Route::Login => html! { <Login /> },
        Route::Event { event_id } => html! { <Event event_id={ event_id } /> },
        Route::Room { room_id } => html! { <Room room_id={ room_id } /> },
        Route::Alias { room_alias } => html! { <Alias room_alias={ room_alias } /> },
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

#[derive(Debug)]
pub enum SomixError {
    Storage(StorageError),
    MatrixSDK(matrix_sdk::Error),
    IdParse(ruma::IdParseError),
    Reqwest(matrix_sdk::reqwest::Error),
}

impl From<StorageError> for SomixError {
    fn from(e: StorageError) -> Self {
        SomixError::Storage(e)
    }
}

impl From<matrix_sdk::Error> for SomixError {
    fn from(e: matrix_sdk::Error) -> Self {
        SomixError::MatrixSDK(e)
    }
}

impl From<ruma::IdParseError> for SomixError {
    fn from(e: ruma::IdParseError) -> Self { SomixError::IdParse(e) }
}

impl From<matrix_sdk::reqwest::Error> for SomixError {
    fn from(e: matrix_sdk::reqwest::Error) -> Self { SomixError::Reqwest(e) }
}

pub fn error_alert(e: SomixError) {
    let mut message = format!("{e:?}");
    match e {
        SomixError::Storage(_) => {}
        SomixError::MatrixSDK(_) => {}
        SomixError::IdParse(_) => {
            message = format!("The user id should be in the form of @username:example.org \n{message}");
        }
        SomixError::Reqwest(_) => {
            message = format!("Unable to connect to homeserver \n{message}");
        }
    }
    message = format!("An error has occurred:\n{message}");
    log!(&message);
    gloo_dialogs::alert(&message);
}
