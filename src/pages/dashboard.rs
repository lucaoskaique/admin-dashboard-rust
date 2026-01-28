use yew::prelude::*;
use crate::components::StatCard;

#[function_component(DashboardPage)]
pub fn dashboard_page() -> Html {
    html! {
        <div class="p-8">
            <div class="mb-8">
                <h2 class="text-3xl font-bold tracking-tight">{"Dashboard"}</h2>
                <p class="text-gray-600 mt-2">
                    {"Welcome to the admin panel. Manage your AI personas and system settings."}
                </p>
            </div>

            // Stats Grid
            <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4 mb-8">
                <StatCard
                    title="Total Personas"
                    value="24"
                    change="+2 this week"
                    positive={true}
                />
                <StatCard
                    title="Total Users"
                    value="1,284"
                    change="+12%"
                    positive={true}
                />
                <StatCard
                    title="Active Sessions"
                    value="48"
                    change="+8%"
                    positive={true}
                />
                <StatCard
                    title="Response Time"
                    value="1.2s"
                    change="-5%"
                    positive={true}
                />
            </div>

            <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-7">
                // Recent Activity
                <div class="col-span-4 rounded-lg border bg-white shadow-sm">
                    <div class="p-6">
                        <h3 class="text-lg font-semibold">{"Recent Activity"}</h3>
                    </div>
                    <div class="p-6 pt-0">
                        <div class="space-y-8">
                            <div class="flex items-center">
                                <div class="space-y-1">
                                    <p class="text-sm font-medium leading-none">{"New persona created"}</p>
                                    <p class="text-sm text-gray-500">{"Math Tutor persona was created by admin@angel.ai"}</p>
                                </div>
                                <div class="ml-auto font-medium text-gray-500 text-sm">{"2 hours ago"}</div>
                            </div>
                            <div class="flex items-center">
                                <div class="space-y-1">
                                    <p class="text-sm font-medium leading-none">{"User registered"}</p>
                                    <p class="text-sm text-gray-500">{"john.doe@example.com joined the platform"}</p>
                                </div>
                                <div class="ml-auto font-medium text-gray-500 text-sm">{"5 hours ago"}</div>
                            </div>
                            <div class="flex items-center">
                                <div class="space-y-1">
                                    <p class="text-sm font-medium leading-none">{"Settings updated"}</p>
                                    <p class="text-sm text-gray-500">{"System settings were modified"}</p>
                                </div>
                                <div class="ml-auto font-medium text-gray-500 text-sm">{"1 day ago"}</div>
                            </div>
                        </div>
                    </div>
                </div>

                // Quick Actions
                <div class="col-span-3 rounded-lg border bg-white shadow-sm">
                    <div class="p-6">
                        <h3 class="text-lg font-semibold">{"Quick Actions"}</h3>
                    </div>
                    <div class="p-6 pt-0">
                        <div class="space-y-4">
                            <button class="w-full text-left px-4 py-3 rounded-lg border border-gray-200 hover:bg-gray-50 transition-colors">
                                <div class="font-medium">{"Create New Persona"}</div>
                                <div class="text-sm text-gray-500">{"Add a new AI persona"}</div>
                            </button>
                            <button class="w-full text-left px-4 py-3 rounded-lg border border-gray-200 hover:bg-gray-50 transition-colors">
                                <div class="font-medium">{"Invite User"}</div>
                                <div class="text-sm text-gray-500">{"Add a new user to the system"}</div>
                            </button>
                            <button class="w-full text-left px-4 py-3 rounded-lg border border-gray-200 hover:bg-gray-50 transition-colors">
                                <div class="font-medium">{"View Reports"}</div>
                                <div class="text-sm text-gray-500">{"Check system analytics"}</div>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
