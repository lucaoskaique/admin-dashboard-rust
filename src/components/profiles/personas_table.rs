use super::{PersonaTableRow, PersonasToolbar};
use angel_api_client_reqwest::types::personas::{Persona, PersonaId};
use std::collections::HashSet;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PersonasTableProps {
    pub personas: Vec<Persona>,
    pub on_view: Callback<PersonaId>,
    pub on_edit: Callback<PersonaId>,
    pub on_delete: Callback<PersonaId>,
}

#[function_component(PersonasTable)]
pub fn personas_table(props: &PersonasTableProps) -> Html {
    let selected_rows = use_state(HashSet::<PersonaId>::new);
    let search_term = use_state(String::new);
    let sort_field = use_state(|| "name");
    let sort_direction = use_state(|| "asc");

    // Filter personas based on search term
    let filtered_personas: Vec<Persona> = props
        .personas
        .iter()
        .filter(|p| {
            let search = search_term.to_lowercase();
            p.name.to_lowercase().contains(&search)
                || p.codename
                    .as_ref()
                    .map_or(false, |c| c.to_string().to_lowercase().contains(&search))
                || p.id.to_string().to_lowercase().contains(&search)
        })
        .cloned()
        .collect();

    // Sort personas
    let mut sorted_personas = filtered_personas.clone();
    sorted_personas.sort_by(|a, b| {
        let field = *sort_field;
        let direction = *sort_direction;

        let ordering = match field {
            "name" => a.name.cmp(&b.name),
            "codename" => match (&a.codename, &b.codename) {
                (Some(ac), Some(bc)) => ac.to_string().cmp(&bc.to_string()),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            },
            _ => a.name.cmp(&b.name),
        };

        if direction == "desc" {
            ordering.reverse()
        } else {
            ordering
        }
    });

    let on_search_change = {
        let search_term = search_term.clone();
        Callback::from(move |value: String| {
            search_term.set(value);
        })
    };

    let on_clear_selection = {
        let selected_rows = selected_rows.clone();
        Callback::from(move |_| {
            selected_rows.set(HashSet::new());
        })
    };

    let on_toggle_select = {
        let selected_rows = selected_rows.clone();
        Callback::from(move |id: PersonaId| {
            let mut new_selection = (*selected_rows).clone();
            if new_selection.contains(&id) {
                new_selection.remove(&id);
            } else {
                new_selection.insert(id);
            }
            selected_rows.set(new_selection);
        })
    };

    let on_toggle_all = {
        let selected_rows = selected_rows.clone();
        let personas = sorted_personas.clone();
        Callback::from(move |_| {
            if selected_rows.len() == personas.len() {
                selected_rows.set(HashSet::new());
            } else {
                selected_rows.set(personas.iter().map(|p| p.id).collect());
            }
        })
    };

    let on_view = {
        let callback = props.on_view.clone();
        Callback::from(move |id: PersonaId| {
            callback.emit(id);
        })
    };

    let on_edit = {
        let callback = props.on_edit.clone();
        Callback::from(move |id: PersonaId| {
            callback.emit(id);
        })
    };

    let all_selected = !sorted_personas.is_empty() && selected_rows.len() == sorted_personas.len();

    html! {
        <div class="space-y-4">
            <PersonasToolbar
                search_value={(*search_term).clone()}
                on_search_change={on_search_change}
                selected_count={selected_rows.len()}
                on_clear_selection={on_clear_selection}
                total_count={sorted_personas.len()}
            />

            <div class="rounded-lg border border-gray-200 overflow-hidden">
                <div class="overflow-x-auto">
                    <table class="w-full">
                        <thead class="bg-gray-50 border-b border-gray-200">
                            <tr>
                                <th class="py-3 px-4 text-left">
                                    <input
                                        type="checkbox"
                                        checked={all_selected}
                                        onchange={on_toggle_all}
                                        class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                                    />
                                </th>
                                <th class="py-3 px-4 text-left font-semibold text-sm text-gray-700">
                                    {"Name"}
                                </th>
                                <th class="py-3 px-4 text-left font-semibold text-sm text-gray-700">
                                    {"Codename"}
                                </th>
                                <th class="py-3 px-4 text-left font-semibold text-sm text-gray-700">
                                    {"Publish State"}
                                </th>
                                <th class="py-3 px-4 text-left font-semibold text-sm text-gray-700">
                                    {"Updated"}
                                </th>
                                <th class="py-3 px-4 text-left font-semibold text-sm text-gray-700">
                                    {"Status"}
                                </th>
                                <th class="py-3 px-4 text-left font-semibold text-sm text-gray-700">
                                    {"Actions"}
                                </th>
                            </tr>
                        </thead>
                        <tbody class="bg-white divide-y divide-gray-200">
                            {
                                if sorted_personas.is_empty() {
                                    html! {
                                        <tr>
                                            <td colspan="8" class="py-8 text-center text-gray-500">
                                                {"No personas found"}
                                            </td>
                                        </tr>
                                    }
                                } else {
                                    sorted_personas.iter().map(|persona| {
                                        let is_selected = selected_rows.contains(&persona.id);
                                        html! {
                                            <PersonaTableRow
                                                key={persona.id.to_string()}
                                                persona={persona.clone()}
                                                selected={is_selected}
                                                on_toggle_select={on_toggle_select.clone()}
                                                on_view={on_view.clone()}
                                                on_edit={on_edit.clone()}
                                                on_delete={props.on_delete.clone()}
                                            />
                                        }
                                    }).collect::<Html>()
                                }
                            }
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    }
}
