mod client;
mod components;

use crate::{
    components::{feed::Feed, nav::Nav},
};
use gloo_console::log;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    log!("Rendering App");
    html! {
        <div>
            <Nav></Nav>
            <br/>
            <div class="columns has-text-centered is-centered">
                <div class="column is-two-fifths">
                <br/>
                    <div>
            <Feed />
        </div>
                </div>
                <div class="column is-one-fifth">
                    <p class="box has-background-dark has-text-primary">{"Lorem ipsum"}</p>
                </div>
            </div>
        </div>
    }
}
