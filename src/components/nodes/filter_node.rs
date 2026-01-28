use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct FilterNodeProps {
    pub label: String,
    #[prop_or_default]
    pub filter_type: Option<String>,
    #[prop_or_default]
    pub description: Option<String>,
}

#[function_component(FilterNode)]
pub fn filter_node(props: &FilterNodeProps) -> Html {
    let filter_type = props.filter_type.as_deref().unwrap_or("Filter");
    
    html! {
        <div class="relative min-w-[200px] max-w-[280px] rounded-xl shadow-lg hover:shadow-xl transition-all duration-200 hover:-translate-y-0.5 overflow-hidden bg-gradient-to-br from-teal-500 to-teal-600 border-2 border-teal-700">
            <div class="flex items-center p-3 gap-3">
                <div class="text-2xl flex-shrink-0">{"🔍"}</div>
                <div class="flex flex-col gap-0.5 flex-1 min-w-0">
                    <span class="font-semibold text-sm text-white truncate">{filter_type}</span>
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
                <div class="flex items-center gap-1 text-[10px]">{"← ×2"}</div>
                <div class="flex items-center gap-1">{"→"}</div>
            </div>
        </div>
    }
}
