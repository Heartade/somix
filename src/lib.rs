mod components;
mod store;

use components::atoms::guest::Guest;
use components::molecules::nav::Nav;

use yew::prelude::*;

use gloo_console::log;

#[function_component(App)]
pub fn app() -> Html {
    log!("Rendering App");
    html! {
        <div>
            <Nav></Nav>
            <br/>
            <div class="columns has-text-centered is-centered">
                <div class="column is-two-fifths">
                <Guest></Guest>
                <br/>
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
