use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct WebNodeProps {
    pub label: String,
    pub web_type: String, // URLScraper, URLScraperList, WolframAlpha, YouDotCom, Perplexity
    #[prop_or_default]
    pub description: Option<String>,
}

#[function_component(WebNode)]
pub fn web_node(props: &WebNodeProps) -> Html {
    let icon = if props.web_type.contains("Wolfram") {
        "🔬"
    } else if props.web_type.contains("Scraper") {
        "🕷️"
    } else if props.web_type.contains("Perplexity") {
        "🤔"
    } else {
        "🌐"
    };
    
    html! {
        <div class="relative min-w-[200px] max-w-[280px] rounded-xl shadow-lg hover:shadow-xl transition-all duration-200 hover:-translate-y-0.5 overflow-hidden bg-gradient-to-br from-emerald-500 to-emerald-600 border-2 border-emerald-700">
            <div class="flex items-center p-3 gap-3">
                <div class="text-2xl flex-shrink-0">{icon}</div>
                <div class="flex flex-col gap-0.5 flex-1 min-w-0">
                    <span class="font-semibold text-sm text-white truncate">{&props.web_type}</span>
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
                <div class="flex items-center gap-1">{"→"}</div>
            </div>
            <div class="absolute top-2 right-2 px-2 py-0.5 rounded-full text-[10px] font-semibold uppercase tracking-wide bg-gradient-to-r from-emerald-300 to-emerald-400 text-emerald-900">{"Web"}</div>
        </div>
    }
}
