use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use leptos_meta::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

// #[derive(Serialize, Deserialize)]
// struct GreetArgs<'a> {
//     name: &'a str,
// }

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
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }

            // let args = to_value(&GreetArgs { name: &name }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            // let new_msg = invoke("greet", args).await.as_string().unwrap();

            let new_msg = format!("Hello, {name}! You've been greeted from Rust!");

            set_greet_msg.set(new_msg);
        });
    };

    view! {
        <Title text="Leptos, Tauri and Tailwind"/>
        <main>
            <div class="hero min-h-screen">
                <div class="hero-content text-center">
                    <div class="max-w-md">
                        <h1 class="text-5xl font-bold">"Leptos, Tauri and Tailwind"</h1>
                        <p class="py-6">
                            "Petite application de test pour Tauri + Leptos + Tailwind"
                        </p>

                        <form on:submit=greet>
                            <label class="input input-bordered flex items-center gap-2">
                                Name
                                <input
                                    type="text"
                                    class="grow"
                                    placeholder="George"
                                    on:input=update_name
                                />
                                <kbd class="kbd kbd-sm">Enter</kbd>
                            </label>
                        </form>

                        <p class="py-6">
                            <b>{move || greet_msg.get()}</b>
                        </p>

                    </div>
                </div>
            </div>
        </main>
    }
}
