use leptos::*;
use leptos_router::*;

#[derive(Params, PartialEq)]
struct CategoryId {
    id: String,
}

#[component]
pub fn CategoryPage() -> impl IntoView {
    let params = use_params::<CategoryId>();
    // id: || -> usize
    let id = move || {
        params.with(|params| {
            params
                .as_ref()
                .map(move |params| params.id.clone())
                .unwrap_or_default()
        })
    };

    // Mock data
    let products = vec!["Product 1", "Product 2", "Product 3"];

    view! {
        <div>
            <h1>{format!("Category: {}", id())}</h1>
            <ul>
                {
                    products.iter().map(move |product| view! {
                        <li>
                            <a href=format!("/product/{}", product)>{*product}</a>
                        </li>
                    }).collect::<Vec<_>>()
                }
            </ul>
        </div>
    }
}
