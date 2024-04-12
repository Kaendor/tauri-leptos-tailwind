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
        <Stylesheet id="leptos" href="/pkg/tailwind.css" />
        <Title text="Leptos, Tauri and Tailwind" />
        <main class="bg-black py-8">
            <div class="row">
                <p>"Petite application de test pour Tauri + Leptos (soon + Tailwind)"</p>
            </div>

            <form class="row" on:submit=greet>
                <input
                    id="greet-input"
                    placeholder="Enter a name..."
                    on:input=update_name
                />
                <button type="submit">"Greet"</button>
            </form>

            <p><b>{ move || greet_msg.get() }</b></p>
        </main>
    }
}
