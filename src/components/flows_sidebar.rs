use angel_api_client_reqwest::types::flows::{FlowId, FlowMetadata};
use yew::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Properties, Clone, PartialEq)]
pub struct FlowsSidebarProps {
    pub flows: Vec<FlowMetadata>,
    pub loading: bool,
    pub error: Option<String>,
    pub selected_flow_id: Option<FlowId>,
    pub on_flow_select: Callback<FlowId>,
    pub search_query: String,
    pub on_search_change: Callback<String>,
}

#[function_component(FlowsSidebar)]
pub fn flows_sidebar(props: &FlowsSidebarProps) -> Html {
    // Filter flows based on search query
    let filtered_flows: Vec<FlowMetadata> = if props.search_query.is_empty() {
        props.flows.clone()
    } else {
        let query = props.search_query.to_lowercase();
        props.flows.iter()
            .filter(|flow| {
                // Match by name
                let name_match = flow.name.as_ref()
                    .map(|name| name.to_lowercase().contains(&query))
                    .unwrap_or(false);
                
                // Match by flow ID
                let id_match = flow.id.to_string().to_lowercase().contains(&query);
                
                name_match || id_match
            })
            .cloned()
            .collect()
    };

    let on_input = {
        let on_search_change = props.on_search_change.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            on_search_change.emit(input.value());
        })
    };

    html! {
        <div class="w-80 bg-white border-r border-gray-200 overflow-y-auto">
            <div class="p-6 border-b border-gray-200">
                <h1 class="text-2xl font-bold text-gray-900">{"Flows"}</h1>
                <p class="text-sm text-gray-500 mt-1">{"Select a flow to view"}</p>
            </div>

            // Search bar
            <div class="p-4 border-b border-gray-200">
                <div class="relative">
                    <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                        <svg class="h-5 w-5 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                        </svg>
                    </div>
                    <input
                        type="text"
                        class="block w-full pl-10 pr-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
                        placeholder="Search by name or ID..."
                        value={props.search_query.clone()}
                        oninput={on_input}
                    />
                </div>
            </div>

            <div class="p-4">
                if props.loading {
                    <div class="flex items-center justify-center py-8">
                        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-indigo-600"></div>
                    </div>
                } else if let Some(err) = props.error.clone() {
                    <div class="bg-red-50 border border-red-200 rounded-lg p-4">
                        <p class="text-sm text-red-800">{err}</p>
                    </div>
                } else if filtered_flows.is_empty() {
                    <div class="text-center py-8">
                        <p class="text-gray-500">
                            {if props.search_query.is_empty() {
                                "No flows found"
                            } else {
                                "No matching flows"
                            }}
                        </p>
                    </div>
                } else {
                    <div class="space-y-2">
                        {for filtered_flows.iter().map(|flow_metadata| {
                            let flow_id = flow_metadata.id;
                            let is_selected = props.selected_flow_id == Some(flow_id);
                            let on_click = {
                                let on_flow_select = props.on_flow_select.clone();
                                Callback::from(move |_| on_flow_select.emit(flow_id))
                            };

                            html! {
                                <button
                                    onclick={on_click}
                                    class={classes!(
                                        "w-full", "text-left", "p-4", "rounded-lg",
                                        "transition-colors", "border",
                                        if is_selected {
                                            "bg-indigo-50 border-indigo-200"
                                        } else {
                                            "bg-white border-gray-200 hover:bg-gray-50"
                                        }
                                    )}
                                >
                                    <div class="flex items-start justify-between">
                                        <div class="flex-1 min-w-0">
                                            <h3 class="font-medium text-gray-900 truncate">
                                                {flow_metadata.name.clone().unwrap_or_else(|| "Untitled Flow".to_string())}
                                            </h3>
                                            <p class="text-xs text-gray-500 mt-1">
                                                {"ID: "}{flow_metadata.id.to_string()}
                                            </p>
                                            <p class="text-xs text-gray-400 mt-1">
                                                {"Created: "}{flow_metadata.created_at.to_string()}
                                            </p>
                                        </div>
                                        if is_selected {
                                            <svg class="w-5 h-5 text-indigo-600 flex-shrink-0 ml-2" fill="currentColor" viewBox="0 0 20 20">
                                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
                                            </svg>
                                        }
                                    </div>
                                </button>
                            }
                        })}
                    </div>
                }
            </div>
        </div>
    }
}
