use std::ops::Deref;

use gloo_console::log;
use gloo_storage::{LocalStorage, Storage};
use ruma::{EventId, events::{MessageLikeEvent, reaction::ReactionEventContent, relation::RelationType}, OwnedEventId, RoomId};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    client::{get_client, Post, react_to_event, redact_event},
    Route, SomixError,
};
use crate::client::{get_post_info, PostInfo};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub event_id: String,
    pub show_return_button: bool,
}

#[function_component(PostComp)]
pub fn post(props: &Props) -> Html {
    let posts: Vec<Post> = LocalStorage::get("somix:posts").unwrap_or_default();
    let mut post: Option<Post> = None;

    for post_ in posts {
        if post_.event_id == props.event_id {
            post = Some(post_)
        }
    }
    let post = post.unwrap();

    let post_info: UseStateHandle<Option<PostInfo>> = use_state(|| None);
    let post_info_callback = {
        let post_info = post_info.clone();
        let event_id = post.clone().event_id;
        let room_id = post.clone().room_id;
        Callback::from(move |post: Post| {
            let post_info = post_info.clone();
            let event_id = event_id.clone();
            let room_id = room_id.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let post_info = post_info.clone();
                let event_id = EventId::parse(event_id.clone()).unwrap();
                let room_id = RoomId::parse(room_id.clone()).unwrap();
                post_info.set(Some(get_post_info(event_id, room_id).await.unwrap()));
            });
        })
    };

        {
            let post_info_callback = post_info_callback.clone();
            let post = post.clone();
            use_effect_with_deps(
                move |_| {
                    post_info_callback.emit(post.clone());
                }, ()
            );
        }

    let room_id_state = use_state(|| post.room_id.clone());
    let event_id_state = use_state(|| post.event_id.clone());

    let score_state = use_state(|| "loading".to_string());
    let score_callback = {
        let score_state = score_state.clone();
        let event_id_state = event_id_state.clone();
        let room_id_state = room_id_state.clone();
        Callback::from(move |_: String| {
            let score_state = score_state.clone();
            let event_id_state = event_id_state.clone();
            let room_id_state = room_id_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let score_state = score_state.clone();
                let event_id = EventId::parse(event_id_state.deref().clone()).unwrap();
                let room_id = RoomId::parse(room_id_state.deref().clone()).unwrap();
                let score = get_post_info(event_id, room_id).await.unwrap();
                score_state.set(score);
            });
        })
    };

    {
        let score_callback = score_callback.clone();
        use_effect_with_deps(
            move |_| {
                score_callback.emit("".to_string());
            },
            (),
        );
    }

    let react_callback = {
        let room_id_state = room_id_state.clone();
        let event_id_state = event_id_state.clone();
        let post = post.clone();
        let score_state = score_state.clone();
        let post_info = post_info.clone();
        Callback::from(move |reaction: String| {
            let post = post.clone();
            let room_id_state = room_id_state.clone();
            let event_id_state = event_id_state.clone();
            let score_state = score_state.clone();
            let post_info = post_info.clone();
            //score_state.set("loading".to_string());
            post_info.set(None);
            wasm_bindgen_futures::spawn_local(async move {
                match react_to_event(
                    room_id_state.deref().to_string(),
                    event_id_state.deref().to_string(),
                    reaction,
                )
                    .await
                {
                    Ok(_) => {}
                    Err(e) => match e {
                        SomixError::MatrixSDK(e) => match e {
                            matrix_sdk::Error::Http(e) => {
                                if e.to_string().contains("Can't send same reaction twice") {
                                    log!("redact reaction!!!");
                                    let client = get_client().await.unwrap();
                                    let room_id = RoomId::parse(post.room_id.clone()).unwrap();
                                    let event_id = EventId::parse(post.event_id.clone()).unwrap();
                                    let request = ruma::api::client::relations::get_relating_events_with_rel_type::v1::Request::new(
                                        &room_id, &event_id, RelationType::Annotation);
                                    let resp = client.send(request, None).await.unwrap();
                                    for event in resp.chunk.iter() {
                                        let event: MessageLikeEvent<ReactionEventContent> =
                                            serde_json::from_str(event.json().to_string().as_str())
                                                .unwrap();
                                        match event {
                                            MessageLikeEvent::Original(event) => {
                                                if event.sender == client.session().unwrap().user_id
                                                {
                                                    if event.content.relates_to.key
                                                        == "👍️".to_string()
                                                        || event.content.relates_to.key
                                                        == "👎️".to_string()
                                                    {
                                                        redact_event(
                                                            event.room_id.to_string(),
                                                            event.event_id.to_string(),
                                                        )
                                                            .await;
                                                    }
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                } else {
                                    log!(e.to_string())
                                }
                            }
                            _ => log!(e.to_string()),
                        },
                        _ => log!(format!("{e:?}")),
                    },
                }
                let event_id = EventId::parse(event_id_state.deref().clone()).unwrap();
                let room_id = RoomId::parse(room_id_state.deref().clone()).unwrap();
                //score_state.set(get_post_info(event_id, room_id).await.unwrap().score.to_string());
                post_info.set(Some(get_post_info(event_id, room_id).await.unwrap()));
            });
        })
    };

    let upvote_onclick = {
        let react_callback = react_callback.clone();
        Callback::from(move |_| {
            react_callback.emit("👍️".to_string());
        })
    };

    let downvote_onclick = {
        let react_callback = react_callback.clone();
        Callback::from(move |_| {
            react_callback.emit("👎️".to_string());
        })
    };

    html! {
        <div class="flex w-full mb-4 border border-tuatara-400 rounded bg-tuatara-600">
            <div class="flex flex-col bg-tuatara-700 p-2 rounded"> //left
                <button class="group hover:bg-tuatara-500 rounded p-1" onclick={upvote_onclick}>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"
                         class="w-6 h-6 stroke-tuatara-400 group-hover:stroke-charm-300">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 19.5v-15m0 0l-6.75 6.75M12 4.5l6.75 6.75" />
                    </svg>
                </button> //thumbs up
                {
                    match post_info.deref().clone() {
                        None => html! {
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="animate-spin w-8 h-8 stroke-tuatara-400 justify-self-center">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99" />
                            </svg>
                        },
                        Some(post_info) => html! {
                            <span class="text-center text-tuatara-400">{post_info.score.to_string()}</span>
                        }
                    }
                }
                <button class="group hover:bg-tuatara-500 rounded p-1" onclick={downvote_onclick}>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"
                         class="w-6 h-6 stroke-tuatara-400 group-hover:stroke-stiletto-500">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m0 0l6.75-6.75M12 19.5l-6.75-6.75" />
                    </svg>
                </button> //thumbs down
            </div>
            <div class="flex flex-col w-full bg-tuatara-600 text-charm-400 p-4 rounded"> //right
                <div class="flex gap-2 text-charm-400 justify-between"> //top
                    <Link<Route> to={Route::Room { room_id: post.room_id.clone() }} classes={classes!{String::from("flex gap-1 align group")}}>
                        <img src="assets/logo_128x128.webp" class="h-6 w-6 rounded-full" /> //room image
                        <span class="group-hover:text-charm-300 group-hover:underline">{ post.room_name.clone() }</span> //room name
                    </Link<Route>>
                    {
                        match post_info.deref().clone() {
                            None => html! {
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="animate-spin w-8 h-8 stroke-charm-300 justify-self-center">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99" />
                                </svg>
                            },
                            Some(post_info) => html! {
                                <span>{"Sent by"}{post_info.sender.to_string()}</span>
                            }
                        }
                    }
                </div>
                <Link<Route> to={Route::Event { event_id: post.event_id.clone() }} classes={classes!(String::from("py-4 hover:text-charm-300"))}>
                    {
                        match post_info.deref().clone() {
                            None => html! {
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="animate-spin w-8 h-8 stroke-charm-300 justify-self-center">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99" />
                                </svg>
                            },
                            Some(post_info) => html! {
                                <span class="text-3xl font-bold">{post_info.body}</span>
                            }
                        }
                    }
                </Link<Route>>
                <div class="flex gap-2"> //bottom
                    <Link<Route> to={Route::Event { event_id: post.event_id.clone() }} classes={classes!(String::from(
                        "flex gap-1 hover:bg-tuatara-500 p-2 rounded group"))}
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"
                            class="w-6 h-6 group-hover:stroke-charm-300">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M7.5 8.25h9m-9 3H12m-9.75 1.51c0 1.6 1.123 2.994 2.707 3.227 1.129.166 2.27.293 3.423.379.35.026.67.21.865.501L12 21l2.755-4.133a1.14 1.14 0 01.865-.501 48.172 48.172 0 003.423-.379c1.584-.233 2.707-1.626 2.707-3.228V6.741c0-1.602-1.123-2.995-2.707-3.228A48.394 48.394 0 0012 3c-2.392 0-4.744.175-7.043.513C3.373 3.746 2.25 5.14 2.25 6.741v6.018z" />
                        </svg>
                        <span class="group-hover:text-charm-300">{post.reply_ids.clone().len()}{" Comment(s)"}</span>
                    </Link<Route>>
                    /*
                    <button class="flex gap-1 hover:bg-tuatara-500 p-2 rounded group">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"
                             class="w-6 h-6 group-hover:stroke-charm-300">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15.182 15.182a4.5 4.5 0 01-6.364 0M21 12a9 9 0 11-18 0 9 9 0 0118 0zM9.75 9.75c0 .414-.168.75-.375.75S9 10.164 9 9.75 9.168 9 9.375 9s.375.336.375.75zm-.375 0h.008v.015h-.008V9.75zm5.625 0c0 .414-.168.75-.375.75s-.375-.336-.375-.75.168-.75.375-.75.375.336.375.75zm-.375 0h.008v.015h-.008V9.75z" />
                        </svg>
                        <span class="group-hover:text-charm-300">{"React"}</span>
                    </button>
                    */
                    {
                        if props.show_return_button {
                            match post.reply_to.clone() {
                                Some(reply_to) => html! {
                                    <Link<Route> to={Route::Event { event_id: reply_to }} classes={classes!(String::from(
                                            "flex gap-1 hover:bg-tuatara-500 p-2 rounded group"))}
                                    >
                                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"
                                             class="w-6 h-6 group-hover:stroke-charm-300">
                                          <path stroke-linecap="round" stroke-linejoin="round" d="M9 15L3 9m0 0l6-6M3 9h12a6 6 0 010 12h-3" />
                                        </svg>
                                        <span class="group-hover:text-charm-300">{"Return"}</span>
                                    </Link<Route>>
                                },
                                None => html! {}
                            }
                        } else { html! {} }
                    }
                </div>
            </div>
        </div>
    }
}
