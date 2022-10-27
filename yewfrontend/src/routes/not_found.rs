use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <p>{"Sorry the page you are looing for isn't available"}</p>
    }
}
