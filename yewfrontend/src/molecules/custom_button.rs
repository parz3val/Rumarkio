use stylist::{yew::styled_component, style};
use yew::{html, prelude};


#[styled_component(LoginButton)]
pub fn login_button() -> Html {
    let stylesheet = style!(
        r#"
            button {
                background-color: #0e2e018c;
                color: rgb(211, 209, 209);
                font-family: 'Noto Serif Display', serif;
                padding: 15pt 50pt;
                font-size: 15pt;
                border-width: 0px;
                margin-right: 20pt;
            }
            button:hover {
                background-color: #8456168c;
                color: rgb(25, 24, 24);
                font-family: 'Noto Serif Display', serif;
                padding: 15pt 50pt;
                font-size: 16pt;
                border-width: 0px;
            }
        "#
    ).unwrap();
    html! {
        <div class={stylesheet}>
        <button >{"Login"}</button>
        </div>
    }
}

#[styled_component(RegisterButton)]
pub fn register_button() -> Html {
    let stylesheet = style!(
        r#"
            button {
                background-color: #0e2e018c;
                color: rgb(211, 209, 209);
                font-family: 'Noto Serif Display', serif;
                padding: 15pt 50pt;
                font-size: 15pt;
                border-width: 0px;
            }
            button:hover {
                background-color: #8456168c;
                color: rgb(25, 24, 24);
                font-family: 'Noto Serif Display', serif;
                padding: 15pt 50pt;
                font-size: 16pt;
                border-width: 0px;
            }
        "#
    ).unwrap();
    html! {
        <div class={stylesheet}>
        <button >{"Register"}</button>
        </div>
    }
}

#[styled_component(ResetButton)]
pub fn reset_button() -> Html {
    let stylesheet = style!(
        r#"
            button {
                background-color: #0e2e018c;
                color: rgb(211, 209, 209);
                font-family: 'Noto Serif Display', serif;
                padding: 15pt 50pt;
                font-size: 15pt;
                border-width: 0px;
                margin-right: 20pt;
            }
            button:hover {
                background-color: #8456168c;
                color: rgb(25, 24, 24);
                font-family: 'Noto Serif Display', serif;
                padding: 15pt 50pt;
                font-size: 16pt;
                border-width: 0px;
            }
        "#
    ).unwrap();
    html! {
        <div class={stylesheet}>
        <button >{"Reset"}</button>
        </div>
    }
}


