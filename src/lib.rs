mod components;
use components::atoms::nav::Nav;

use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <Nav></Nav>
            <br/>
            <div class="columns has-text-centered is-centered">
                <div class="column is-two-fifths">
                    <div class="box has-background-dark">
                        <p class="subtitle has-text-primary">
                            {"Lorem ipsum dolor sit amet"}
                        </p>
                    </div>
                    <div class="box has-background-dark">
                        <p class="subtitle has-text-primary">
                            {"consectetur adipiscing elit"}
                        </p>
                    </div>
                </div>
                <div class="column is-one-fifth">
                    <p class="box has-background-dark has-text-primary">{"Lorem ipsum"}</p>
                </div>
            </div>
        </div>
    }
}