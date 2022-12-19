use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;

use crate::client::Post;

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
                    <div>
                        <br />
                        <article class="message is-small">
                            <div class="message-header is-dark has-text-primary">
                                <p></p><p>{"Sent by "} <span class="is-italic">{ comment.sender_id }</span></p>
                            </div>
                            <div class="message-body has-text-primary has-background-dark is-size-5 is-centered">
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
                                    <span>{"Comment... "}</span>
                                 </button>
                                { comment.content }
                            </div>
                        </article>
                    </div>
            }
        })
        .collect::<Html>();

    let content = match post {
        Some(post) => html! {
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
                        { post.content }
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
