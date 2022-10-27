use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;

#[function_component(Header)]
pub fn header() -> Html {
    let login = false;
    html! {
        <nav class="navbar">
        {
            if login {
                login_view()
            } else {
                logout_view()
            }
        }
        </nav>
    }
}

fn login_view() -> Html {
    html! {}
}
fn logout_view() -> Html {
    html! {
        <>
            <ul class="nav navbar-nav pull-xs-right">
            <li class="nav-item">
                <Link<AppRoute> to={AppRoute::Home}>
                    { "Home" }
                </Link<AppRoute>>
            </li>
            <li class="nav-item">
                <Link<AppRoute> to={AppRoute::Login} classes="nav-link">
                    { "Sign in" }
                </Link<AppRoute>>
            </li>
            <li class="nav-item">
                <Link<AppRoute> to={AppRoute::Register} classes="nav-link">
                    { "Sign up" }
                </Link<AppRoute>>
            </li>
        </ul>
    </>
    }
}
