use crate::api::PublishState;
use angel_api_client_reqwest::types::personas::{PersonaId, PersonaSourceData, PersonaUpdate};
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};
use yew::prelude::*;

#[derive(Properties)]
pub struct EditPersonaDialogProps {
    pub persona: Option<PersonaSourceData>,
    pub on_close: Callback<()>,
    pub on_save: Callback<(PersonaId, PersonaUpdate)>,
}

impl PartialEq for EditPersonaDialogProps {
    fn eq(&self, other: &Self) -> bool {
        match (&self.persona, &other.persona) {
            (Some(a), Some(b)) => a.id == b.id,
            (None, None) => true,
            _ => false,
        }
    }
}

#[function_component(EditPersonaDialog)]
pub fn edit_persona_dialog(props: &EditPersonaDialogProps) -> Html {
    let is_loading = use_state(|| false);
    let name = use_state(|| String::new());
    let publish_state = use_state(|| PublishState::Private);
    let template = use_state(|| String::new());
    let voice_id = use_state(|| String::new());
    let model_id = use_state(|| Option::<String>::None);
    let stability = use_state(|| 0.5);
    let similarity_boost = use_state(|| 0.5);
    let style = use_state(|| Option::<f64>::None);
    let use_speaker_boost = use_state(|| false);

    // Update state when persona changes
    {
        let name = name.clone();
        let publish_state = publish_state.clone();
        let template = template.clone();
        let voice_id = voice_id.clone();
        let model_id = model_id.clone();
        let stability = stability.clone();
        let similarity_boost = similarity_boost.clone();
        let style = style.clone();
        let use_speaker_boost = use_speaker_boost.clone();
        let persona_id = props.persona.as_ref().map(|p| p.id);
        let persona_data = props.persona.as_ref().map(|p| {
            (
                p.name.clone(),
                p.publish_state,
                p.template.clone(),
                p.voice_id.clone(),
                p.model_id.clone(),
                p.stability,
                p.similarity_boost,
                p.style,
                p.use_speaker_boost,
            )
        });
        
        use_effect_with(persona_id, move |_| {
            if let Some((pname, ppublish, ptemplate, pvoice, pmodel, pstab, psim, pstyle, pspeaker)) = persona_data {
                name.set(pname);
                publish_state.set(ppublish);
                template.set(ptemplate);
                voice_id.set(pvoice);
                model_id.set(pmodel);
                stability.set(pstab);
                similarity_boost.set(psim);
                style.set(pstyle);
                use_speaker_boost.set(pspeaker.unwrap_or(false));
            }
            || ()
        });
    }

    if let Some(persona) = &props.persona {
        let on_name_change = {
            let name = name.clone();
            Callback::from(move |e: InputEvent| {
                if let Some(target) = e.target() {
                    if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                        name.set(input.value());
                    }
                }
            })
        };

        let on_template_change = {
            let template = template.clone();
            Callback::from(move |e: InputEvent| {
                if let Some(target) = e.target() {
                    if let Ok(textarea) = target.dyn_into::<HtmlTextAreaElement>() {
                        template.set(textarea.value());
                    }
                }
            })
        };

        let on_voice_id_change = {
            let voice_id = voice_id.clone();
            Callback::from(move |e: InputEvent| {
                if let Some(target) = e.target() {
                    if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                        voice_id.set(input.value());
                    }
                }
            })
        };

        let on_model_id_change = {
            let model_id = model_id.clone();
            Callback::from(move |e: Event| {
                if let Some(target) = e.target() {
                    if let Ok(select) = target.dyn_into::<HtmlSelectElement>() {
                        let value = select.value();
                        if value == "__none__" {
                            model_id.set(None);
                        } else {
                            model_id.set(Some(value));
                        }
                    }
                }
            })
        };

        let on_stability_change = {
            let stability = stability.clone();
            Callback::from(move |e: InputEvent| {
                if let Some(target) = e.target() {
                    if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                        if let Ok(val) = input.value().parse::<f64>() {
                            stability.set(val);
                        }
                    }
                }
            })
        };

        let on_similarity_boost_change = {
            let similarity_boost = similarity_boost.clone();
            Callback::from(move |e: InputEvent| {
                if let Some(target) = e.target() {
                    if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                        if let Ok(val) = input.value().parse::<f64>() {
                            similarity_boost.set(val);
                        }
                    }
                }
            })
        };

        let on_style_change = {
            let style = style.clone();
            Callback::from(move |e: InputEvent| {
                if let Some(target) = e.target() {
                    if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                        let value = input.value();
                        if value.is_empty() {
                            style.set(None);
                        } else if let Ok(val) = value.parse::<f64>() {
                            style.set(Some(val));
                        }
                    }
                }
            })
        };

        let on_speaker_boost_change = {
            let use_speaker_boost = use_speaker_boost.clone();
            Callback::from(move |e: Event| {
                if let Some(target) = e.target() {
                    if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                        use_speaker_boost.set(input.checked());
                    }
                }
            })
        };

        let on_publish_state_change = {
            let publish_state = publish_state.clone();
            Callback::from(move |e: Event| {
                if let Some(target) = e.target() {
                    if let Ok(select) = target.dyn_into::<HtmlSelectElement>() {
                        let new_state = match select.value().as_str() {
                            "Public" => PublishState::Public,
                            "Unlisted" => PublishState::Unlisted,
                            _ => PublishState::Private,
                        };
                        publish_state.set(new_state);
                    }
                }
            })
        };

        let on_submit = {
            let on_save = props.on_save.clone();
            let on_close = props.on_close.clone();
            let is_loading = is_loading.clone();
            let persona_id = persona.id;
            let name = name.clone();
            let publish_state = publish_state.clone();
            let template = template.clone();
            let voice_id = voice_id.clone();
            let model_id = model_id.clone();
            let stability = stability.clone();
            let similarity_boost = similarity_boost.clone();
            let style = style.clone();
            let use_speaker_boost = use_speaker_boost.clone();

            Callback::from(move |e: MouseEvent| {
                e.prevent_default();
                is_loading.set(true);
                
                let update = PersonaUpdate {
                    name: Some((*name).clone()),
                    publish_state: Some(*publish_state),
                    template: Some((*template).clone()),
                    voice_id: Some((*voice_id).clone()),
                    model_id: (*model_id).clone().map(|s| s.parse().ok()).flatten(),
                    stability: Some(*stability),
                    similarity_boost: Some(*similarity_boost),
                    style: *style,
                    use_speaker_boost: Some(*use_speaker_boost),
                    image: None,
                    thumbnail: None,
                    sample_audio: None,
                    seed: None,
                };
                
                on_save.emit((persona_id, update));
                on_close.emit(());
            })
        };

        let on_cancel = {
            let on_close = props.on_close.clone();
            Callback::from(move |_| {
                on_close.emit(());
            })
        };

        let on_backdrop_click = {
            let on_close = props.on_close.clone();
            Callback::from(move |e: MouseEvent| {
                e.stop_propagation();
                on_close.emit(());
            })
        };

        let on_dialog_click = Callback::from(|e: MouseEvent| {
            e.stop_propagation();
        });

        html! {
            <div
                class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
                onclick={on_backdrop_click}
            >
                <div
                    class="bg-white rounded-lg shadow-xl max-w-3xl w-full mx-4 max-h-[80vh] overflow-y-auto"
                    onclick={on_dialog_click}
                >
                    // Header
                    <div class="p-6 border-b">
                        <h2 class="text-2xl font-bold">{"Edit Persona"}</h2>
                        <p class="text-sm text-gray-600 mt-1">{"Update the persona details. Changes will be saved immediately."}</p>
                    </div>

                    // Content
                    <div class="p-6 space-y-6">
                        // Basic Information Section
                        <div class="space-y-4">
                            <h3 class="text-lg font-semibold">{"Basic Information"}</h3>
                            <div class="grid gap-4">
                                // Name Field
                                <div class="space-y-2">
                                    <label for="name" class="text-sm font-medium text-gray-700">{"Name"}</label>
                                    <input
                                        id="name"
                                        type="text"
                                        class="w-full px-3 py-2 border border-gray-300 rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                                        value={(*name).clone()}
                                        oninput={on_name_change}
                                        placeholder="Enter persona name"
                                    />
                                </div>

                                // Publish State Field
                                <div class="space-y-2">
                                    <label for="publishState" class="text-sm font-medium text-gray-700">{"Publish State"}</label>
                                    <select
                                        id="publishState"
                                        class="w-full px-3 py-2 border border-gray-300 rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                                        value={format!("{:?}", *publish_state)}
                                        onchange={on_publish_state_change}
                                    >
                                        <option value="Public">{"Public"}</option>
                                        <option value="Unlisted">{"Unlisted"}</option>
                                        <option value="Private">{"Private"}</option>
                                    </select>
                                </div>
                            </div>
                        </div>

                        // Voice Settings Section
                        <div class="space-y-4">
                            <h3 class="text-lg font-semibold">{"Voice Settings"}</h3>
                            <div class="grid gap-4">
                                // Voice ID Field
                                <div class="space-y-2">
                                    <label for="voiceId" class="text-sm font-medium text-gray-700">{"Voice ID"}</label>
                                    <input
                                        id="voiceId"
                                        type="text"
                                        class="w-full px-3 py-2 border border-gray-300 rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent font-mono"
                                        value={(*voice_id).clone()}
                                        oninput={on_voice_id_change}
                                        placeholder="Enter voice ID"
                                    />
                                </div>

                                // Model ID Field
                                <div class="space-y-2">
                                    <label for="modelId" class="text-sm font-medium text-gray-700">{"Model ID"}</label>
                                    <select
                                        id="modelId"
                                        class="w-full px-3 py-2 border border-gray-300 rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent font-mono"
                                        value={(*model_id).clone().unwrap_or_else(|| "__none__".to_string())}
                                        onchange={on_model_id_change}
                                    >
                                        <option value="__none__">{"None"}</option>
                                        <option value="eleven_turbo_v2_5">{"eleven_turbo_v2_5"}</option>
                                        <option value="eleven_turbo_v2">{"eleven_turbo_v2"}</option>
                                        <option value="eleven_multilingual_v2">{"eleven_multilingual_v2"}</option>
                                        <option value="eleven_flash_v2_5">{"eleven_flash_v2_5"}</option>
                                        <option value="eleven_flash_v2">{"eleven_flash_v2"}</option>
                                    </select>
                                </div>

                                // Numeric Fields Row
                                <div class="grid grid-cols-3 gap-4">
                                    <div class="space-y-2">
                                        <label for="stability" class="text-sm font-medium text-gray-700">{"Stability"}</label>
                                        <input
                                            id="stability"
                                            type="number"
                                            min="0"
                                            max="1"
                                            step="0.01"
                                            class="w-full px-3 py-2 border border-gray-300 rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                                            value={format!("{}", *stability)}
                                            oninput={on_stability_change}
                                        />
                                    </div>
                                    <div class="space-y-2">
                                        <label for="similarityBoost" class="text-sm font-medium text-gray-700">{"Similarity Boost"}</label>
                                        <input
                                            id="similarityBoost"
                                            type="number"
                                            min="0"
                                            max="1"
                                            step="0.01"
                                            class="w-full px-3 py-2 border border-gray-300 rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                                            value={format!("{}", *similarity_boost)}
                                            oninput={on_similarity_boost_change}
                                        />
                                    </div>
                                    <div class="space-y-2">
                                        <label for="style" class="text-sm font-medium text-gray-700">{"Style"}</label>
                                        <input
                                            id="style"
                                            type="number"
                                            min="0"
                                            max="1"
                                            step="0.01"
                                            class="w-full px-3 py-2 border border-gray-300 rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                                            value={(*style).map(|s| format!("{}", s)).unwrap_or_default()}
                                            oninput={on_style_change}
                                        />
                                    </div>
                                </div>

                                // Speaker Boost Toggle
                                <div class="flex items-center space-x-2">
                                    <input
                                        id="useSpeakerBoost"
                                        type="checkbox"
                                        checked={*use_speaker_boost}
                                        onchange={on_speaker_boost_change}
                                        class="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
                                    />
                                    <label for="useSpeakerBoost" class="text-sm font-medium text-gray-700 cursor-pointer">
                                        {"Enable Speaker Boost"}
                                    </label>
                                </div>
                            </div>
                        </div>

                        // Template Section
                        <div class="space-y-4">
                            <h3 class="text-lg font-semibold">{"Template"}</h3>
                            <div class="space-y-2">
                                <label for="template" class="text-sm font-medium text-gray-700">{"Persona Template"}</label>
                                <textarea
                                    id="template"
                                    rows="8"
                                    class="w-full px-3 py-2 border border-gray-300 rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent font-mono text-sm"
                                    value={(*template).clone()}
                                    oninput={on_template_change}
                                    placeholder="Enter persona template"
                                />
                            </div>
                        </div>
                    </div>

                    // Footer
                    <div class="flex justify-end gap-3 p-6 border-t bg-gray-50">
                        <button
                            onclick={on_cancel}
                            disabled={*is_loading}
                            class="px-4 py-2 border border-gray-300 rounded-lg text-gray-700 hover:bg-gray-50 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                            {"Cancel"}
                        </button>
                        <button
                            onclick={on_submit}
                            disabled={*is_loading}
                            class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                            {if *is_loading { "Saving..." } else { "Save Changes" }}
                        </button>
                    </div>
                </div>
            </div>
        }
    } else {
        html! {}
    }
}
