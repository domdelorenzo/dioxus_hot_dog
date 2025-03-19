use dioxus::prelude::*;

#[component]
pub fn Favorites() -> Element {
    // Create a pending resource that resolves to the list of dogs from the backend
    // Wait for the favorites list to resolve with `.suspend()`
    // However, this won't allow you to call favorites.restart() to update the list on deletion 
    // let favorites: MappedSignal<Result<Vec<(usize, String)>, ServerFnError>> = use_resource(crate::backend::list_dogs).suspend()?;

    // make favorites mutable and remove suspend so that we can call favorites.restart() to update the list on deletion
    let mut favorites = use_resource(crate::backend::list_dogs);
    // assign favorites to a signal and suspend there to pass it to the for loop
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