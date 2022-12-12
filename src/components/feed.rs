use crate::{
    client::{get_posts, Post},
    Route,
};
use gloo_console::log;
use gloo_storage::errors::StorageError::*;
use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[function_component(Feed)]
pub fn feed() -> Html {
    let navigator = use_navigator().unwrap();
    wasm_bindgen_futures::spawn_local(async move {
        match get_posts().await {
            Ok(_) => {}
            Err(e) => {
                if e.to_string() == "key matrix-social:session not found".to_owned() {
                    navigator.push(&Route::Login);
                }
            }
        }
    });
    let posts: Html = match LocalStorage::get("matrix-social:posts") {
        Ok(posts) => {
            let posts: Vec<Post> = posts;
            html! {
                <>
                    {
                        posts.into_iter().map(|post: Post| {
                            html! {
                                <div>
                                    <article class="message">
                                        <div class="message-header is-dark has-text-primary">
                                            {
                                                html! {
                                                    <p>{"Sent by "} { post.sender } {" in "} { post.room }</p>
                                                }
                                            }
                                        </div>
                                        <div class="message-body has-text-primary has-background-dark">
                                            { post.content.body() }
                                        </div>
                                    </article>
                                    <br />
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </>
            }
        }
        Err(e) => html! {
            <>
                {
                    e.to_string()
                }
            </>
        },
    };
    html! {
        <div class="columns has-text-centered is-centered">
            <div class="column is-two-fifths">
                <br/>
                { posts }
            </div>
            <div class="column is-one-fifth">
                <p class="box has-background-dark has-text-primary">{"Lorem ipsum"}</p>
            </div>
        </div>
    }
}
