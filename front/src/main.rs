#![allow(non_snake_case)]
mod components;
mod models;

use components::{FilmModal, Footer, Header};
// Import the Dioxus prelude to gain access to the `rsx!` macro and the `Scope` and `Element` types.
use dioxus::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default().module_prefix("front"));
    // Launch the web application using the App component as the root.
    dioxus_web::launch(App);
}

// Define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        main {
            FilmModal {
                on_create_or_update: move |_| {},
                on_cancel: move |_| {},
            }
        }
    })
}

// #[derive(Props)]
// pub struct FilmModalProps<'a> {
//     on_create_or_update: EventHandler<'a, Film>,
//     on_cancel: EventHandler<'a, MouseEvent>,
//     #[props(!optional)]
//     film: Option<Film>,
// }

// #[inline_props]
// pub fn FilmCard<'a> (
//     cx: Scope<'a>,
//     film:  &'a Film,
//     on_edit: EventHandler<'a, MouseEvent>,
//     on_delete: EventHandler<'a, MouseEvent>,
// ) -> Element {

// }
