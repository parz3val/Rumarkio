use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div>
        <h1>{"Not found!"}</h1>
        <p>{"Sorry the page you are looing for isn't available"}</p>
        </div>
    }
}
