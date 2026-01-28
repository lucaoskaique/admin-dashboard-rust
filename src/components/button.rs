use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Ghost,
    Danger,
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or(ButtonVariant::Primary)]
    pub variant: ButtonVariant,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub r#type: Option<String>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let base_classes = "px-4 py-2 rounded-lg font-medium transition-colors duration-200 disabled:opacity-50 disabled:cursor-not-allowed";

    let variant_classes = match props.variant {
        ButtonVariant::Primary => "bg-blue-600 hover:bg-blue-700 text-white",
        ButtonVariant::Ghost => "hover:bg-gray-100 text-gray-700",
        ButtonVariant::Danger => "bg-red-600 hover:bg-red-700 text-white",
    };

    let classes = format!("{} {} {}", base_classes, variant_classes, props.class);

    html! {
        <button
            type={props.r#type.clone().unwrap_or_else(|| "button".to_string())}
            class={classes}
            onclick={props.onclick.clone()}
            disabled={props.disabled}
        >
            { for props.children.iter() }
        </button>
    }
}
