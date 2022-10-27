mod components;
pub mod routes;

use crate::components::{footer::Footer, header::Header};
use routes::{switch, AppRoute};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
            <div>
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
