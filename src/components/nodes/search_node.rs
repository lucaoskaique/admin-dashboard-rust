use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct SearchNodeProps {
    pub label: String,
    pub search_type: String, // BingDirect, BingLegacy, BraveSearchImage
    #[prop_or_default]
    pub description: Option<String>,
}

#[function_component(SearchNode)]
pub fn search_node(props: &SearchNodeProps) -> Html {
    let icon = if props.search_type.contains("Image") || props.search_type.contains("Brave") {
        "🖼️"
    } else {
        "🔎"
    };

    html! {
        <div class="relative min-w-[200px] max-w-[280px] rounded-xl shadow-lg hover:shadow-xl transition-all duration-200 hover:-translate-y-0.5 overflow-hidden bg-gradient-to-br from-cyan-500 to-cyan-600 border-2 border-cyan-700">
            <div class="flex items-center p-3 gap-3">
                <div class="text-2xl flex-shrink-0">{icon}</div>
                <div class="flex flex-col gap-0.5 flex-1 min-w-0">
                    <span class="font-semibold text-sm text-white truncate">{&props.search_type}</span>
                    <span class="text-xs text-white/80 font-mono">{format!("#{}", props.label)}</span>
                </div>
            </div>
            {if let Some(desc) = &props.description {
                html! {
                    <div class="px-4 pb-3 text-xs text-white/90 leading-tight border-t border-white/10 pt-2 mx-3">{desc}</div>
                }
            } else {
                html! {}
            }}
            <div class="flex justify-between px-4 py-2 bg-black/10 text-xs text-white/70 font-medium">
                <div class="flex items-center gap-1">{"←"}</div>
                <div class="flex items-center gap-1 text-[10px]">{"→ ×2"}</div>
            </div>
            <div class="absolute top-2 right-2 px-2 py-0.5 rounded-full text-[10px] font-semibold uppercase tracking-wide bg-gradient-to-r from-cyan-300 to-cyan-400 text-cyan-900">{"Search"}</div>
        </div>
    }
}
