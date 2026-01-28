use yew::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct InputProps {
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub onchange: Callback<String>,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub label: Option<String>,
    #[prop_or(AttrValue::from("text"))]
    pub r#type: AttrValue,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: String,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let onchange = props.onchange.clone();
    let oninput = Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        onchange.emit(input.value());
    });
    
    let input_classes = format!(
        "w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:bg-gray-100 {}",
        props.class
    );
    
    html! {
        <div class="w-full">
            if let Some(label) = &props.label {
                <label class="block text-sm font-medium text-gray-700 mb-1">
                    { label }
                </label>
            }
            <input
                type={props.r#type.clone()}
                class={input_classes}
                value={props.value.clone()}
                oninput={oninput}
                placeholder={props.placeholder.clone()}
                disabled={props.disabled}
            />
        </div>
    }
}
