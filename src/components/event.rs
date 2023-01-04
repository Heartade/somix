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
    let posts: Vec<Post> = LocalStorage::get("matrix-social:posts").unwrap_or_default();
    let mut post: Option<Post> = None;

    for post_ in posts {
        if post_.event_id == event_id {
            post = Some(post_)
        }
    }

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
                <PostComp post={comment} show_return_button={false} />
            }
        })
        .collect::<Html>();

    let content = match post {
        Some(post) => html! {
            <PostComp post={post} show_return_button={true} />
        },
        None => html! { format!{"Post \"{event_id}\" not found."} },
    };
    html! {
       <>
       <div class="columns has-text-centered is-centered">
           <div class="column is-two-fifths">
               { content }
               <article class="media">
               <div class="media-content">
                 <div class="field">
                   <p class="control">
                     <textarea class="textarea has-background-dark has-text-primary" placeholder="Say something...">{"aaa"}</textarea>
                   </p>
                 </div>
                 <nav class="level">
                   <div class="level-left">
                     <div class="level-item">
                       <a class="button is-primary">{"Send"}</a>
                     </div>
                   </div>
                 </nav>
               </div>
             </article>
             { comments }
           </div>
           <div class="column is-one-fifth">
               <p class="box has-background-dark has-text-primary">{"Placeholder"}</p>
           </div>
       </div>
       </>
    }
}
