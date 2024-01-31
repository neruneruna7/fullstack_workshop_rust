#![allow(non_snake_case)]
mod components;
mod models;

use components::{FilmCard, FilmModal, Footer, Header};
// Import the Dioxus prelude to gain access to the `rsx!` macro and the `Scope` and `Element` types.
use dioxus::{html::section, prelude::*};

use models::FilmModalVisibility;
use shared::models::Film;

fn main() {
    wasm_logger::init(wasm_logger::Config::default().module_prefix("front"));
    // Launch the web application using the App component as the root.
    dioxus_web::launch(App);
}

// Define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    // グローバルな状態
    use_shared_state_provider(cx, || FilmModalVisibility(false));
    let is_modal_visible = use_shared_state::<FilmModalVisibility>(cx).unwrap();

    // ローカルな状態
    let films = use_state::<Option<Vec<Film>>>(cx, || None);
    let selected_film = use_state::<Option<Film>>(cx, || None);
    let force_get_films = use_state(cx, || ());

    cx.render(rsx! {
        main {
            section {
                class: "md:container md:mx-auto md:py-8 flex-1",
                if let Some(films) = films.get() {
                    rsx!(
                        ul {
                            class: "flex flex-row justify-center items-stretch gap-4 flex-wrap",
                            {films.iter().map(|film| {
                                rsx!(FilmCard {
                                    key: "{film.id}",
                                    film: film,
                                    on_edit: move |_| {
                                        selected_film.set(Some(film.clone()));
                                        is_modal_visible.write().0 = true;
                                    },
                                    on_delete: move |_| {}
                                })
                            })}
                        }
                    )
                }
            }
            FilmModal {
                film: selected_film.get().clone(),
                on_create_or_update: move |new_film| {},
                on_cancel: move |_| {
                    selected_film.set(None);
                    is_modal_visible.write().0 = false;
                },
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
