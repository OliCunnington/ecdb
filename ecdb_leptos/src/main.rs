use leptos::*;
use leptos::prelude::*;
use gloo_net::http::Request;

fn main() {
    leptos::mount::mount_to_body(MyComponent)
}


async fn fetch_data() -> Result<String, String> {
    let res = Request::get("localhost:3000/api/test")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    res.text().await.map_err(|e| e.to_string())
}

#[component]
pub fn MyComponent() -> impl IntoView {
    let fetch_data_action = Action::new(|_input: &()| { //async { //} move {
        fetch_data()
    });

    view! {
        <button on:click=move |_| { fetch_data_action.dispatch(()); }>
            "Fetch Data"
        </button>
        <p>{move || fetch_data_action.value().get().unwrap_or_else(|| Err("Loading...".to_string()))}</p>
    }
}   