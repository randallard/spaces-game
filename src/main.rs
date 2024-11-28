use leptos::*;
use leptos::prelude::*;
use mount::mount_to_body;
use std::convert::Into;

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-slate-900 text-white flex items-center justify-center">
            <h1 class="text-4xl font-bold">"Space Game Coming Soon"</h1>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}