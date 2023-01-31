use gloo_console::log;
use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;

use crate::client::Post;
use crate::components::compose::Compose;
use crate::components::post::PostComp;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub event_id: String,
}

#[function_component(Event)]
pub fn event(props: &Props) -> Html {
    let event_id = props.event_id.clone();
    let mut comments: Vec<Post> = vec![];
    let mut post: Vec<Post> = vec![];

    match LocalStorage::get("matrix-social:posts") {
        Ok(posts) => {
            let posts: Vec<Post> = posts;
            for post_ in posts {
                if post_.clone().event_id == event_id {
                    post.push(post_.clone());
                    break;
                }
            }
        }
        Err(e) => {
            log!(e.to_string());
        }
    };

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
        Err(e) => {
            log!(e.to_string());
        }
    }

    let comments: Html = comments
        .into_iter()
        .map(|comment: Post| {
            html! {
                <PostComp event_id={comment.event_id} show_return_button={false} />
            }
        })
        .collect::<Html>();

    let compose: Html = post
        .into_iter()
        .map(|post: Post| {
            html! {
                <Compose room_id={post.clone().room_id} reply_to={post.clone()}/>
            }
        })
        .collect::<Html>();

    html! {
       <div class="flex sm:flex-nowrap justify-center">
            <div class="w-max md:w-2/3 lg:w-1/2 xl:w-2/5 2xl:w-1/3 mx-4">
                <PostComp event_id={event_id} show_return_button={true} />
                <div class="flex justify-center w-full h-46 gap-4 py-1 px-8 mb-4">
                    { compose }
                </div>
                { comments }
            </div>
       </div>
    }
}
