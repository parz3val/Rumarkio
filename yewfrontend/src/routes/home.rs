use yew::prelude::*;
use stylist::yew::styled_component;

// const STYLE_FILE: &str = include_str!("home.css");

#[styled_component(Home)]
pub fn home() -> Html {
    // let stylesheet = stylist::Style::new(STYLE_FILE).expect("Couldn't load home.css");
    html! {
        <div>
        <div class="hero-container">
        <img src="https://images.unsplash.com/photo-1472173148041-00294f0814a2?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1170&q=80" alt="vintage table with book and cameras" class="hero-img"/>
        <div class="hero-text">{"Bookmark your life"}</div>
        </div>
        </div>
    }
}
