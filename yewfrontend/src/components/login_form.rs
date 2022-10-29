use std::ops::Deref;

use crate::molecules;
use crate::molecules::custom_input::CustomPasswordInput;
use crate::types::api::user::api_login;
use crate::UserAuth;
use molecules::custom_button::LoginButton;
use molecules::custom_button::ResetButton;
use molecules::custom_input::CustomTextInput;

use crate::types;
use types::api::user::LoginDetails;

use gloo_console::log;
use stylist::yew::styled_component;
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("login_form.css");

#[styled_component(LoginForm)]
pub fn login_form() -> Html {
    let style = stylist::Style::new(STYLE_FILE).unwrap();
    let details_state = use_state(|| LoginDetails::default());
    let auth_state = use_context::<UserAuth>();

    // handle form submit
    let auth = auth_state.clone();
    let details = details_state.clone();
    let form_submitted = Callback::from(move |event: FocusEvent| {
        event.prevent_default();
        let _data = details.deref().clone();
        let username = _data.username.clone();
        let password = _data.password.clone();
        let auth_clone = auth_state.clone();
        // log!("Form submitted event login form");
        wasm_bindgen_futures::spawn_local(async {
            let response = api_login(username, password).await;
            let original_callback = auth_clone.clone();
            auth_clone.unwrap().login_callback.emit(UserAuth {
                access_token: response.accessToken,
                logged_in: true,
                login_callback: (original_callback.unwrap().login_callback),
            });
            // log!(response.accessToken);
        });
    });
    let details = details_state.clone();
    let on_password_change = Callback::from(move |password: String| {
        details.set(LoginDetails {
            password,
            ..details.deref().clone()
        });
    });
    let details = details_state.clone();
    let on_username_change = Callback::from(move |username: String| {
        details.set(LoginDetails {
            username,
            ..details.deref().clone()
        });
    });
    html! {
        <form onsubmit={form_submitted}>
        <div class={style}>
            <div class="login-container">
                <CustomTextInput name={"username".to_owned()} onchange= {on_username_change} placeholder={"username".to_owned()}/>
                <CustomPasswordInput name={"password".to_owned()} onchange = {on_password_change} placeholder={"password".to_owned()}/>
                <div class="button-container">
                    <LoginButton/>
                    <ResetButton/>
                </div>
            </div>
        </div>
        </form>
    }
}
