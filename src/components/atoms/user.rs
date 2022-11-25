use yew::prelude::*;
use yewdux::prelude::*;
use serde_json::json;
use crate::store::user::UserStore;

#[function_component(UserName)]
pub fn user_name() -> Html {
    let (user, dispatch) = use_store::<UserStore>();
    let user_label = json!(*user)["user_id"].as_str().unwrap().to_string();
    html! {
        <div>
            {user_label}
        </div>
    }
}
