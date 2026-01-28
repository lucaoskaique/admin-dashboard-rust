use crate::api::{get_flow, get_flows};
use crate::components::{FlowViewer, FlowsSidebar};
use angel_api_client_reqwest::types::flows::{Flow, FlowId, FlowMetadata};
use gloo_console::log;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(FlowsPage)]
pub fn flows_page() -> Html {
    let flows = use_state(|| Vec::<FlowMetadata>::new());
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let selected_flow_id = use_state(|| None::<FlowId>);
    let selected_flow = use_state(|| None::<Flow>);
    let loading_flow = use_state(|| false);
    let search_query = use_state(|| String::new());

    const API_BASE_URL: &str = "https://dev-api.angelq.ai";

    // Fetch flows on mount
    {
        let flows = flows.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                log!("Fetching flows...");
                match get_flows(API_BASE_URL).await {
                    Ok(mut data) => {
                        log!("Successfully fetched flows:", data.len());
                        // Sort by created_at descending (newest first)
                        data.sort_by(|a, b| b.created_at.cmp(&a.created_at));
                        flows.set(data);
                        loading.set(false);
                    }
                    Err(e) => {
                        log!("Error fetching flows:", e.clone());
                        error.set(Some(e));
                        loading.set(false);
                    }
                }
            });
            || ()
        });
    }

    // Fetch flow when selected
    {
        let selected_flow_id = selected_flow_id.clone();
        let selected_flow = selected_flow.clone();
        let loading_flow = loading_flow.clone();

        use_effect_with((*selected_flow_id).clone(), move |flow_id| {
            if let Some(id) = *flow_id {
                loading_flow.set(true);
                spawn_local(async move {
                    log!("Fetching flow details:", id.to_string());
                    match get_flow(API_BASE_URL, id).await {
                        Ok(flow) => {
                            log!("Successfully fetched flow details");
                            selected_flow.set(Some(flow));
                            loading_flow.set(false);
                        }
                        Err(e) => {
                            log!("Error fetching flow:", e);
                            loading_flow.set(false);
                        }
                    }
                });
            } else {
                selected_flow.set(None);
            }
            || ()
        });
    }

    let on_flow_click = {
        let selected_flow_id = selected_flow_id.clone();
        Callback::from(move |flow_id: FlowId| {
            selected_flow_id.set(Some(flow_id));
        })
    };

    let on_search_change = {
        let search_query = search_query.clone();
        Callback::from(move |query: String| {
            search_query.set(query);
        })
    };

    html! {
        <div class="flex h-full bg-gray-50">
            // Sidebar with flows list
            <FlowsSidebar
                flows={(*flows).clone()}
                loading={*loading}
                error={(*error).clone()}
                selected_flow_id={*selected_flow_id}
                on_flow_select={on_flow_click}
                search_query={(*search_query).clone()}
                on_search_change={on_search_change}
            />

            // Main viewer area
            <div class="flex-1 overflow-hidden">
                if *loading_flow {
                    <div class="flex items-center justify-center h-full">
                        <div class="text-center">
                            <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-indigo-600 mx-auto mb-4"></div>
                            <p class="text-gray-500">{"Loading flow..."}</p>
                        </div>
                    </div>
                } else if let Some(flow) = (*selected_flow).clone() {
                    <div class="h-full">
                        <FlowViewer flow={flow} />
                    </div>
                } else {
                    <div class="flex items-center justify-center h-full">
                        <div class="text-center">
                            <svg class="w-16 h-16 text-gray-300 mx-auto mb-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                            </svg>
                            <p class="text-gray-500">{"Select a flow to view details"}</p>
                        </div>
                    </div>
                }
            </div>
        </div>
    }
}
