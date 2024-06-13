use leptos::*;

#[derive(Clone, PartialEq, Debug)]
struct Category {
    id: u64,
    name: String,
    img: String,
}

#[component]
pub fn CategoriesPage() -> impl IntoView {
    // Mock data
    let categories: Vec<Category> = vec![
        Category {
            id: 1,
            name: "Хозтовары".to_string(),
            img: "https://vls.pushkind.com/static/upload/2613997710.jpg".to_string(),
        },
        Category {
            id: 2,
            name: "Текстиль".to_string(),
            img: "https://vls.pushkind.com/static/upload/2617406836.jpg".to_string(),
        },
        Category {
            id: 1,
            name: "Хозтовары".to_string(),
            img: "https://vls.pushkind.com/static/upload/2613997710.jpg".to_string(),
        },
        Category {
            id: 2,
            name: "Текстиль".to_string(),
            img: "https://vls.pushkind.com/static/upload/2617406836.jpg".to_string(),
        },
        Category {
            id: 1,
            name: "Хозтовары".to_string(),
            img: "https://vls.pushkind.com/static/upload/2613997710.jpg".to_string(),
        },
        Category {
            id: 2,
            name: "Текстиль".to_string(),
            img: "https://vls.pushkind.com/static/upload/2617406836.jpg".to_string(),
        },
    ];

    view! {
        <div class="container itemFilterList">
            <div class="row row-cols-1 row-cols-lg-6 row-cols-md-4 row-cols-sm-2">
                {
                    categories.iter().map(move |category| view! {
                        <CategoryCard cat=category.clone() />
                    }).collect::<Vec<_>>()
                }
            </div>
        </div>
    }
}

#[component]
fn CategoryCard(cat: Category) -> impl IntoView {
    view! {
        <div class="col my-2 itemFilterItem">
            <div class="card text-center selectable overflow-hidden" data-id=format!("{}", cat.id)>
                <div class="card-body">
                    <a href=format!("/category/{}", cat.id) class="link-dark">
                        <img src=format!("{}", cat.img) height="128" width="128" alt="thumbnail" />
                    </a>
                </div>
                <div class="card-footer bg-white border-top-0">
                    <a href=format!("/category/{}", cat.id) class="link-dark">
                        {cat.name}
                    </a>
                </div>
            </div>
        </div>
    }
}
