use crate::api::PublishState;
use angel_api_client_reqwest::types::personas::{Persona, PersonaId};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PersonaTableRowProps {
    pub persona: Persona,
    pub selected: bool,
    pub on_toggle_select: Callback<PersonaId>,
    pub on_view: Callback<PersonaId>,
    pub on_edit: Callback<PersonaId>,
    pub on_delete: Callback<PersonaId>,
}

#[function_component(PersonaTableRow)]
pub fn persona_table_row(props: &PersonaTableRowProps) -> Html {
    let persona = &props.persona;
    let persona_id = persona.id;

    let on_checkbox_change = {
        let on_toggle_select = props.on_toggle_select.clone();
        Callback::from(move |_| {
            on_toggle_select.emit(persona_id);
        })
    };

    let on_view_click = {
        let on_view = props.on_view.clone();
        Callback::from(move |_| {
            on_view.emit(persona_id);
        })
    };

    let on_edit_click = {
        let on_edit = props.on_edit.clone();
        Callback::from(move |_| {
            on_edit.emit(persona_id);
        })
    };

    let on_delete_click = {
        let on_delete = props.on_delete.clone();
        Callback::from(move |_| {
            on_delete.emit(persona_id);
        })
    };

    let publish_badge = match persona.visibility {
        PublishState::Public => html! {
            <span class="px-2 py-1 bg-green-100 text-green-800 rounded-full text-xs font-medium">
                {"Public"}
            </span>
        },
        PublishState::Private => html! {
            <span class="px-2 py-1 bg-gray-100 text-gray-800 rounded-full text-xs font-medium">
                {"Private"}
            </span>
        },
        PublishState::Unlisted => html! {
            <span class="px-2 py-1 bg-yellow-100 text-yellow-800 rounded-full text-xs font-medium">
                {"Unlisted"}
            </span>
        },
    };

    let updated_display = if let Some(updated) = &persona.updated_at {
        // Format as "Oct 7, 2025" or similar readable format
        format!("{}", updated.date())
    } else {
        "—".to_string()
    };

    html! {
        <tr class="border-b hover:bg-gray-50">
            // Checkbox
            <td class="py-3 px-4">
                <input
                    type="checkbox"
                    checked={props.selected}
                    onchange={on_checkbox_change}
                    class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                />
            </td>

            // Name
            <td class="py-3 px-4 font-medium">
                {&persona.name}
            </td>

            // Codename
            <td class="py-3 px-4">
                {
                    if let Some(codename) = &persona.codename {
                        html! {
                            <span class="font-mono text-xs bg-gray-100 px-2 py-1 rounded">
                                {codename.to_string()}
                            </span>
                        }
                    } else {
                        html! { <span class="text-gray-400">{"—"}</span> }
                    }
                }
            </td>

            // Publish State
            <td class="py-3 px-4">
                {publish_badge}
            </td>

            // Updated
            <td class="py-3 px-4 text-sm text-gray-600">
                {updated_display}
            </td>

            // Status
            <td class="py-3 px-4">
                {
                    if persona.deleted_at.is_some() {
                        html! {
                            <span class="px-2 py-1 bg-red-100 text-red-800 rounded-full text-xs font-medium">
                                {"Deleted"}
                            </span>
                        }
                    } else {
                        html! {
                            <span class="px-2 py-1 bg-green-100 text-green-800 rounded-full text-xs font-medium">
                                {"Active"}
                            </span>
                        }
                    }
                }
            </td>

            // Actions
            <td class="py-3 px-4">
                <div class="flex items-center gap-2">
                    <button
                        onclick={on_view_click}
                        class="text-blue-600 hover:text-blue-700 text-sm font-medium"
                    >
                        {"View"}
                    </button>
                    <button
                        onclick={on_edit_click}
                        class="text-gray-600 hover:text-gray-700 text-sm font-medium"
                    >
                        {"Edit"}
                    </button>
                    <button
                        onclick={on_delete_click}
                        class="text-red-600 hover:text-red-700 text-sm font-medium"
                    >
                        {"Delete"}
                    </button>
                </div>
            </td>
        </tr>
    }
}
