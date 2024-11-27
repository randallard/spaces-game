use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/tailwind.css"/>
        <Router>
            <main>
                <Routes>
                    <Route
                        path="/"
                        view=HomePage
                    />
                    <Route
                        path="/board/:id"
                        view=BoardPage
                    />
                    <Route
                        path="/*any"
                        view=NotFound
                    />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-slate-900 text-white">
            // Header
            <header class="p-4 border-b border-slate-700">
                <div class="max-w-7xl mx-auto flex justify-between items-center">
                    <h1 class="text-2xl font-bold">"Space Game"</h1>
                    <div class="space-x-4">
                        <button class="px-4 py-2 bg-slate-700 rounded hover:bg-slate-600">
                            "Login"
                        </button>
                        <button class="px-4 py-2 bg-slate-700 rounded hover:bg-slate-600">
                            "Sign Up"
                        </button>
                    </div>
                </div>
            </header>

            // Main content
            <main class="max-w-7xl mx-auto p-4">
                <div class="grid grid-cols-12 gap-4">
                    // Left sidebar - Players
                    <div class="col-span-3 bg-slate-800 rounded-lg p-4">
                        <h2 class="text-xl font-semibold mb-4">"Players"</h2>
                        
                        // Friends section
                        <div class="mb-6">
                            <h3 class="text-sm text-slate-400 mb-2">"Friends"</h3>
                            <div class="space-y-2">
                                <button class="flex items-center space-x-2 w-full p-2 rounded hover:bg-slate-700">
                                    // We'll add an icon component later
                                    <span>"No friends yet"</span>
                                </button>
                            </div>
                        </div>

                        // AI Players section
                        <div>
                            <h3 class="text-sm text-slate-400 mb-2">"AI Players"</h3>
                            <button class="flex items-center space-x-2 w-full p-2 rounded hover:bg-slate-700">
                                <span>"Random Player"</span>
                            </button>
                        </div>
                    </div>

                    // Main content area
                    <div class="col-span-9 space-y-4">
                        // Share link
                        <div class="bg-slate-800 rounded-lg p-4">
                            <h2 class="text-xl font-semibold mb-2">"Invite Friends"</h2>
                            <div class="flex space-x-2">
                                <input 
                                    type="text"
                                    readonly
                                    value="https://yourgame.com/invite/ABC123"
                                    class="flex-grow bg-slate-700 rounded px-3 py-2"
                                />
                                <button class="p-2 bg-blue-600 rounded hover:bg-blue-500">
                                    "Share"
                                </button>
                            </div>
                        </div>

                        // Create board button
                        <div class="bg-slate-800 rounded-lg p-4">
                            <h2 class="text-xl font-semibold mb-4">"Your Boards"</h2>
                            <button class="flex items-center space-x-2 px-4 py-2 bg-blue-600 rounded hover:bg-blue-500">
                                "Create New Board"
                            </button>
                            
                            // Placeholder for board grid
                            <div class="grid grid-cols-4 gap-4 mt-4">
                                <div class="aspect-square bg-slate-700 rounded-lg flex items-center justify-center">
                                    "No boards yet"
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </main>
        </div>
    }
}

#[component]
fn BoardPage() -> impl IntoView {
    view! {
        <div>"Board page coming soon"</div>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-slate-900 text-white flex items-center justify-center">
            <div class="text-center">
                <h1 class="text-4xl font-bold mb-4">"404"</h1>
                <p class="mb-4">"Page not found"</p>
                <A
                    href="/"
                    class="px-4 py-2 bg-blue-600 rounded hover:bg-blue-500"
                >
                    "Go Home"
                </A>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}