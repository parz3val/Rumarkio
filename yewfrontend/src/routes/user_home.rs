use crate::routes::login::Login;
use gloo_console::log;
use stylist::yew::styled_component;
use yew::prelude::*;

use crate::UserAuth;

// const STYLE_FILE: &str = include_str!("home.css");

#[styled_component(UserHome)]
pub fn home() -> Html {
    // let stylesheet = stylist::Style::new(STYLE_FILE).expect("Couldn't load home.css");
    let auth = use_context::<UserAuth>();
    let auth = auth.unwrap().logged_in;
    log!("The auth status is :: ", auth.clone());
    html! {
        if auth {
            <LoggedInHome />
        } else {
            <Login />
        }

    }
}

#[styled_component(LoggedInHome)]
pub fn logged_in_home() -> Html {
    // let stylesheet = stylist::Style::new(STYLE_FILE).expect("Couldn't load home.css");
    html! {
        <div>
        </div>
    }
}
