use crate::{
    client::{get_posts, Post},
    Route,
};
use gloo_console::log;
use gloo_storage::errors::StorageError::*;
use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_router::prelude::*;

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
                                match post.reply_to {
                                    Some(reply_to) => {
                                        html! {}
                                    },
                                    None => html! {
                                        <div>
                                            <article class="message">
                                                <div class="message-header is-dark has-text-primary">
                                                    {
                                                        html! {
                                                            <>
                                                            <p><span class="is-underlined">{ post.room_name }</span><span class="has-text-primary-dark">{" ("} { post.room_id } {")"}</span></p>
                                                            <p>{"Sent by "} <span class="is-italic">{ post.sender_id }</span></p>
                                                            </>
                                                        }
                                                    }
                                                </div>
                                                <div class="message-body has-text-primary has-background-dark has-text-weight-bold is-size-4 is-centered">
                                                    <Link<Route> to={Route::Event { event_id: post.event_id }}>{ post.content }</Link<Route>>
                                                    <br /><br />
                                                    <button class="button is-small is-dark has-text-primary is-pulled-left">
                                                        <span class="icon is-small">
                                                            <ion-icon name="thumbs-up-sharp"></ion-icon>
                                                        </span>
                                                        <span>{"Like(s): "}{"0"}</span>
                                                    </button>
                                                    <button class="button is-small is-dark has-text-primary is-pulled-left">
                                                        <span class="icon is-small">
                                                            <ion-icon name="thumbs-down-sharp"></ion-icon>
                                                        </span>
                                                        <span>{"Dislike(s): "}{"0"}</span>
                                                    </button>
                                                    <button class="button is-small is-dark has-text-primary is-pulled-right">
                                                        <span class="icon is-small">
                                                            <ion-icon name="chatbox-ellipses-sharp"></ion-icon>
                                                        </span>
                                                        <span>{"Comment(s): "}{"0"}</span>
                                                     </button>
                                                </div>
                                            </article>
                                            <br />
                                        </div>
                                    }
                                }
                            }
                        }).collect::<Html>()
                    }
                </>
            }
        }
        Err(e) => html! {
            <p class="has-text-danger">
                {"Error: "}{e.to_string()}
                <br />
                {"You may need to logout/login or clear your Browser LocalStorage."}
            </p>
        },
    };
    html! {
        <div class="columns has-text-centered is-centered">
            <div class="column is-two-fifths">
                <br/>
                { posts }
            </div>
            <div class="column is-one-fifth">
                <p class="box has-background-dark has-text-primary">{"Placeholder"}</p>
            </div>
        </div>
    }
}
