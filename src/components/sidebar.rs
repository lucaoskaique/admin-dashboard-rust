use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq)]
pub struct NavItem {
    pub title: &'static str,
    pub route: Route,
    pub icon: &'static str,
    pub items: Vec<SubNavItem>,
}

#[derive(Clone, PartialEq)]
pub struct SubNavItem {
    pub title: &'static str,
    pub route: Route,
}

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub user_email: String,
    pub on_logout: Callback<()>,
    pub collapsed: bool,
    pub on_toggle: Callback<()>,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let nav_items = vec![
        NavItem {
            title: "Dashboard",
            route: Route::Home,
            icon: "📊",
            items: vec![],
        },
        NavItem {
            title: "Users",
            route: Route::Users,
            icon: "👥",
            items: vec![],
        },
        NavItem {
            title: "Personas",
            route: Route::Personas,
            icon: "🤖",
            items: vec![],
        },
        NavItem {
            title: "Flows",
            route: Route::Flows,
            icon: "🔄",
            items: vec![],
        },
        NavItem {
            title: "Settings",
            route: Route::Settings,
            icon: "⚙️",
            items: vec![],
        },
    ];

    html! {
        <>
            <aside class={classes!(
                "fixed", "left-0", "top-0", "z-40", "h-screen", "transition-all", "duration-300",
                "bg-white", "border-r", "border-gray-200", "flex", "flex-col",
                if props.collapsed { "w-16" } else { "w-64" }
            )}>
                // Header
                <div class="h-16 flex items-center px-4 border-b border-gray-200 flex-shrink-0">
                    if !props.collapsed {
                        <div class="flex items-center gap-3">
                            <div class="w-8 h-8 rounded-lg bg-gradient-to-br from-indigo-500 to-purple-600 flex items-center justify-center text-white font-bold text-sm">
                                {"A"}
                            </div>
                            <div class="flex flex-col">
                                <span class="text-sm font-semibold text-gray-900">{"Angel Admin"}</span>
                                <span class="text-xs text-gray-500">{"Admin Dashboard"}</span>
                            </div>
                        </div>
                    } else {
                        <div class="w-10 h-10 rounded-lg bg-gradient-to-br from-indigo-500 to-purple-600 flex items-center justify-center text-white font-bold text-base">
                            {"A"}
                        </div>
                    }
                </div>

            // Navigation
            <nav class="flex-1 overflow-y-auto p-4 space-y-1">
                <div class="mb-6">
                    <p class={classes!(
                        "text-xs", "font-semibold", "text-gray-500", "uppercase", "tracking-wider", "mb-3",
                        if props.collapsed { "hidden" } else { "block" }
                    )}>
                        {"General"}
                    </p>
                    {for nav_items.iter().map(|item| {
                        html! {
                            <Link<Route>
                                to={item.route.clone()}
                                classes={classes!(
                                    "flex", "items-center", "gap-3", "px-3", "py-2", "rounded-lg",
                                    "transition-colors", "group", "relative",
                                    if props.collapsed { "justify-center" } else { "" },
                                    "text-gray-700", "hover:bg-gray-100"
                                )}
                            >
                                <span class={classes!(
                                    if props.collapsed { "text-base" } else { "text-lg" }
                                )}>{item.icon}</span>
                                if !props.collapsed {
                                    <span class="flex-1 text-sm font-medium">{item.title}</span>
                                }
                            </Link<Route>>
                        }
                    })}
                </div>
            </nav>

            // Footer - User Profile
            <div class="border-t border-gray-200 p-4 flex-shrink-0">
                <div class={classes!(
                    "flex", "items-center", "gap-3",
                    if props.collapsed { "justify-center" } else { "" }
                )}>
                    <div class={classes!(
                        "rounded-full", "bg-gradient-to-br", "from-indigo-500", "to-purple-600",
                        "flex", "items-center", "justify-center", "text-white", "font-semibold",
                        if props.collapsed { "w-8 h-8 text-xs" } else { "w-8 h-8 text-sm" }
                    )}>
                        {props.user_email.chars().next().unwrap_or('U').to_uppercase().to_string()}
                    </div>
                    if !props.collapsed {
                        <div class="flex-1 min-w-0">
                            <p class="text-sm font-medium text-gray-900 truncate">{&props.user_email}</p>
                            <p class="text-xs text-gray-500">{"Admin"}</p>
                        </div>
                        <button
                            onclick={props.on_logout.reform(|_| ())}
                            class="p-1.5 rounded-md hover:bg-gray-100 transition-colors"
                            title="Sign out"
                        >
                            <svg class="w-4 h-4 text-gray-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
                            </svg>
                        </button>
                    }
                </div>
            </div>
        </aside>

        // Toggle Button - Outside sidebar
        <button
            onclick={props.on_toggle.reform(|_| ())}
            class={classes!(
                "fixed", "z-50", "top-4", "p-2", "rounded-full",
                "bg-white", "border", "border-gray-200", "shadow-md",
                "hover:bg-gray-50", "transition-all", "duration-300",
                if props.collapsed { "left-14" } else { "left-60" }
            )}
            title={if props.collapsed { "Expand sidebar" } else { "Collapse sidebar" }}
        >
            <svg class="w-4 h-4 text-gray-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                {if props.collapsed {
                    html! {
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                    }
                } else {
                    html! {
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                    }
                }}
            </svg>
        </button>
        </>
    }
}
