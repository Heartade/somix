use crate::components::post::PostComp;
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
                                        <PostComp post={post} />
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
                { posts }
            </div>
            <div class="column is-one-fifth">
                <p class="box has-background-dark has-text-primary">{"Placeholder"}</p>
            </div>
        </div>
    }
}
