use crate::client::matrix_social_client;
use gloo_console::log;
use serde_json::Value;
use yew::prelude::*;
use yew_hooks::prelude::*;

#[function_component(Feed)]
pub fn feed() -> Html {
    let state = use_async(async move { matrix_social_client().await });
    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            log!("Starting sync...");
            state.run();
        })
    };
    html! {
        <div class="columns has-text-centered is-centered">
                <div class="column is-two-fifths">
                <br/>
                    <div>
        <div>
        <button {onclick} class="button is-primary has-text-dark">{"Load Messages"}</button>
            <div /><br />
    <p class={classes!("has-text-white")}>
            {
                if let Some(data) = &state.data {
                    //let data: Vec<Value> = serde_json::from_str(data).unwrap();
                    data.into_iter().map(|event| {
                            //log!(format!("{}", &event["sender"].to_string()));
                            html!{
                                <article class="message">
                                <div class="message-header is-dark has-text-primary">
                                <p>{
                                    html!{
                                    //    &event["sender"].to_string()
                                    }
                                }</p>
                                </div>
                                <div class="message-body has-text-primary has-background-dark">
                                {
                                    html!{
                                    //    &event["content"]["body"].to_string()
                                        &event.body
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
                </div>
                <div class="column is-one-fifth">
                    <p class="box has-background-dark has-text-primary">{"Lorem ipsum"}</p>
                </div>
            </div>
        }
}
