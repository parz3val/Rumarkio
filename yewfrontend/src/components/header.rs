use crate::{routes::AppRoute, UserAuth};
use gloo_console::log;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;

#[styled_component(Header)]
pub fn header() -> Html {
    let auth_state = use_context::<UserAuth>();
    let login = auth_state.unwrap_or_default().logged_in;
    log!(login);
    html! {
        <nav>
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
    html! {
        <div>
        <h1>
            {"You are logged in mister!"}
        </h1>
        </div>
    }
}
fn logout_view() -> Html {
    html! {
        <>

                <Link<AppRoute> to={AppRoute::Home}>
                    { "Rumarkio" }
                </Link<AppRoute>>

                <Link<AppRoute> to={AppRoute::Login} >
                    { "Sign in" }
                </Link<AppRoute>>

                <Link<AppRoute> to={AppRoute::Register} >
                    { "Sign up" }
                </Link<AppRoute>>

    </>
    }
}
