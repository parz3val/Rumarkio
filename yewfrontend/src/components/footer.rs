use stylist::yew::styled_component;
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("footer.css");

#[styled_component(Footer)]
pub fn footer() -> Html {
    let style = stylist::Style::new(STYLE_FILE).unwrap();
    html! {
        <footer class={style}>
        <div>
        <p>
        <ul>
        <li>
        {"Rumarkio @2022"}
        </li>
        <li>
        {"Built with Rust and Yew"}
        </li>
        </ul>
        </p>
        </div>
        </footer>
    }
}
