use std::ops::Deref;

use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::{
    client::{get_room_info, Post},
    components::post::PostComp,
    Route,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub room_id: String,
}

#[function_component(Room)]
pub fn room(props: &Props) -> Html {
    let room_id = props.room_id.clone();
    let loading_state = use_state(|| true);
    let room_name_state = use_state(|| "".to_string());
    let room_desc_state = use_state(|| "".to_string());
    let room_avatar_url_state = use_state(|| "".to_string());
    {
        let room_id = room_id.clone();
        let loading_state = loading_state.clone();
        let room_name_state = room_name_state.clone();
        let room_desc_state = room_desc_state.clone();
        let room_avatar_url_state = room_avatar_url_state.clone();
        if loading_state.deref().clone() {
            wasm_bindgen_futures::spawn_local(async move {
                let loading_state = loading_state.clone();
                let room_desc_state = room_desc_state.clone();
                let room_name_state = room_name_state.clone();
                let room_avatar_url_state = room_avatar_url_state.clone();
                let (room_name, room_desc, room_avatar_url) = get_room_info(room_id).await;
                room_name_state.set(room_name);
                room_desc_state.set(room_desc);
                room_avatar_url_state.set(room_avatar_url);
                loading_state.set(false);
            });
        }
    }
    let posts: Html = match LocalStorage::get("matrix-social:posts") {
        Ok(posts) => {
            let posts: Vec<Post> = posts;
            html! {
                <>
                    {
                        posts.into_iter().map(|post: Post| {
                            if post.room_id == props.room_id.clone() {
                                match post.reply_to {
                                    Some(_) => {
                                        html! {}
                                    },
                                    None => html! {
                                        <PostComp event_id={post.event_id} show_return_button={false} />
                                    }
                                }
                            } else {
                                html! {}
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
        <div>
            {
                if *loading_state.clone() {
                    html! { <div class="flex justify-center"><span class="text-center text-charm-400">{"Loading..."}</span></div> }
                } else {
                    html! {
                        <>
                            <div class="w-full h-48 bg-charm-500 flex flex-col gap-4 text-center align-center justify-end">
                                <div class="flex justify-center gap-4">
                                    <img src={ room_avatar_url_state.deref().clone() }
                                         class="rounded-full w-16 h-16 border border-tuatara-500 ring ring-tuatara-500" />
                                    <span class="text-tuatara-500 text-5xl">{room_name_state.deref().clone()}</span>
                                </div>
                                <Link<Route> to={Route::Room {room_id: props.room_id.clone()}}
                                    classes={classes!(String::from("text-tuatara-400 hover:text-tuatara-500"))}>
                                    {"Room Id: "}{props.room_id.clone()}
                                </Link<Route>>
                                <br />
                            </div>
                            <br />
                            <div class="flex flex-wrap-reverse sm:flex-nowrap justify-center">
                                <div class="w-max md:w-2/3 lg:w-1/2 xl:w-2/5 2xl:w-1/3 mx-4">
                                    { posts }
                                </div>
                                <div>
                                    <div class="flex flex-col justify-start align-center max-h-min w-80 min-h-96 gap-4 py-4 px-8 mb-4 border border-tuatara-400 rounded bg-tuatara-700">
                                        <span class="sticky text-center text-charm-400 text-2xl max-h-96">{room_name_state.deref().clone()}</span>
                                        <span class="sticky text-center text-charm-400 max-h-96">{room_desc_state.deref().clone()}</span>
                                    </div>
                                </div>
                            </div>
                        </>
                    }
                }
            }
        </div>
    }
}
