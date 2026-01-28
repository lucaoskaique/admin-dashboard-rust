use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ComparisonNodeProps {
    pub label: String,
    pub comparison_type: String, // All, Any
    #[prop_or_default]
    pub description: Option<String>,
}

#[function_component(ComparisonNode)]
pub fn comparison_node(props: &ComparisonNodeProps) -> Html {
    html! {
        <div class="relative min-w-[200px] max-w-[280px] rounded-xl shadow-lg hover:shadow-xl transition-all duration-200 hover:-translate-y-0.5 overflow-hidden bg-gradient-to-br from-lime-500 to-lime-600 border-2 border-lime-700">
            <div class="flex items-center p-3 gap-3">
                <div class="text-2xl flex-shrink-0">{"🧮"}</div>
                <div class="flex flex-col gap-0.5 flex-1 min-w-0">
                    <span class="font-semibold text-sm text-white truncate">{&props.comparison_type}</span>
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
                <div class="flex items-center gap-1 text-[10px]">{"← ×3"}</div>
                <div class="flex items-center gap-1">{"→"}</div>
            </div>
            <div class="absolute top-2 right-2 px-2 py-0.5 rounded-full text-[10px] font-semibold uppercase tracking-wide bg-gradient-to-r from-lime-300 to-lime-400 text-lime-900">{"Boolean"}</div>
        </div>
    }
}
