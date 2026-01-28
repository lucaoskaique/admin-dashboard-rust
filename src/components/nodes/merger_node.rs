use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct MergerNodeProps {
    pub label: String,
    pub merger_type: String, // StreamMerger, StreamMerger3-9, InterleaveLongest2-5, etc.
    #[prop_or_default]
    pub description: Option<String>,
    #[prop_or(0)]
    pub input_count: usize,
    #[prop_or(0)]
    pub output_count: usize,
}

#[function_component(MergerNode)]
pub fn merger_node(props: &MergerNodeProps) -> Html {
    // Use actual input count from flow data, fallback to parsing name if zero
    let stream_count = if props.input_count > 0 {
        props.input_count
    } else {
        props
            .merger_type
            .chars()
            .last()
            .and_then(|c| c.to_digit(10))
            .unwrap_or(2) as usize
    };

    let icon = if props.merger_type.starts_with("Interleave") {
        "🔀"
    } else {
        "🔗"
    };

    html! {
        <div class="relative min-w-[200px] max-w-[280px] rounded-xl shadow-lg hover:shadow-xl transition-all duration-200 hover:-translate-y-0.5 overflow-hidden bg-gradient-to-br from-cyan-500 to-cyan-600 border-2 border-cyan-700">
            <div class="flex items-center p-3 gap-3">
                <div class="text-2xl flex-shrink-0">{icon}</div>
                <div class="flex flex-col gap-0.5 flex-1 min-w-0">
                    <span class="font-semibold text-sm text-white truncate">{&props.merger_type}</span>
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
                <div class="flex items-center gap-1 text-[10px]">{format!("← ×{}", stream_count)}</div>
                <div class="flex items-center gap-1">{"→"}</div>
            </div>
        </div>
    }
}
