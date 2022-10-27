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

        {"Rumarkio @2022"}
</p>
<p>
        {"Built with Rust and Yew"}
</p>
        </div>
        </footer>
    }
}
