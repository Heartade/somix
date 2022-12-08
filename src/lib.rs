mod client;

use crate::client::matrix_social_client;
use gloo_console::log;
use serde_json::Value;
use yew::prelude::*;
use yew_hooks::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    log!("Rendering App");
    let state = use_async(async move { matrix_social_client().await });
    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            log!("Starting sync...");
            state.run();
        })
    };
    html! {
        <div>
            <Nav></Nav>
            <br/>
            <div class="columns has-text-centered is-centered">
                <div class="column is-two-fifths">
                <br/>
                    <div>
            <button {onclick} class="button is-primary has-text-dark">{"Load Messages"}</button>
            <div /><br />
            <p class={classes!("has-text-white")}>
            {
                if let Some(data) = &state.data {
                    let data: Vec<Value> = serde_json::from_str(data).unwrap();
                    data.into_iter().map(|event| {
                            html!{
                                <article class="message">
                                <div class="message-header is-dark has-text-primary">
                                <p>{
                                    html!{
                                        &event["sender"].to_string()
                                    }
                                }</p>
                                </div>
                                <div class="message-body has-text-primary has-background-dark">
                                {
                                    html!{
                                        &event["content"]["body"].to_string()
                                    }
                                }
                                </div>
                                </article>
                            }
                        }).collect::<Html>()
                }
                else {
                    html! { }
                }
            }
            {
                if state.loading {
                    html! {
                        <div>
                            <p>{"Loading..."}</p>
                            <progress class="progress is-small is-primary" max="100">{"30%"}</progress>
                        </div>
                     }
                } else {
                    html! { }
                }
            }
            </p>
        </div>
                </div>
                <div class="column is-one-fifth">
                    <p class="box has-background-dark has-text-primary">{"Lorem ipsum"}</p>
                </div>
            </div>
        </div>
    }
}

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <div>
            <nav class="navbar is-fixed-top is-dark">
                <div class="navbar-brand">
                    <div class="navbar-item">
                        <a class="title has-text-primary">{"matrix-social"}</a>
                    </div>
                </div>
                <div class="navbar-menu is-active">
                    <div class="navbar-start">
                        <a class="navbar-item has-text-primary">{"Feed"}</a>
                    </div>
                    <div class="navbar-end">
                        <div class="navbar-item has-text-primary">
                            { "Username Placeholder" }
                        </div>
                        <div class="navbar-item">
                            <a class="button is-primary has-text-dark">{"Login"}</a>
                        </div>
                    </div>
                </div>
            </nav>
        </div>
    }
}
