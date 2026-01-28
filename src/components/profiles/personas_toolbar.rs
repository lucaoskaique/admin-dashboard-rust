use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PersonasToolbarProps {
    pub search_value: String,
    pub on_search_change: Callback<String>,
    pub selected_count: usize,
    pub on_clear_selection: Callback<()>,
    pub total_count: usize,
}

#[function_component(PersonasToolbar)]
pub fn personas_toolbar(props: &PersonasToolbarProps) -> Html {
    let on_input = {
        let on_search_change = props.on_search_change.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            on_search_change.emit(input.value());
        })
    };

    html! {
        <div class="flex items-center justify-between py-4">
            <div class="flex flex-1 items-center space-x-2">
                // Search input
                <div class="flex items-center space-x-2">
                    <input
                        type="text"
                        placeholder="Search personas..."
                        class="px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 w-64"
                        value={props.search_value.clone()}
                        oninput={on_input}
                    />
                </div>

                // Selected items indicator
                if props.selected_count > 0 {
                    <div class="flex items-center space-x-2">
                        <span class="text-sm text-gray-600">
                            {format!("{} of {} row(s) selected", props.selected_count, props.total_count)}
                        </span>
                        <button
                            onclick={props.on_clear_selection.reform(|_| ())}
                            class="text-sm text-blue-600 hover:text-blue-700"
                        >
                            {"Clear"}
                        </button>
                    </div>
                }
            </div>

            <div class="flex items-center space-x-2">
                <span class="text-sm text-gray-600">
                    {format!("{} persona(s)", props.total_count)}
                </span>
            </div>
        </div>
    }
}
