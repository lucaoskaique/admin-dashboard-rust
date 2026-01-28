use crate::utils::FlowNode;
use gloo_console;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InfoPanelProps {
    pub node: FlowNode,
    pub on_close: Callback<()>,
}

#[function_component(InfoPanel)]
pub fn info_panel(props: &InfoPanelProps) -> Html {
    gloo_console::log!("InfoPanel rendering for node:", &props.node.id);

    html! {
        <div class="w-80 bg-white rounded-lg shadow-xl border border-gray-200 max-h-[calc(100vh-200px)] overflow-hidden flex flex-col">
            // Header
            <div class="flex items-center justify-between p-4 border-b border-gray-200 bg-gradient-to-r from-indigo-50 to-purple-50">
                <h3 class="font-semibold text-gray-900">{"Node Details"}</h3>
                <button
                    onclick={props.on_close.reform(|_| ())}
                    class="text-gray-500 hover:text-gray-700 transition-colors"
                    title="Close"
                >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>
            </div>

            // Content
            <div class="overflow-y-auto flex-1">
                <div class="p-4 space-y-4">
                    // Node ID
                    <div>
                        <label class="text-xs font-semibold text-gray-500 uppercase tracking-wide">{"Node ID"}</label>
                        <div class="mt-1 text-sm text-gray-900 font-mono bg-gray-50 px-3 py-2 rounded border border-gray-200">
                            {&props.node.id}
                        </div>
                    </div>

                    // Label
                    <div>
                        <label class="text-xs font-semibold text-gray-500 uppercase tracking-wide">{"Label"}</label>
                        <div class="mt-1 text-sm text-gray-900 font-medium">
                            {&props.node.label}
                        </div>
                    </div>

                    // Component Type
                    <div>
                        <label class="text-xs font-semibold text-gray-500 uppercase tracking-wide">{"Component Type"}</label>
                        <div class="mt-1">
                            <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-indigo-100 text-indigo-800">
                                {&props.node.component_type}
                            </span>
                        </div>
                    </div>

                    // Visual Properties
                    <div>
                        <label class="text-xs font-semibold text-gray-500 uppercase tracking-wide">{"Visual Properties"}</label>
                        <div class="mt-2 space-y-2">
                            <div class="flex items-center gap-2">
                                <span class="text-xs text-gray-500 w-16">{"Icon:"}</span>
                                <span class="text-lg">{&props.node.icon}</span>
                            </div>
                            <div class="flex items-center gap-2">
                                <span class="text-xs text-gray-500 w-16">{"Color:"}</span>
                                <div class="flex items-center gap-2">
                                    <div
                                        class="w-6 h-6 rounded border border-gray-300"
                                        style={format!("background-color: {}", props.node.color)}
                                    />
                                    <span class="text-xs font-mono text-gray-600">{&props.node.color}</span>
                                </div>
                            </div>
                            <div class="flex items-center gap-2">
                                <span class="text-xs text-gray-500 w-16">{"Size:"}</span>
                                <span class="text-xs text-gray-600">
                                    {format!("{}×{} px", props.node.width as i32, props.node.height as i32)}
                                </span>
                            </div>
                        </div>
                    </div>

                    // Position
                    <div>
                        <label class="text-xs font-semibold text-gray-500 uppercase tracking-wide">{"Position"}</label>
                        <div class="mt-1 grid grid-cols-2 gap-2">
                            <div class="bg-gray-50 px-3 py-2 rounded border border-gray-200">
                                <div class="text-xs text-gray-500">{"X"}</div>
                                <div class="text-sm font-mono text-gray-900">{format!("{:.0}", props.node.x)}</div>
                            </div>
                            <div class="bg-gray-50 px-3 py-2 rounded border border-gray-200">
                                <div class="text-xs text-gray-500">{"Y"}</div>
                                <div class="text-sm font-mono text-gray-900">{format!("{:.0}", props.node.y)}</div>
                            </div>
                        </div>
                    </div>

                    // Additional Info
                    <div class="pt-4 border-t border-gray-200">
                        <div class="flex items-start gap-2 text-xs text-gray-500">
                            <svg class="w-4 h-4 mt-0.5 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                            </svg>
                            <span>{"Click anywhere outside this panel or press ESC to close"}</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
