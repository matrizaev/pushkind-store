use crate::pages;
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=|| view! { <pages::categories::CategoriesPage /> } />
                    <Route path="/category/:id" view=|| view! { <pages::category::CategoryPage /> } />
                    <Route path="/product/:id" view=|| view! { <pages::product::ProductPage /> } />
                    <Route path="/cart" view=|| view! { <pages::cart::CartPage /> } />
                </Routes>
            </main>
        </Router>
    }
}
