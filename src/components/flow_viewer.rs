use crate::components::nodes::*;
use crate::components::{InfoPanel, ZoomControls};
use crate::utils::{apply_elk_layout, flow_to_graph, FlowGraph, FlowNode};
use angel_api_client_reqwest::types::flows::Flow;
use gloo_console;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{MouseEvent, WheelEvent};
use yew::prelude::*;

// JavaScript interop for D3.js rendering edges only
#[wasm_bindgen(module = "/js/d3-flow-renderer.js")]
extern "C" {
    #[wasm_bindgen(js_name = renderEdges)]
    fn render_edges(container_id: &str, nodes_json: &str, edges_json: &str);

    #[wasm_bindgen(js_name = clearFlow)]
    fn clear_flow(container_id: &str);
}

// Helper function to render the appropriate node component based on type
fn render_node_component(node: &FlowNode, on_click: &Callback<FlowNode>) -> Html {
    let style = format!(
        "position: absolute; left: {}px; top: {}px; width: {}px; height: {}px; pointer-events: auto;",
        node.x, node.y, node.width, node.height
    );
    let node_clone = node.clone();
    let on_node_click = {
        let on_click = on_click.clone();
        Callback::from(move |e: MouseEvent| {
            gloo_console::log!("Node clicked:", &node_clone.id);
            e.stop_propagation();
            on_click.emit(node_clone.clone());
        })
    };

    let component_type = node.component_type.as_str();

    html! {
        <div {style} onclick={on_node_click} class="cursor-pointer">
            {match component_type {
                "Input" => html! {
                    <InputNode label={node.label.clone()} />
                },
                "Output" => html! {
                    <OutputNode label={node.label.clone()} />
                },
                "LLM" | "LLM_nonstreaming" | "LLMVision" | "StructuredLLM" => html! {
                    <LlmNode label={node.label.clone()} model_type={component_type.to_string()} />
                },
                "JinjaTemplate" | "Template" => html! {
                    <TemplateNode label={node.label.clone()} template_type={Some(component_type.to_string())} />
                },
                "JmesPathConverter" => html! {
                    <ConverterNode label={node.label.clone()} converter_type={Some(component_type.to_string())} />
                },
                "Filter" | "FilterMotnByAngelRating" | "FilterReelgoodByAngelRating" |
                "FilterVideosByAvailability" | "FilterVideosByScore" | "ImageResolutionFilter" => html! {
                    <FilterNode label={node.label.clone()} filter_type={Some(component_type.to_string())} />
                },
                "Echo" | "EchoLines" | "Collect" | "Flatten" | "FlattenStream" |
                "Chain" | "Split" | "Take" | "Skip" | "StreamCollect" => html! {
                    <TransformNode label={node.label.clone()} transform_type={component_type.to_string()} />
                },
                "BingLegacy" | "BingDirect" | "BraveSearchImage" => html! {
                    <SearchNode label={node.label.clone()} search_type={component_type.to_string()} />
                },
                "ElevenLabs" | "ElevenLabsWithConfig" | "ElevenLabsHttps" | "_ElevenLabsLegacy" => html! {
                    <AudioNode label={node.label.clone()} audio_type={Some(component_type.to_string())} />
                },
                "FloodGate1" | "FloodGate2" | "FloodGate3" | "FloodGate4" | "FloodRouter" => html! {
                    <GateNode label={node.label.clone()} gate_type={component_type.to_string()} />
                },
                "JsonObjectConstructor" | "JsonObjectConstructorStreamData" | "JsonObjectExtender" |
                "JsonObjectMerger" | "JsonPathExtractor" => html! {
                    <JsonNode label={node.label.clone()} json_type={component_type.to_string()} />
                },
                "StreamMerger" | "StreamMerger3" | "StreamMerger4" | "StreamMerger5" |
                "StreamMerger6" | "StreamMerger7" | "StreamMerger8" | "StreamMerger9" |
                "InterleaveLongest2" | "InterleaveLongest3" | "InterleaveLongest4" | "InterleaveLongest5" => html! {
                    <MergerNode
                        label={node.label.clone()}
                        merger_type={component_type.to_string()}
                        input_count={node.input_count}
                        output_count={node.output_count}
                    />
                },
                "All" | "Any" => html! {
                    <ComparisonNode label={node.label.clone()} comparison_type={component_type.to_string()} />
                },
                "URLScraper" | "URLScraperList" | "URLScraperListLined" | "URLScraperListLinedStreaming" |
                "WolframAlpha" | "YouDotCom" | "YouDotComNews" | "Perplexity" => html! {
                    <WebNode label={node.label.clone()} web_type={component_type.to_string()} />
                },
                "ReelgoodShowSearchTitle" | "ReelgoodSearchParamsAdapter" | "ReelgoodToVideoMetadata" |
                "MovieOfTheNightToVideoMetadata" | "YouTubeKidsLegacy" | "TmdbSeriesThumbnails" |
                "PartitionVideosByType" | "ShowSearchTitle" => html! {
                    <VideoNode label={node.label.clone()} video_type={component_type.to_string()} />
                },
                "Pair" | "PairAndMergeDicts" | "TripleAndMergeDicts" | "Zip" |
                "Repeat" | "IsEmpty" => html! {
                    <UtilityNode label={node.label.clone()} utility_type={component_type.to_string()} />
                },
                "BubbleMachine" => html! {
                    <AudioNode label={node.label.clone()} audio_type={Some("BubbleMachine".to_string())} />
                },
                "TextMerger" => html! {
                    <TransformNode label={node.label.clone()} transform_type={"TextMerger".to_string()} />
                },
                _ => html! {
                    // Default/unknown component type
                    <UtilityNode label={node.label.clone()} utility_type={component_type.to_string()} />
                }
            }}
        </div>
    }
}

