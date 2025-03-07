use dioxus::prelude::*;

mod components;
mod backend;

use crate::components::*;

static CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        // Title {}
        // DogView {}
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
    // #[route("/:..segments")]
    // PageNotFound { segments: Vec<String> }, // <------ [TODO] can't find this route
}
