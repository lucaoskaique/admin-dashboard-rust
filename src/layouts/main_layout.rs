use crate::{components::Sidebar, hooks::use_auth};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MainLayoutProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(MainLayout)]
pub fn main_layout(props: &MainLayoutProps) -> Html {
    let auth = use_auth();
    let collapsed = use_state(|| false);

    let user_email = auth
        .user
        .as_ref()
        .map(|u| u.email.clone())
        .unwrap_or_else(|| "Guest".to_string());

    let on_logout = auth.logout.reform(|_| ());

    let toggle_collapsed = {
        let collapsed = collapsed.clone();
        Callback::from(move |_| {
            collapsed.set(!*collapsed);
        })
    };

    html! {
        <div class="flex h-screen bg-gray-50">
            <Sidebar
                user_email={user_email}
                on_logout={on_logout}
                collapsed={*collapsed}
                on_toggle={toggle_collapsed}
            />
            <main class={classes!(
                "flex-1", "overflow-y-auto", "transition-all", "duration-300",
                if *collapsed { "ml-16" } else { "ml-64" },
                "md:ml-0", "md:pl-16", if !*collapsed { "md:pl-64" } else { "" }
            )}>
                { for props.children.iter() }
            </main>
        </div>
    }
}
