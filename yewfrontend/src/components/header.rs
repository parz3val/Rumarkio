use crate::routes::AppRoute;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;


#[styled_component(Header)]
pub fn header() -> Html {
    let login = false;
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
    html! {}
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
