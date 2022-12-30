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
                                        <div class="flex w-full my-4 border border-tuatara-400 rounded bg-tuatara-600">
                                            <div class="flex flex-col bg-tuatara-700 p-2 rounded"> //left
                                                <button class="group hover:bg-tuatara-500 rounded p-1">
                                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"
                                                         class="w-6 h-6 stroke-tuatara-400 group-hover:stroke-charm-300">
                                                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 19.5v-15m0 0l-6.75 6.75M12 4.5l6.75 6.75" />
                                                    </svg>
                                                </button> //thumbs up
                                                <span class="text-center text-tuatara-400">{post.score.to_string()}</span>
                                                <button class="group hover:bg-tuatara-500 rounded p-1">
                                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"
                                                         class="w-6 h-6 stroke-tuatara-400 group-hover:stroke-stiletto-500">
                                                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m0 0l6.75-6.75M12 19.5l-6.75-6.75" />
                                                    </svg>
                                                </button> //thumbs down
                                            </div>
                                            <div class="flex flex-col w-full bg-tuatara-600 text-charm-400 p-4 rounded"> //right
                                                <div class="flex gap-2 text-charm-400 justify-between"> //top
                                                    <a class="flex gap-1 align group">
                                                        <img src="http://localhost:8080/assets/logo_128x128.webp" class="h-6 w-6 rounded-full" /> //room image
                                                        <span class="group-hover:text-charm-300 group-hover:underline">{ post.room_name.clone() }</span> //room name
                                                    </a>
                                                    <span>{"Sent by "}{ post.sender_id.clone() }</span>
                                                </div>
                                                <Link<Route> to={Route::Event { event_id: post.event_id.clone() }} classes={classes!(String::from("py-4 hover:text-charm-300"))}>
                                                    <span class="text-3xl font-bold">{ post.content.clone() }</span>
                                                </Link<Route>>
                                                <div class="flex gap-2"> //bottom
                                                    <button class="flex gap-1 hover:bg-tuatara-500 p-2 rounded">
                                                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                                            <path stroke-linecap="round" stroke-linejoin="round" d="M7.5 8.25h9m-9 3H12m-9.75 1.51c0 1.6 1.123 2.994 2.707 3.227 1.129.166 2.27.293 3.423.379.35.026.67.21.865.501L12 21l2.755-4.133a1.14 1.14 0 01.865-.501 48.172 48.172 0 003.423-.379c1.584-.233 2.707-1.626 2.707-3.228V6.741c0-1.602-1.123-2.995-2.707-3.228A48.394 48.394 0 0012 3c-2.392 0-4.744.175-7.043.513C3.373 3.746 2.25 5.14 2.25 6.741v6.018z" />
                                                        </svg>
                                                        <span class="">{"0 Comments"}</span>
                                                    </button>
                                                    <button class="flex gap-1 hover:bg-tuatara-500 p-2 rounded">
                                                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                                            <path stroke-linecap="round" stroke-linejoin="round" d="M15.182 15.182a4.5 4.5 0 01-6.364 0M21 12a9 9 0 11-18 0 9 9 0 0118 0zM9.75 9.75c0 .414-.168.75-.375.75S9 10.164 9 9.75 9.168 9 9.375 9s.375.336.375.75zm-.375 0h.008v.015h-.008V9.75zm5.625 0c0 .414-.168.75-.375.75s-.375-.336-.375-.75.168-.75.375-.75.375.336.375.75zm-.375 0h.008v.015h-.008V9.75z" />
                                                        </svg>
                                                        <span>{"React"}</span>
                                                    </button>
                                                </div>
                                            </div>
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
                { posts }
            </div>
            <div class="column is-one-fifth">
                <p class="box has-background-dark has-text-primary">{"Placeholder"}</p>
            </div>
        </div>
    }
}