#[derive(Properties, Clone)]
pub struct FlowViewerProps {
    pub flow: Flow,
}

// Custom PartialEq implementation
impl PartialEq for FlowViewerProps {
    fn eq(&self, other: &Self) -> bool {
        // Compare flows by serializing to JSON
        serde_json::to_string(&self.flow).ok() == serde_json::to_string(&other.flow).ok()
    }
}

#[function_component(FlowViewer)]
pub fn flow_viewer(props: &FlowViewerProps) -> Html {
    let loading = use_state(|| true);
    let flow_name = props
        .flow
        .name
        .as_ref()
        .map(|n| n.to_string())
        .unwrap_or_else(|| "Unnamed Flow".to_string());

    // State for zoom and pan
    let zoom_level = use_state(|| 1.0);
    let pan_x = use_state(|| 0.0);
    let pan_y = use_state(|| 0.0);
    let is_panning = use_state(|| false);
    let last_mouse_x = use_state(|| 0.0);
    let last_mouse_y = use_state(|| 0.0);

    // State for selected node (info panel)
    let selected_node = use_state(|| None::<FlowNode>);

    // Check if flow has no components
    if props.flow.components.is_empty() {
        return html! {
            <div class="flow-viewer h-full bg-gray-50 rounded-lg overflow-hidden flex flex-col">
                <div class="p-4 border-b border-gray-200 bg-white">
                    <h2 class="text-xl font-semibold text-gray-900">{flow_name}</h2>
                    <p class="text-sm text-gray-500 mt-1">{"Empty flow"}</p>
                </div>

                <div class="flex-1 flex items-center justify-center">
                    <div class="text-center p-8">
                        <svg class="w-16 h-16 text-gray-300 mx-auto mb-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4" />
                        </svg>
                        <p class="text-gray-500 text-lg">{"This flow has no components"}</p>
                        <p class="text-gray-400 text-sm mt-2">{"Components will appear here when added"}</p>
                    </div>
                </div>
            </div>
        };
    }

    // State to hold the laid out graph
    let graph_data = use_state(|| None::<FlowGraph>);

    // Effect 1: Run ELK layout when component mounts
    {
        let flow = props.flow.clone();
        let loading = loading.clone();
        let graph_data = graph_data.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                // Step 1: Convert Flow to intermediate graph structure
                let graph = flow_to_graph(&flow);
                gloo_console::log!(
                    "Converted flow to graph:",
                    format!("{} nodes, {} edges", graph.nodes.len(), graph.edges.len())
                );

                // Step 2: Apply ELK layout to compute positions
                match apply_elk_layout(graph).await {
                    Ok(laid_out_graph) => {
                        gloo_console::log!("ELK layout complete");
                        graph_data.set(Some(laid_out_graph));
                        loading.set(false);
                    }
                    Err(e) => {
                        gloo_console::error!("ELK layout error:", e);
                        loading.set(false);
                    }
                }
            });

            || {}
        });
    }

    // Effect 2: Render edges with D3 when graph data is ready
    {
        let graph_data = graph_data.clone();
        let loading = loading.clone();

        use_effect_with(
            ((*loading), (*graph_data).clone()),
            move |(is_loading, data)| {
                gloo_console::log!("=== D3 Edge Rendering Effect ===");
                gloo_console::log!("Loading:", is_loading.to_string());
                gloo_console::log!("Has graph data:", data.is_some().to_string());

                if !is_loading {
                    if let Some(graph) = data {
                        gloo_console::log!("Graph nodes count:", graph.nodes.len());
                        gloo_console::log!("Graph edges count:", graph.edges.len());

                        let nodes_json = serde_json::to_string(&graph.nodes).unwrap();
                        let edges_json = serde_json::to_string(&graph.edges).unwrap();

                        gloo_console::log!("Nodes JSON:", &nodes_json);
                        gloo_console::log!("Edges JSON:", &edges_json);

                        wasm_bindgen_futures::spawn_local(async move {
                            gloo_timers::future::TimeoutFuture::new(10).await;
                            gloo_console::log!("Calling render_edges...");
                            render_edges("flow-svg-container", &nodes_json, &edges_json);
                        });
                    }
                }

                || {
                    clear_flow("flow-svg-container");
                }
            },
        );
    }

    // Zoom controls callbacks
    let on_zoom_in = {
        let zoom_level = zoom_level.clone();
        Callback::from(move |_| {
            let new_zoom = (*zoom_level * 1.2_f64).min(4.0);
            zoom_level.set(new_zoom);
        })
    };

    let on_zoom_out = {
        let zoom_level = zoom_level.clone();
        Callback::from(move |_| {
            let new_zoom = (*zoom_level / 1.2_f64).max(0.1);
            zoom_level.set(new_zoom);
        })
    };

    let on_reset_view = {
        let zoom_level = zoom_level.clone();
        let pan_x = pan_x.clone();
        let pan_y = pan_y.clone();
        Callback::from(move |_| {
            zoom_level.set(1.0);
            pan_x.set(0.0);
            pan_y.set(0.0);
        })
    };

    // Mouse wheel for zoom
    let on_wheel = {
        let zoom_level = zoom_level.clone();
        Callback::from(move |e: WheelEvent| {
            e.prevent_default();
            let delta = if e.delta_y() < 0.0 { 1.1_f64 } else { 0.9_f64 };
            let new_zoom = (*zoom_level * delta).clamp(0.1, 4.0);
            zoom_level.set(new_zoom);
        })
    };

    // Pan controls - only pan when clicking on background, not nodes
    let on_background_mouse_down = {
        let is_panning = is_panning.clone();
        let last_mouse_x = last_mouse_x.clone();
        let last_mouse_y = last_mouse_y.clone();
        Callback::from(move |e: MouseEvent| {
            // Only start panning if clicking directly on the background
            if e.button() == 0 {
                is_panning.set(true);
                last_mouse_x.set(e.client_x() as f64);
                last_mouse_y.set(e.client_y() as f64);
            }
        })
    };

    let on_mouse_move = {
        let is_panning = is_panning.clone();
        let pan_x = pan_x.clone();
        let pan_y = pan_y.clone();
        let last_mouse_x = last_mouse_x.clone();
        let last_mouse_y = last_mouse_y.clone();
        let zoom_level = zoom_level.clone();
        Callback::from(move |e: MouseEvent| {
            if *is_panning {
                let dx = e.client_x() as f64 - *last_mouse_x;
                let dy = e.client_y() as f64 - *last_mouse_y;
                pan_x.set(*pan_x + dx / *zoom_level);
                pan_y.set(*pan_y + dy / *zoom_level);
                last_mouse_x.set(e.client_x() as f64);
                last_mouse_y.set(e.client_y() as f64);
                e.prevent_default();
            }
        })
    };

    let on_mouse_up = {
        let is_panning = is_panning.clone();
        Callback::from(move |_: MouseEvent| {
            is_panning.set(false);
        })
    };

    // Node click handler
    let on_node_click = {
        let selected_node = selected_node.clone();
        Callback::from(move |node: FlowNode| {
            gloo_console::log!("on_node_click callback fired for:", &node.id);
            gloo_console::log!("Setting selected_node state");
            selected_node.set(Some(node));
        })
    };

    // Close info panel
    let on_close_info = {
        let selected_node = selected_node.clone();
        Callback::from(move |_| {
            selected_node.set(None);
        })
    };

    // Debug logging for selected node state
    gloo_console::log!(
        "Rendering FlowViewer, selected_node:",
        (*selected_node)
            .as_ref()
            .map(|n| n.id.as_str())
            .unwrap_or("None")
    );

    html! {
        <div class="flow-viewer h-full bg-gray-50 rounded-lg overflow-hidden flex flex-col">
            <div class="p-4 border-b border-gray-200 bg-white">
                <h2 class="text-xl font-semibold text-gray-900">{flow_name}</h2>
                <p class="text-sm text-gray-500 mt-1">
                    {if *loading {
                        "Computing layout...".to_string()
                    } else {
                        format!("{} components", props.flow.components.len())
                    }}
                </p>
            </div>

            {if *loading {
                html! {
                    <div class="flex-1 flex items-center justify-center">
                        <div class="text-center p-8">
                            <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500 mx-auto"></div>
                            <p class="text-gray-500 mt-4">{"Calculating optimal layout with ELK"}</p>
                        </div>
                    </div>
                }
            } else if let Some(graph) = (*graph_data).clone() {
                // Calculate container dimensions from nodes
                let min_width = graph.nodes.iter()
                    .map(|n| n.x + n.width)
                    .fold(0.0, f64::max) + 100.0;
                let min_height = graph.nodes.iter()
                    .map(|n| n.y + n.height)
                    .fold(0.0, f64::max) + 100.0;

                html! {
                    <div
                        class="flex-1 overflow-auto relative bg-gray-100"
                        onwheel={on_wheel}
                        onmousedown={on_background_mouse_down}
                        onmousemove={on_mouse_move}
                        onmouseup={on_mouse_up.clone()}
                        onmouseleave={on_mouse_up}
                    >
                        // Zoom Controls
                        <div class="absolute top-4 right-4 z-30">
                            <ZoomControls
                                zoom_level={*zoom_level}
                                on_zoom_in={on_zoom_in}
                                on_zoom_out={on_zoom_out}
                                on_reset={on_reset_view}
                            />
                        </div>

                        // Node Info Panel
                        if let Some(node) = (*selected_node).clone() {
                            <div class="absolute top-4 left-4 z-30">
                                <InfoPanel
                                    node={node}
                                    on_close={on_close_info}
                                />
                            </div>
                        }

                        // Content with zoom/pan transform
                        <div
                            class="relative pointer-events-none"
                            style={format!(
                                "min-width: {}px; min-height: {}px; transform: translate({}px, {}px) scale({}); transform-origin: 0 0;",
                                min_width, min_height, *pan_x, *pan_y, *zoom_level
                            )}
                        >
                            // Nodes container (Yew components)
                            {for graph.nodes.iter().map(|node| {
                                render_node_component(node, &on_node_click)
                            })}

                            // SVG container - SAME size as nodes container, not inset-0
                            <div
                                id="flow-svg-container"
                                class="absolute"
                                style={format!("top: 0; left: 0; width: {}px; height: {}px; pointer-events: none; z-index: 10;", min_width, min_height)}
                            ></div>
                        </div>
                    </div>
                }
            } else {
                html! {
                    <div class="flex-1 flex items-center justify-center">
                        <p class="text-gray-500">{"No graph data available"}</p>
                    </div>
                }
            }}
        </div>
    }
}
