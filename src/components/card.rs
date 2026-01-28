use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let classes = format!(
        "bg-white rounded-lg shadow-sm border border-gray-200 {}",
        props.class
    );

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CardHeaderProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

#[function_component(CardHeader)]
pub fn card_header(props: &CardHeaderProps) -> Html {
    let classes = format!("px-6 py-4 border-b border-gray-200 {}", props.class);

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CardContentProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

#[function_component(CardContent)]
pub fn card_content(props: &CardContentProps) -> Html {
    let classes = format!("px-6 py-4 {}", props.class);

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
