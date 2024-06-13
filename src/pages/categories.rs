use leptos::*;

#[component]
pub fn CategoriesPage() -> impl IntoView {
    // Mock data
    let categories = vec!["Electronics", "Clothing", "Books"];

    view! {
        <div>
            <h1>"Categories"</h1>
            <ul>
                {
                    categories.iter().map(move |category| view! {
                        <li>
                            <a href=format!("/category/{}", category)>{*category}</a>
                        </li>
                    }).collect::<Vec<_>>()
                }
            </ul>
        </div>
    }
}
