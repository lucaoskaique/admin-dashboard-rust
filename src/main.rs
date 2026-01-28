use gloo_console::log;
use wasm_bindgen_futures::spawn_local;
use web_sys::SubmitEvent;
use yew::prelude::*;
use yew_router::prelude::*;

mod api;
mod components;
mod hooks;
mod layouts;
mod pages;
mod utils;

use api::login;
use components::Input;
use hooks::{use_auth, AuthProvider};
use layouts::MainLayout;
use pages::{DashboardPage, WorkflowsPage, ProfilesPage, SettingsPage, UsersPage};

// TODO: Replace with your API base URL
const API_BASE_URL: &str = "http://localhost:8080"\;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/users")]
    Users,
    #[at("/profiles")]
    Profiles,
    #[at("/workflows")]
    Workflows,
    #[at("/settings")]
    Settings,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, PartialEq)]
enum LoginState {
    Idle,
    Loading,
    Error(String),
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <AuthProvider>
            <BrowserRouter>
                <AppContent />
            </BrowserRouter>
        </AuthProvider>
    }
}

#[function_component(AppContent)]
fn app_content() -> Html {
    let auth = use_auth();

    if auth.user.is_none() {
        html! { <LoginPage /> }
    } else {
        html! {
            <MainLayout>
                <Switch<Route> render={switch} />
            </MainLayout>
        }
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <DashboardPage /> },
        Route::Users => html! { <UsersPage /> },
        Route::Profiles => html! { <ProfilesPage /> },
        Route::Workflows => html! { <WorkflowsPage /> },
        Route::Settings => html! { <SettingsPage /> },
        Route::NotFound => html! { <h1>{ "404 Not Found" }</h1> },
    }
}

#[function_component(LoginPage)]
fn login_page() -> Html {
    let auth = use_auth();
    let email = use_state(|| String::new());
    let password = use_state(|| String::new());
    let login_state = use_state(|| LoginState::Idle);

    let on_submit = {
        let email = email.clone();
        let password = password.clone();
        let login_state = login_state.clone();
        let auth_login = auth.login.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let email = (*email).clone();
            let password = (*password).clone();
            let login_state = login_state.clone();
            let auth_login = auth_login.clone();

            spawn_local(async move {
                login_state.set(LoginState::Loading);

                match login(API_BASE_URL, &email, &password).await {
                    Ok(user) => {
                        log!("Login successful!");
                        auth_login.emit(user);
                        login_state.set(LoginState::Idle);
                    }
                    Err(err) => {
                        log!("Login failed:", &err);
                        login_state.set(LoginState::Error(err));
                    }
                }
            });
        })
    };

    let on_email_input = {
        let email = email.clone();
        Callback::from(move |value: String| {
            email.set(value);
        })
    };

    let on_password_input = {
        let password = password.clone();
        Callback::from(move |value: String| {
            password.set(value);
        })
    };

    html! {
        <div class="min-h-screen bg-gradient-to-br from-indigo-600 via-purple-600 to-pink-500 flex items-center justify-center p-4">
            <div class="bg-white rounded-2xl shadow-2xl w-full max-w-md p-8 space-y-6">
                <div class="text-center">
                    <h1 class="text-3xl font-bold text-gray-900 mb-2">{"Workflow Dashboard"}</h1>
                    <p class="text-gray-600">{"Sign in to your account"}</p>
                </div>

                <form {onsubmit: on_submit} class="space-y-4">
                    <Input
                        label="Email"
                        input_type="email"
                        placeholder="you@example.com"
                        value={(*email).clone()}
                        on_input={on_email_input}
                        required={true}
                    />

                    <Input
                        label="Password"
                        input_type="password"
                        placeholder="Enter your password"
                        value={(*password).clone()}
                        on_input={on_password_input}
                        required={true}
                    />

                    <button
                        type="submit"
                        class="w-full bg-gradient-to-r from-indigo-600 to-purple-600 text-white py-3 px-4 rounded-lg font-medium hover:from-indigo-700 hover:to-purple-700 transition-all duration-200 shadow-lg hover:shadow-xl disabled:opacity-50 disabled:cursor-not-allowed"
                        disabled={matches!(*login_state, LoginState::Loading)}
                    >
                        {
                            match &*login_state {
                                LoginState::Loading => "Signing in...",
                                _ => "Sign in"
                            }
                        }
                    </button>

                    {
                        match &*login_state {
                            LoginState::Error(err) => html! {
                                <div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-lg">
                                    <p class="text-sm">{err}</p>
                                </div>
                            },
                            _ => html! {}
                        }
                    }
                </form>

                <div class="pt-4 border-t border-gray-200">
                    <p class="text-xs text-center text-gray-500">
                        {"Demo mode: Any credentials will work"}
                    </p>
                </div>
            </div>
        </div>
    }
}
