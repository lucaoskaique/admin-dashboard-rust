use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct StatCardProps {
    pub title: String,
    pub value: String,
    pub change: String,
    #[prop_or(false)]
    pub positive: bool,
}

#[function_component(StatCard)]
pub fn stat_card(props: &StatCardProps) -> Html {
    let change_color = if props.positive {
        "text-green-600"
    } else {
        "text-red-600"
    };
    
    html! {
        <div class="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
            <div class="flex items-center justify-between">
                <div>
                    <p class="text-sm font-medium text-gray-600">{ &props.title }</p>
                    <p class="text-2xl font-bold text-gray-900 mt-2">{ &props.value }</p>
                </div>
            </div>
            <div class="mt-4">
                <span class={format!("text-sm font-medium {}", change_color)}>
                    { &props.change }
                </span>
                <span class="text-sm text-gray-500 ml-2">{ "vs last month" }</span>
            </div>
        </div>
    }
}
