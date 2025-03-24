// Prevents additional console window on Windows in release,
// run bundler with `--platform windows` not `--platform desktop` to ensure config is applied
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod backend;
mod components;

use components::{DogView, Favorites, NavBar};
use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.css");

fn main() {
    // dioxus::launch(App);
   
    #[cfg(feature = "desktop")]
    fn launch_app() {
        use dioxus::desktop::tao;
        let window = tao::window::WindowBuilder::new().with_resizable(true);
        dioxus::LaunchBuilder::new().with_cfg(dioxus::desktop::Config::new().with_window(window).with_menu(None)).launch(App);
    }

    #[cfg(not(feature = "desktop"))]
    fn launch_app() {
        dioxus::launch(App);
    }

    launch_app();
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
