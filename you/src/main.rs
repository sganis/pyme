use yew::prelude::*;
use serde::Deserialize;
use gloo_net::http::Request;

#[derive(Clone, PartialEq, Deserialize)]
struct StatProduct {
    name: String,
    count: String,
    value: String,
}

#[derive(Properties, PartialEq)]
struct StatProductsListProps {
    stat_products: Vec<StatProduct>,
    on_click: Callback<StatProduct>
}

#[derive(Properties, PartialEq)]
struct StatProductsDetailsProps {
    stat_product: StatProduct,
}

#[function_component]
fn StatProductDetails(StatProductsDetailsProps { stat_product }: &StatProductsDetailsProps) 
    -> Html {
    html! {
        <div>
            <h3>{ stat_product.name.clone() }</h3>
        </div>
    }
}

#[function_component]
fn StatProductsList(StatProductsListProps { stat_products, on_click }: &StatProductsListProps) -> Html {
    
    let on_click = on_click.clone();
    stat_products.iter().map(|stat_product| {
        let on_StatProduct_select = {
            let on_click = on_click.clone();
            let stat_product = stat_product.clone();
            Callback::from(move |_| {
                on_click.emit(stat_product.clone())
            })
        };
        html! {
            <p key={stat_product.name.clone()} 
                onclick={on_StatProduct_select}>
                    {format!("{} {} - {}", stat_product.name, stat_product.count, stat_product.value)}</p>
        }
    }).collect()
}

#[function_component(App)]
fn app() -> Html {
    let stat_products = use_state(|| vec![]);
    {
        let stat_products = stat_products.clone();
        use_effect_with((), move |_| {
            let stat_products = stat_products.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_StatProducts: Vec<StatProduct> = Request::get("/pyme/stat/products/")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                stat_products.set(fetched_StatProducts);
            });
            || ()
        });
    }

    let selected_StatProduct = use_state(|| None);

    let on_StatProduct_select = {
        let selected_StatProduct = selected_StatProduct.clone();
        Callback::from(move |StatProduct: StatProduct| {
            selected_StatProduct.set(Some(StatProduct))
        })
    };

    let details = selected_StatProduct.as_ref().map(|stat_product| html! {
        <StatProductDetails stat_product={stat_product.clone()} />
    });

    html! {
        <>
            <h1>{ "Explorer" }</h1>
            <div>
                <h3>{"Items:"}</h3>
                <StatProductsList 
                    stat_products={(*stat_products).clone()} 
                    on_click={on_StatProduct_select.clone()} />
            </div>
            {for details}
            
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}