use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let (name, set_name) = create_signal(String::new());
    let (greet_msg, set_greet_msg) = create_signal(String::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();

        let name = name.get_untracked();
        if name.is_empty() {
            return;
        }

        let greeting_word = match name.as_str() {
            "Cowboy" => "Howdy",
            _ => "Hello",
        };

        let new_msg = format!("{greeting_word}, {name}! You've been greeted from Rust!");

        set_greet_msg.set(new_msg);
    };

    view! {
        <Title text="Leptos, Tauri and Tailwind with DaisyUI" />
        <main>
            <div class="hero min-h-screen">
                <div class="hero-content text-center">
                    <div class="max-w-md">
                        <h1 class="text-5xl font-bold">"Leptos, Tauri and Tailwind"</h1>
                        <p class="py-6">
                            "Little test app for Tauri + Leptos + Tailwind with DaisyUI"
                        </p>

                        <form on:submit=greet>
                            <div class="join">
                                <label class="input input-bordered flex items-center gap-2 join-item">
                                    Name
                                    <input
                                        type="text"
                                        class="grow"
                                        placeholder="George"
                                        on:input=update_name
                                    /> <kbd class="kbd kbd-sm">Enter</kbd>
                                </label>
                                <button class="join-item btn btn-primary">"Greet"</button>
                            </div>
                        </form>

                        <p class="py-6">
                            <b>{greet_msg}</b>
                        </p>

                    </div>
                </div>
            </div>
        </main>
    }
}
