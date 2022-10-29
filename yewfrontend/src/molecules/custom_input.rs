use stylist::{style, yew::styled_component};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{html, Callback, Properties};

#[derive(Properties, Clone, PartialEq)]
pub struct CustomInput {
    pub name: String,
    pub placeholder: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct PasswordModel {
    pub name: String,
    pub placeholder: String,
    pub onchange: Callback<String>,
}
#[derive(Properties, Clone, PartialEq)]
pub struct LoginDetailsModel {
    pub name: String,
    pub placeholder: String,
    pub onchange: Callback<String>,
}
#[styled_component(CustomTextInput)]
pub fn text_input(props: &LoginDetailsModel) -> Html {
    let style = style!(
        r#"
        input {
                background-color: #0e2e018c;
                color: rgb(211, 209, 209);
                font-family: 'Noto Serif Display', serif;
                padding: 15pt 50pt;
                font-size: 15pt;
                border-width: 0px;
                margin-right: 10pt;
                margin-bottom: 20pt;
                margin-top: 20pt;
        }
        "#
    )
    .unwrap();
    let change_prop = props.onchange.clone();
    let on_change = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        change_prop.emit(value);
    });
    html! {
        <div class={style}>
        <input type="text" onchange={on_change} name={props.name.clone()} placeholder={props.placeholder.clone()}/>
        </div>

    }
}

#[styled_component(CustomPasswordInput)]
pub fn password_input(props: &PasswordModel) -> Html {
    let style = style!(
        r#"
        input {
                background-color: #0e2e018c;
                color: rgb(211, 209, 209);
                font-family: 'Noto Serif Display', serif;
                padding: 15pt 50pt;
                font-size: 15pt;
                border-width: 0px;
                margin-right: 10pt;
                margin-bottom: 20pt;
                margin-top: 20pt;
        }
        "#
    )
    .unwrap();
    let change_prop = props.onchange.clone();
    let on_change = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        change_prop.emit(value);
    });
    html! {
        <div class={style}>
        <input type="password" name={props.name.clone()} onchange={on_change} placeholder={props.placeholder.clone()}/>
        </div>

    }
}
