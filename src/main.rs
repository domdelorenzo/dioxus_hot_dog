mod backend;
mod components;

use components::{DogView, Favorites, NavBar};
use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.css");

fn main() {

    #[cfg(not(feature = "server"))]
    server_fn::client::set_server_url("http://127.0.0.1:8080");

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        Router::<Route> {}
    }
}

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    DogView,
    #[route("/favorites")]
    Favorites,
}
