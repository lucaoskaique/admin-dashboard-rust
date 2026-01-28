use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct GateNodeProps {
    pub label: String,
    pub gate_type: String, // FloodGate1, FloodGate2, FloodGate3, FloodGate4, FloodRouter
    #[prop_or_default]
    pub description: Option<String>,
}

#[function_component(GateNode)]
pub fn gate_node(props: &GateNodeProps) -> Html {
    let stream_count = if props.gate_type.contains("FloodGate") {
        props
            .gate_type
            .chars()
            .last()
            .and_then(|c| c.to_digit(10))
            .unwrap_or(1)
    } else {
        2 // FloodRouter
    };

    html! {
        <div class="relative min-w-[200px] max-w-[280px] rounded-xl shadow-lg hover:shadow-xl transition-all duration-200 hover:-translate-y-0.5 overflow-hidden bg-gradient-to-br from-red-500 to-red-600 border-2 border-red-700">
            <div class="flex items-center p-3 gap-3">
                <div class="text-2xl flex-shrink-0">{"🚦"}</div>
                <div class="flex flex-col gap-0.5 flex-1 min-w-0">
                    <span class="font-semibold text-sm text-white truncate">{&props.gate_type}</span>
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
                <div class="flex items-center gap-1 text-[10px]">{format!("← ×{}", stream_count + 2)}</div>
                <div class="flex items-center gap-1 text-[10px]">{format!("→ ×{}", stream_count)}</div>
            </div>
            <div class="absolute top-2 right-2 px-2 py-0.5 rounded-full text-[10px] font-semibold uppercase tracking-wide bg-gradient-to-r from-red-300 to-red-400 text-white">{"Conditional"}</div>
        </div>
    }
}
