mod components;
pub mod routes;

use crate::components::{footer::Footer, header::Header};
use routes::{switch, AppRoute};
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;

const STYLE_FILE: &str = include_str!("main.css");

#[styled_component(App)]
fn app() -> Html {
    let stylesheet = stylist::Style::new(STYLE_FILE).expect("Failed to load css");
    html! {
            <div class={stylesheet}>
            <BrowserRouter>
            <Header/>
            <Switch<AppRoute> render={Switch::render(switch)} />
            </BrowserRouter>
            <Footer/>
            </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
