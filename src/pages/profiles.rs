use crate::api::{delete_persona, get_persona_source, get_personas, update_persona};
use crate::components::personas::{EditPersonaDialog, ViewPersonaDialog};
use crate::components::{Button, PersonasTable};
use angel_api_client_reqwest::types::personas::{
    Persona, PersonaId, PersonaSourceData, PersonaUpdate,
};
use gloo_console::log;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(PersonasPage)]
pub fn personas_page() -> Html {
    let personas = use_state(|| Vec::<Persona>::new());
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let view_persona = use_state(|| None::<PersonaSourceData>);
    let edit_persona = use_state(|| None::<PersonaSourceData>);

    // Use the same API base URL as the rest of the app
    const API_BASE_URL: &str = "https://dev-api.angelq.ai";
    let base_url = API_BASE_URL;

    // Fetch personas on mount
    {
        let personas = personas.clone();
        let loading = loading.clone();
        let error = error.clone();
        let base_url = base_url.to_string();

        use_effect_with((), move |_| {
            spawn_local(async move {
                log!("Fetching personas from:", base_url.clone());
                match get_personas(&base_url).await {
                    Ok(data) => {
                        log!("Successfully fetched personas:", data.len());
                        // Filter out deleted personas
                        let active_personas: Vec<Persona> = data
                            .into_iter()
                            .filter(|p| p.deleted_at.is_none())
                            .collect();
                        log!("Active personas:", active_personas.len());
                        personas.set(active_personas);
                        loading.set(false);
                    }
                    Err(e) => {
                        log!("Error fetching personas:", e.clone());
                        error.set(Some(e));
                        loading.set(false);
                    }
                }
            });
            || ()
        });
    }

    let on_view = {
        let view_persona = view_persona.clone();
        let base_url = base_url.to_string();
        Callback::from(move |id: PersonaId| {
            log!(
                "on_view callback - fetching persona source:",
                id.to_string()
            );
            let view_persona = view_persona.clone();
            let base_url = base_url.clone();

            spawn_local(async move {
                match get_persona_source(&base_url, &id).await {
                    Ok(Some(persona_source)) => {
                        log!(
                            "on_view - successfully fetched persona source:",
                            persona_source.name.clone()
                        );
                        view_persona.set(Some(persona_source));
                    }
                    Ok(None) => {
                        log!("on_view - persona source not found");
                    }
                    Err(e) => {
                        log!("on_view - error fetching persona source:", e);
                    }
                }
            });
        })
    };

    let on_close_view = {
        let view_persona = view_persona.clone();
        Callback::from(move |_| {
            view_persona.set(None);
        })
    };

    let on_edit = {
        let edit_persona = edit_persona.clone();
        let base_url = base_url.to_string();
        Callback::from(move |id: PersonaId| {
            log!(
                "on_edit callback - fetching persona source:",
                id.to_string()
            );
            let edit_persona = edit_persona.clone();
            let base_url = base_url.clone();

            spawn_local(async move {
                match get_persona_source(&base_url, &id).await {
                    Ok(Some(persona_source)) => {
                        log!(
                            "on_edit - successfully fetched persona source:",
                            persona_source.name.clone()
                        );
                        edit_persona.set(Some(persona_source));
                    }
                    Ok(None) => {
                        log!("on_edit - persona source not found");
                    }
                    Err(e) => {
                        log!("on_edit - error fetching persona source:", e);
                    }
                }
            });
        })
    };

    let on_close_edit = {
        let edit_persona = edit_persona.clone();
        Callback::from(move |_| {
            edit_persona.set(None);
        })
    };

    let on_save_edit = {
        let edit_persona = edit_persona.clone();
        let personas = personas.clone();
        let base_url = base_url.to_string();
        Callback::from(move |(id, update_data): (PersonaId, PersonaUpdate)| {
            let edit_persona = edit_persona.clone();
            let personas = personas.clone();
            let base_url = base_url.clone();

            spawn_local(async move {
                match update_persona(&base_url, &id, update_data).await {
                    Ok(_) => {
                        log!("Persona updated successfully");
                        edit_persona.set(None);
                        // Refresh the list
                        match get_personas(&base_url).await {
                            Ok(data) => {
                                let active_personas: Vec<Persona> = data
                                    .into_iter()
                                    .filter(|p| p.deleted_at.is_none())
                                    .collect();
                                personas.set(active_personas);
                            }
                            Err(e) => {
                                log!("Error refreshing personas:", e);
                            }
                        }
                    }
                    Err(e) => {
                        log!("Error updating persona:", e);
                    }
                }
            });
        })
    };

    let on_delete = {
        let personas = personas.clone();
        let base_url = base_url.to_string();

        Callback::from(move |id: PersonaId| {
            let personas = personas.clone();
            let base_url = base_url.clone();

            spawn_local(async move {
                match delete_persona(&base_url, &id).await {
                    Ok(_) => {
                        log!("Persona deleted successfully");
                        // Refresh the list
                        match get_personas(&base_url).await {
                            Ok(data) => {
                                let active_personas: Vec<Persona> = data
                                    .into_iter()
                                    .filter(|p| p.deleted_at.is_none())
                                    .collect();
                                personas.set(active_personas);
                            }
                            Err(e) => {
                                log!("Error refreshing personas:", e);
                            }
                        }
                    }
                    Err(e) => {
                        log!("Error deleting persona:", e);
                    }
                }
            });
        })
    };

    html! {
        <div class="p-8">
            <div class="mb-6 flex justify-between items-center">
                <div>
                    <h2 class="text-2xl font-bold tracking-tight">{"Persona List"}</h2>
                    <p class="text-gray-600 mt-1">{"Manage your AI personas and their characteristics here."}</p>
                </div>
                <Button variant={crate::components::ButtonVariant::Primary}>
                    {"Create Persona"}
                </Button>
            </div>

            {
                if *loading {
                    html! {
                        <div class="flex justify-center items-center py-12">
                            <div class="text-gray-500">{"Loading personas..."}</div>
                        </div>
                    }
                } else if let Some(err) = (*error).as_ref() {
                    html! {
                        <div class="bg-red-50 border border-red-200 text-red-800 px-4 py-3 rounded-lg">
                            <p class="font-semibold">{"Error loading personas"}</p>
                            <p class="text-sm">{err}</p>
                        </div>
                    }
                } else {
                    html! {
                        <PersonasTable
                            personas={(*personas).clone()}
                            on_view={on_view}
                            on_edit={on_edit}
                            on_delete={on_delete}
                        />
                    }
                }
            }

            // View dialog
            <ViewPersonaDialog
                persona={(*view_persona).clone()}
                on_close={on_close_view}
            />

            // Edit dialog
            <EditPersonaDialog
                persona={(*edit_persona).clone()}
                on_close={on_close_edit}
                on_save={on_save_edit}
            />
        </div>
    }
}
