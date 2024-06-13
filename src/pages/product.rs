use leptos::*;
use leptos_router::*;

#[derive(Params, PartialEq)]
struct ProductId {
    id: String,
}

#[component]
pub fn ProductPage() -> impl IntoView {
    let params = use_params::<ProductId>();
    // id: || -> usize
    let id = move || {
        params.with(|params| {
            params
                .as_ref()
                .map(|params| params.id.clone())
                .unwrap_or_default()
        })
    };

    view! {
        <div>
            <h1>{format!("Product: {}", id())}</h1>
            <button on:click=move |_| {
                // Add to cart logic
            }>"Add to Cart"</button>
        </div>
    }
}
