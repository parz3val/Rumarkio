use yew::prelude::*;
use yew_router::Routable;

pub(crate) mod home;
pub(crate) mod fancy_home;
pub(crate) mod login;
pub(crate) mod not_found;
pub(crate) mod register;
pub(crate) mod user_home;

use home::Home;
use user_home::UserHome;
use login::Login;
use not_found::NotFound;
use register::Register;

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/")]
    Home,
    #[at("/home")]
    UserHome,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &AppRoute) -> Html {
    match route {
        AppRoute::Login => html! {<Login/>},
        AppRoute::Register => html! {<Register/>},
        AppRoute::Home => html! {<Home/>},
        AppRoute::UserHome => html! {<UserHome/>},
        AppRoute::NotFound => html! {<NotFound/>},
    }
}
