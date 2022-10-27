use stylist::{style, yew::styled_component};
use yew::{html, Properties};

#[derive(Properties, Clone, PartialEq)]
pub struct CustomInput {
    pub name: String,
    pub placeholder: String,
}

#[styled_component(CustomTextInput)]
pub fn text_input(props: &CustomInput) -> Html {
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
    html! {
        <div class={style}>
        <input type="text" name={props.name.clone()} placeholder={props.placeholder.clone()}/>
        </div>

    }
}

#[styled_component(CustomPasswordInput)]
pub fn password_input(props: &CustomInput) -> Html {
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
    html! {
        <div class={style}>
        <input type="password" name={props.name.clone()} placeholder={props.placeholder.clone()}/>
        </div>

    }
}