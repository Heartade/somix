use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onchange: Callback<String>,
    pub class: Option<String>,
    pub type_: Option<String>,
    pub placeholder: Option<String>,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let text_input_onchange = {
        let emit_onchange = props.onchange.clone();
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            emit_onchange.emit(value.clone());
        })
    };
    html! {
        <input
            onchange={text_input_onchange}
            class={props.class.clone().unwrap_or_default()}
            type={props.type_.clone().unwrap_or_default()}
            placeholder={props.placeholder.clone().unwrap_or_default()}
            />
    }
}
