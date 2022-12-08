use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
    <div>
      <div class="columns has-text-centered is-centered">
        <div class="column is-two-fifths">
          <br />
          <div>
            <p class="title has-text-primary">{"Login"}</p>
            <div>
              <div class="field">
                <label class="label has-text-primary has-text-left">{"User Id:"}</label>
                <div class="control">
                  <input class="input is-primary" type="text" placeholder="@user:example.com" />
                </div>
              </div>

              <div class="field">
                <label class="label has-text-primary has-text-left">{"Password:"}</label>
                <div class="control">
                  <input class="input is-primary " type="password" />
                </div>
              </div>

              <div class="field is-grouped">
                <div class="control">
                  <button class="button is-primary">{"Login"}</button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
    }
}
