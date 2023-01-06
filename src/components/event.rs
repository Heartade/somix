use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::client::Post;
use crate::components::post::PostComp;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub event_id: String,
}

#[function_component(Event)]
pub fn event(props: &Props) -> Html {
    let event_id = props.event_id.clone();
    let mut comments: Vec<Post> = vec![];

    match LocalStorage::get("matrix-social:posts") {
        Ok(posts) => {
            let posts: Vec<Post> = posts;
            for post in posts {
                match post.clone().reply_to {
                    Some(reply_to) => {
                        if reply_to == event_id {
                            comments.push(post)
                        }
                    }
                    None => {}
                }
            }
        }
        Err(e) => {}
    }

    let comments: Html = comments
        .into_iter()
        .map(|comment: Post| {
            html! {
                <PostComp event_id={comment.event_id} show_return_button={false} />
            }
        })
        .collect::<Html>();

    html! {
       <div class="flex sm:flex-nowrap justify-center">
            <div class="w-max md:w-2/3 lg:w-1/2 xl:w-2/5 2xl:w-1/3 mx-4">
                <PostComp event_id={event_id} show_return_button={true} />
                <div class="flex justify-center w-full h-46 gap-4 py-4 px-8 mb-4 border border-tuatara-400 rounded bg-tuatara-700">
                    <span class="sticky text-center text-charm-400">{"Placeholder"}</span>
                </div>
                { comments }
            </div>
       </div>
    }
}
