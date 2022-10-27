use crate::components::login_form::LoginForm;
use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <div class="login-page">
        <h1 class="page-title"> {"Login page"}</h1>
        <LoginForm/>
        </div>
    }
}
