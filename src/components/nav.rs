use yew::prelude::*;

use crate::BASE_URL;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <div>
            <nav class="navbar is-fixed-top is-dark">
                <div class="navbar-brand">
                    <div class="navbar-item">
                        <a class="title has-text-primary" href={BASE_URL.to_owned()+"/"}>{"matrix-social"}</a>
                    </div>
                </div>
                <div class="navbar-menu is-active">
                    <div class="navbar-start">
                        <a class="navbar-item has-text-primary" href={BASE_URL.to_owned()+"/feed"}>{"Feed"}</a>
                    </div>
                    <div class="navbar-end">
                        <div class="navbar-item has-text-primary">
                            { "Username Placeholder" }
                        </div>
                        <div class="navbar-item">
                            <a class="button is-primary has-text-dark">{"Login"}</a>
                        </div>
                    </div>
                </div>
            </nav>
        </div>
    }
}
