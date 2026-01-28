use yew::prelude::*;
use crate::components::{Card, CardContent, CardHeader, Input, Button};

#[function_component(SettingsPage)]
pub fn settings_page() -> Html {
    let api_endpoint = use_state(|| "https://dev-api.angelq.ai".to_string());
    let max_sessions = use_state(|| "100".to_string());
    
    html! {
        <div class="p-8">
            <div class="mb-8">
                <h2 class="text-3xl font-bold tracking-tight">{"Settings"}</h2>
                <p class="text-gray-600 mt-2">{"Configure system settings and preferences"}</p>
            </div>

            <div class="max-w-3xl space-y-6">
                <Card>
                    <CardHeader>
                        <h3 class="text-lg font-semibold">{"API Configuration"}</h3>
                    </CardHeader>
                    <CardContent class="space-y-4">
                        <Input
                            label="API Endpoint"
                            value={(*api_endpoint).clone()}
                            onchange={Callback::from(move |v: String| api_endpoint.set(v))}
                            placeholder="Enter API endpoint"
                        />
                        <Button variant={crate::components::ButtonVariant::Primary}>
                            {"Save Changes"}
                        </Button>
                    </CardContent>
                </Card>

                <Card>
                    <CardHeader>
                        <h3 class="text-lg font-semibold">{"Session Management"}</h3>
                    </CardHeader>
                    <CardContent class="space-y-4">
                        <Input
                            label="Max Concurrent Sessions"
                            value={(*max_sessions).clone()}
                            onchange={Callback::from(move |v: String| max_sessions.set(v))}
                            placeholder="Enter maximum sessions"
                            r#type="number"
                        />
                        <Button variant={crate::components::ButtonVariant::Primary}>
                            {"Save Changes"}
                        </Button>
                    </CardContent>
                </Card>

                <Card>
                    <CardHeader>
                        <h3 class="text-lg font-semibold">{"Danger Zone"}</h3>
                    </CardHeader>
                    <CardContent>
                        <div class="space-y-4">
                            <div>
                                <h4 class="font-medium text-gray-900 mb-2">{"Clear All Cache"}</h4>
                                <p class="text-sm text-gray-600 mb-4">
                                    {"This will clear all cached data. This action cannot be undone."}
                                </p>
                                <Button variant={crate::components::ButtonVariant::Danger}>
                                    {"Clear Cache"}
                                </Button>
                            </div>
                        </div>
                    </CardContent>
                </Card>
            </div>
        </div>
    }
}
