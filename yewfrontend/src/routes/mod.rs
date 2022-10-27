use yew::prelude::*;
use yew_router::Routable;

pub(crate) mod home;
pub(crate) mod fancy_home;
pub(crate) mod login;
pub(crate) mod not_found;
pub(crate) mod register;
use home::Home;
use fancy_home::FancyHome;
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
    FancyHome,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &AppRoute) -> Html {
    match route {
        AppRoute::Login => html! {<Login/>},
        AppRoute::Register => html! {<Register/>},
        AppRoute::Home => html! {<Home/>},
        AppRoute::FancyHome => html! {<FancyHome/>},
        AppRoute::NotFound => html! {<NotFound/>},
    }
}
