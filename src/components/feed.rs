use crate::components::post::PostComp;
use crate::{
    client::{get_posts, Post},
    Route,
};

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
                if e.to_string() == "key somix:session not found".to_owned() {
                    navigator.push(&Route::Login);
                }
            }
        }
    });
    let posts: Html = match LocalStorage::get("somix:posts") {
        Ok(posts) => {
            let posts: Vec<Post> = posts;
            html! {
                <>
                    {
                        posts.into_iter().map(|post: Post| {
                            html! {
                                match post.reply_to {
                                    Some(_) => {
                                        html! {}
                                    },
                                    None => html! {
                                        <PostComp event_id={post.event_id} show_return_button={false} />
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
        <>
        <br />
        <div class="flex flex-wrap-reverse sm:flex-nowrap justify-center">
            <div class="w-max md:w-2/3 lg:w-1/2 xl:w-2/5 2xl:w-1/3 mx-4">
                { posts }
            </div>
            <div class="flex justify-center w-80 h-96 gap-4 py-4 px-8 mb-4 border border-tuatara-400 rounded bg-tuatara-700">
                <span class="sticky text-center text-charm-400">{"Placeholder"}</span>
            </div>
        </div>
        </>
    }
}
