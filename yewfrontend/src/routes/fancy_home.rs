use yew::prelude::*;
use stylist::yew::styled_component;

// const STYLE_FILE: &str = include_str!("home.css");

#[styled_component(FancyHome)]
pub fn home() -> Html {
    // let stylesheet = stylist::Style::new(STYLE_FILE).expect("Couldn't load home.css");
    html! {
        <div>
            <div class="container">
                <div class="box">
                    <img src="https://images.unsplash.com/photo-1457369804613-52c61a468e7d?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=2670&q=80" />
                    <span>{"Rumarkio let's you easily bookmark your research and links!"}</span>
                    <p class="title">{"Easy bookmarking"}</p>
                </div>
                <div class="box">
                    <img src="https://images.unsplash.com/photo-1615114814213-a245ffc79e9a?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=880&q=80" />
                    <span>{"Rumarkio offers easy online sync."}</span>
                    <p class="title">{"Online sync"}</p>
                </div>
                <div class="box">
                    <img src="https://images.unsplash.com/photo-1553356126-71d9da2295e2?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1470&q=80" />
                    <span>{"Easily collect reasearch from chrome."}</span>
                    <p class="title">{"Chrome extension"}</p>
                </div>
                <div class="box">
                    <img src="https://images.unsplash.com/photo-1590607669385-730b705cbd88?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1374&q=80" />
                    <span>{"Free and open source!"}</span>
                    <p class="title">{"Free to use"}</p>
                </div>
            </div>
        </div>
    }
}
