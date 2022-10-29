use std::ops::Deref;

use crate::molecules;
use crate::molecules::custom_input::CustomPasswordInput;
use molecules::custom_button::LoginButton;
use molecules::custom_button::ResetButton;
use molecules::custom_input::CustomTextInput;

use crate::types;
use types::api::user::LoginDetails;

use stylist::yew::styled_component;
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("login_form.css");

#[styled_component(LoginForm)]
pub fn login_form() -> Html {
    let style = stylist::Style::new(STYLE_FILE).unwrap();
    let details_state = use_state(|| LoginDetails::default());
    
    // handle form submit
    let form_submitted = 
        Callback::from(move |event: FocusEvent| {
        let details_state = details_state.clone();
            event.prevent_default();
            let _data = details_state.deref().clone();
        });
    html! {
        <form onsubmit={form_submitted}>
        <div class={style}>
            <div class="login-container">
                <CustomTextInput name={"username".to_owned()} placeholder={"username".to_owned()}/>
                <CustomPasswordInput name={"password".to_owned()} placeholder={"password".to_owned()}/>
                <div class="button-container">
                    <LoginButton/>
                    <ResetButton/>
                </div>
            </div>
        </div>
        </form>
    }
}
