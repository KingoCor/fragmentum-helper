use leptos::*;
use wasm_bindgen_futures::{js_sys::Promise, JsFuture};
use web_sys::HtmlInputElement;
use crate::state::State;

#[component]
pub fn LoadState(
    #[prop(into)]
    on_load: Callback<State>
) -> impl IntoView {
    let load_file = create_action(move |input: &Promise| {
        let input = input.to_owned();
        async move {
            if let Ok(text) = JsFuture::from(input).await {
                if let Ok(state) = serde_json::from_str::<State>(&text.as_string().unwrap()) {
                    on_load.call(state);
                }
            }

        }
    });

    view! {
        <input type="file" accept=".json" on:input=move |ev| {
            let input = event_target::<HtmlInputElement>(&ev);
            if let Some(files) = input.files() {
                let file = files.get(0).unwrap();
                load_file.dispatch(file.text());
            }
        }/>
    }

}
