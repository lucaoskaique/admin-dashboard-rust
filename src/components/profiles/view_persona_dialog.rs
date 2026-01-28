use crate::api::PublishState;
use angel_api_client_reqwest::types::personas::PersonaSourceData;
use yew::prelude::*;

#[derive(Properties)]
pub struct ViewPersonaDialogProps {
    pub persona: Option<PersonaSourceData>,
    pub on_close: Callback<()>,
}

impl PartialEq for ViewPersonaDialogProps {
    fn eq(&self, other: &Self) -> bool {
        // Compare based on persona ID if present
        match (&self.persona, &other.persona) {
            (Some(a), Some(b)) => a.id == b.id,
            (None, None) => true,
            _ => false,
        }
    }
}

#[function_component(ViewPersonaDialog)]
pub fn view_persona_dialog(props: &ViewPersonaDialogProps) -> Html {
    if let Some(persona) = &props.persona {
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

        let publish_badge = match persona.publish_state {
            PublishState::Public => html! {
                <span class="px-2 py-1 bg-blue-100 text-blue-800 rounded text-xs font-medium">
                    {"Public"}
                </span>
            },
            PublishState::Private => html! {
                <span class="px-2 py-1 bg-gray-100 text-gray-800 rounded text-xs font-medium">
                    {"Private"}
                </span>
            },
            PublishState::Unlisted => html! {
                <span class="px-2 py-1 bg-yellow-100 text-yellow-800 rounded text-xs font-medium">
                    {"Unlisted"}
                </span>
            },
        };

        let status_badge = if persona.deleted_at.is_some() {
            html! {
                <span class="px-2 py-1 bg-red-100 text-red-800 rounded text-xs font-medium">
                    {"Deleted"}
                </span>
            }
        } else {
            html! {
                <span class="px-2 py-1 bg-green-100 text-green-800 rounded text-xs font-medium">
                    {"Active"}
                </span>
            }
        };

        let updated_display = if let Some(updated) = &persona.updated_at {
            // Format the date/time in a simple way without external dependencies
            format!(
                "{}/{}/{}, {}:{}:{}",
                updated.month() as u8,
                updated.day(),
                updated.year(),
                updated.hour(),
                updated.minute(),
                updated.second()
            )
        } else {
            "—".to_string()
        };

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
                        <h2 class="text-2xl font-bold">{"Persona Details"}</h2>
                        <p class="text-sm text-gray-600 mt-1">{"View complete information about this persona"}</p>
                    </div>

                    // Content
                    <div class="p-6 space-y-6">
                        // Basic Information
                        <div class="space-y-4">
                            <h3 class="text-lg font-semibold">{"Basic Information"}</h3>
                            <div class="grid grid-cols-2 gap-4">
                                <div class="space-y-2">
                                    <label class="text-sm font-medium text-gray-700">{"Name"}</label>
                                    <p class="text-sm">{&persona.name}</p>
                                </div>
                                <div class="space-y-2">
                                    <label class="text-sm font-medium text-gray-700">{"Codename"}</label>
                                    <p class="text-sm">
                                        {
                                            if let Some(codename) = &persona.codename {
                                                html! { <span class="px-2 py-1 bg-gray-100 text-gray-800 rounded text-xs font-mono">{codename.to_string()}</span> }
                                            } else {
                                                html! { <span class="text-gray-400">{"—"}</span> }
                                            }
                                        }
                                    </p>
                                </div>
                                <div class="space-y-2">
                                    <label class="text-sm font-medium text-gray-700">{"Publish State"}</label>
                                    <p class="text-sm">{publish_badge}</p>
                                </div>
                                <div class="space-y-2">
                                    <label class="text-sm font-medium text-gray-700">{"Status"}</label>
                                    <p class="text-sm">{status_badge}</p>
                                </div>
                                <div class="space-y-2">
                                    <label class="text-sm font-medium text-gray-700">{"Owner"}</label>
                                    <p class="text-sm font-mono text-xs break-all">{persona.owner.to_string()}</p>
                                </div>
                                <div class="space-y-2">
                                    <label class="text-sm font-medium text-gray-700">{"Updated"}</label>
                                    <p class="text-sm">{updated_display}</p>
                                </div>
                            </div>
                        </div>

                        // Voice Settings
                        <div class="space-y-4">
                            <h3 class="text-lg font-semibold">{"Voice Settings"}</h3>
                            <div class="grid grid-cols-2 gap-4">
                                <div class="space-y-2">
                                    <label class="text-sm font-medium text-gray-700">{"Voice ID"}</label>
                                    <p class="text-sm font-mono break-all">
                                        {
                                            if !persona.voice_id.is_empty() {
                                                html! { <span>{&persona.voice_id}</span> }
                                            } else {
                                                html! { <span class="text-gray-400">{"—"}</span> }
                                            }
                                        }
                                    </p>
                                </div>
                                <div class="space-y-2">
                                    <label class="text-sm font-medium text-gray-700">{"Model ID"}</label>
                                    <p class="text-sm font-mono break-all">
                                        {
                                            if let Some(model_id) = &persona.model_id {
                                                html! { <span>{model_id}</span> }
                                            } else {
                                                html! { <span class="text-gray-400">{"—"}</span> }
                                            }
                                        }
                                    </p>
                                </div>
                                <div class="space-y-2">
                                    <label class="text-sm font-medium text-gray-700">{"Stability"}</label>
                                    <p class="text-sm">{format!("{:.2}", persona.stability)}</p>
                                </div>
                                <div class="space-y-2">
                                    <label class="text-sm font-medium text-gray-700">{"Similarity Boost"}</label>
                                    <p class="text-sm">{format!("{:.2}", persona.similarity_boost)}</p>
                                </div>
                                <div class="space-y-2">
                                    <label class="text-sm font-medium text-gray-700">{"Style"}</label>
                                    <p class="text-sm">
                                        {
                                            if let Some(style) = persona.style {
                                                format!("{:.2}", style)
                                            } else {
                                                "—".to_string()
                                            }
                                        }
                                    </p>
                                </div>
                                <div class="space-y-2">
                                    <label class="text-sm font-medium text-gray-700">{"Speaker Boost"}</label>
                                    <p class="text-sm">
                                        {
                                            if let Some(use_speaker_boost) = persona.use_speaker_boost {
                                                if use_speaker_boost {
                                                    html! { <span class="px-2 py-1 bg-blue-100 text-blue-800 rounded text-xs font-medium">{"Enabled"}</span> }
                                                } else {
                                                    html! { <span class="px-2 py-1 bg-gray-100 text-gray-800 rounded text-xs font-medium">{"Disabled"}</span> }
                                                }
                                            } else {
                                                html! { <span class="px-2 py-1 bg-gray-100 text-gray-800 rounded text-xs font-medium">{"Disabled"}</span> }
                                            }
                                        }
                                    </p>
                                </div>
                            </div>
                        </div>

                        // Template
                        <div class="space-y-4">
                            <h3 class="text-lg font-semibold">{"Template"}</h3>
                            <div class="rounded-md bg-gray-50 p-4">
                                <p class="text-sm whitespace-pre-wrap font-mono">
                                    {
                                        if !persona.template.is_empty() {
                                            html! { <span>{&persona.template}</span> }
                                        } else {
                                            html! { <span class="text-gray-400">{"No template set"}</span> }
                                        }
                                    }
                                </p>
                            </div>
                        </div>

                        // Media Assets
                        <div class="space-y-4">
                            <h3 class="text-lg font-semibold">{"Media Assets"}</h3>
                            <div class="flex gap-4 flex-wrap">
                                {
                                    if !persona.image.is_empty() {
                                        html! { <span class="px-2 py-1 bg-gray-100 text-gray-800 rounded text-xs border border-gray-300">{"Image Available"}</span> }
                                    } else {
                                        html! {}
                                    }
                                }
                                {
                                    if !persona.thumbnail.is_empty() {
                                        html! { <span class="px-2 py-1 bg-gray-100 text-gray-800 rounded text-xs border border-gray-300">{"Thumbnail Available"}</span> }
                                    } else {
                                        html! {}
                                    }
                                }
                                {
                                    if !persona.sample_audio.is_empty() {
                                        html! { <span class="px-2 py-1 bg-gray-100 text-gray-800 rounded text-xs border border-gray-300">{"Audio Sample Available"}</span> }
                                    } else {
                                        html! {}
                                    }
                                }
                                {
                                    if persona.image.is_empty() && persona.thumbnail.is_empty() && persona.sample_audio.is_empty() {
                                        html! { <span class="text-sm text-gray-400">{"No media assets"}</span> }
                                    } else {
                                        html! {}
                                    }
                                }
                            </div>
                        </div>

                        // Identification
                        <div class="space-y-4">
                            <h3 class="text-lg font-semibold">{"Identification"}</h3>
                            <div class="space-y-2">
                                <label class="text-sm font-medium text-gray-700">{"Persona ID"}</label>
                                <p class="text-sm font-mono text-gray-600 break-all">{persona.id.to_string()}</p>
                            </div>
                        </div>
                    </div>

                    // Footer
                    <div class="flex justify-end p-6 border-t bg-gray-50">
                        <button
                            onclick={props.on_close.reform(|_| ())}
                            class="px-4 py-2 border border-gray-300 rounded-lg text-gray-700 hover:bg-gray-50 transition-colors"
                        >
                            {"Close"}
                        </button>
                    </div>
                </div>
            </div>
        }
    } else {
        html! {}
    }
}
