use crate::client::{get_posts, Post};
use gloo_console::log;
use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;

#[function_component(Feed)]
pub fn feed() -> Html {
    wasm_bindgen_futures::spawn_local(async move {
        get_posts().await.unwrap();
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
                { e }
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
