use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ZoomControlsProps {
    pub zoom_level: f64,
    pub on_zoom_in: Callback<()>,
    pub on_zoom_out: Callback<()>,
    pub on_reset: Callback<()>,
}

#[function_component(ZoomControls)]
pub fn zoom_controls(props: &ZoomControlsProps) -> Html {
    let zoom_percent = (props.zoom_level * 100.0) as i32;

    html! {
        <div class="absolute bottom-6 right-6 flex flex-col gap-2 z-10">
            <div class="bg-white rounded-lg shadow-lg border border-gray-200 overflow-hidden">
                <button
                    onclick={props.on_zoom_in.reform(|_| ())}
                    disabled={props.zoom_level >= 4.0}
                    class="w-10 h-10 flex items-center justify-center text-gray-700 hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed transition-colors border-b border-gray-200"
                    title="Zoom In"
                >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
                    </svg>
                </button>

                <div class="px-2 py-1 text-xs font-semibold text-gray-600 text-center border-b border-gray-200 bg-gray-50">
                    {format!("{}%", zoom_percent)}
                </div>

                <button
                    onclick={props.on_zoom_out.reform(|_| ())}
                    disabled={props.zoom_level <= 0.1}
                    class="w-10 h-10 flex items-center justify-center text-gray-700 hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed transition-colors border-b border-gray-200"
                    title="Zoom Out"
                >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 12H4" />
                    </svg>
                </button>

                <button
                    onclick={props.on_reset.reform(|_| ())}
                    class="w-10 h-10 flex items-center justify-center text-gray-700 hover:bg-gray-50 transition-colors"
                    title="Reset View (Fit to Screen)"
                >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 8V4m0 0h4M4 4l5 5m11-1V4m0 0h-4m4 0l-5 5M4 16v4m0 0h4m-4 0l5-5m11 5l-5-5m5 5v-4m0 4h-4" />
                    </svg>
                </button>
            </div>
        </div>
    }
}
