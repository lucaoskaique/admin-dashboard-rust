use yew::prelude::*;
use crate::components::{Button, Card, CardContent, CardHeader};

#[function_component(UsersPage)]
pub fn users_page() -> Html {
    html! {
        <div class="p-8">
            <div class="mb-8 flex justify-between items-center">
                <div>
                    <h2 class="text-3xl font-bold tracking-tight">{"Users"}</h2>
                    <p class="text-gray-600 mt-2">{"Manage system users and their permissions"}</p>
                </div>
                <Button variant={crate::components::ButtonVariant::Primary}>
                    {"Add User"}
                </Button>
            </div>

            <Card>
                <CardHeader>
                    <h3 class="text-lg font-semibold">{"All Users"}</h3>
                </CardHeader>
                <CardContent>
                    <div class="overflow-x-auto">
                        <table class="w-full">
                            <thead>
                                <tr class="border-b">
                                    <th class="text-left py-3 px-4 font-medium text-gray-700">{"Name"}</th>
                                    <th class="text-left py-3 px-4 font-medium text-gray-700">{"Email"}</th>
                                    <th class="text-left py-3 px-4 font-medium text-gray-700">{"Role"}</th>
                                    <th class="text-left py-3 px-4 font-medium text-gray-700">{"Status"}</th>
                                    <th class="text-left py-3 px-4 font-medium text-gray-700">{"Actions"}</th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr class="border-b hover:bg-gray-50">
                                    <td class="py-3 px-4">{"John Doe"}</td>
                                    <td class="py-3 px-4">{"john.doe@example.com"}</td>
                                    <td class="py-3 px-4">
                                        <span class="px-2 py-1 bg-blue-100 text-blue-800 rounded-full text-xs font-medium">
                                            {"Admin"}
                                        </span>
                                    </td>
                                    <td class="py-3 px-4">
                                        <span class="px-2 py-1 bg-green-100 text-green-800 rounded-full text-xs font-medium">
                                            {"Active"}
                                        </span>
                                    </td>
                                    <td class="py-3 px-4">
                                        <Button variant={crate::components::ButtonVariant::Ghost} class="text-sm">
                                            {"Edit"}
                                        </Button>
                                    </td>
                                </tr>
                                <tr class="border-b hover:bg-gray-50">
                                    <td class="py-3 px-4">{"Jane Smith"}</td>
                                    <td class="py-3 px-4">{"jane.smith@example.com"}</td>
                                    <td class="py-3 px-4">
                                        <span class="px-2 py-1 bg-gray-100 text-gray-800 rounded-full text-xs font-medium">
                                            {"User"}
                                        </span>
                                    </td>
                                    <td class="py-3 px-4">
                                        <span class="px-2 py-1 bg-green-100 text-green-800 rounded-full text-xs font-medium">
                                            {"Active"}
                                        </span>
                                    </td>
                                    <td class="py-3 px-4">
                                        <Button variant={crate::components::ButtonVariant::Ghost} class="text-sm">
                                            {"Edit"}
                                        </Button>
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </CardContent>
            </Card>
        </div>
    }
}
