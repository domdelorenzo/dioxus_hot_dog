use dioxus::prelude::*;

#[component]
pub fn Favorites() -> Element {
    // Create a pending resource that resolves to the list of dogs from the backend
    // Wait for the favorites list to resolve with `.suspend()`
    // let favorites: MappedSignal<Result<Vec<(usize, String)>, ServerFnError>> = use_resource(crate::backend::list_dogs).suspend()?;


    // let mut favorite_resource = use_resource(crate::backend::list_dogs);
    // let favorites = favorite_resource.suspend()?;

    let mut favorites = use_resource(crate::backend::list_dogs);
    let favorites_signal = favorites.suspend()?;
    rsx! {
        div { id: "favorites",
            div { id: "favorites-container",
                for (id , url) in favorites_signal().unwrap() {
                    div { key: id, class: "favorite-dog",
                        img { src: "{url}" }
                        div { class: "favorite-dog",
                            button {
                                id: "delete",
                                onclick: move |_| async move {
                                    _ = crate::backend::delete_dog(id).await;
                                    favorites.restart();
                                },
                                "❌"
                            }
                        }
                    }
                }
            }
        }
    }
}