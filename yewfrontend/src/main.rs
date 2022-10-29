mod components;
pub mod molecules;
pub mod routes;
pub mod types;

use std::ops::Deref;

use crate::components::{footer::Footer, header::Header};
use gloo_console::log;
use routes::{switch, AppRoute};
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;

const STYLE_FILE: &str = include_str!("main.css");

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct UserAuth {
    pub access_token: String,
    pub logged_in: bool,
    pub login_callback: Callback<UserAuth>,
}

#[styled_component(App)]
fn app() -> Html {
    let stylesheet = stylist::Style::new(STYLE_FILE).expect("Failed to load css");
    // let auth_state = use_state(|| UserAuth {
    //     access_token: String::new(),
    //     logged_in: false,
    //     login_callback: (auth_handler),
    // });
    let logged_in = use_state(|| false);
    let logged_in_cloned = logged_in.clone();
    let logged_in_state = logged_in.clone();
    let auth_handler =
        Callback::from(move |auth_state: UserAuth| logged_in_cloned.set(auth_state.logged_in));
    let auth_state = use_state(|| UserAuth {
        access_token: String::new(),
        logged_in: *logged_in_state,
        login_callback: (auth_handler),
    });

    html! {
        <ContextProvider<UserAuth> context={auth_state.deref().clone()}>
            <div class={stylesheet}>
            <BrowserRouter>
            <Header auth={*logged_in}/>
            <Switch<AppRoute> render={Switch::render(switch)} />
            </BrowserRouter>
            <Footer/>
            </div>
        </ContextProvider<UserAuth>>
    }
}

fn main() {
    yew::start_app::<App>();
}
