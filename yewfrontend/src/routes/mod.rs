use yew::prelude::*;
use yew_router::Routable;

pub(crate) mod login;
pub(crate) mod not_found;
pub(crate) mod register;
use crate::components::home::Home;
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
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &AppRoute) -> Html {
    match route {
        AppRoute::Login => html! {<Login/>},
        AppRoute::Register => html! {<Register/>},
        AppRoute::Home => html! {<Home/>},
        AppRoute::NotFound => html! {<NotFound/>},
    }
}
