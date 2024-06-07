use yew::prelude::*;
use reqwest::Client;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;

#[derive(Deserialize, Debug, Clone)]
struct StockData {
    close: Vec<f64>,
}

#[function_component(StockAnalysis)]
pub fn stock_analysis() -> Html {
    let data = use_state(|| None);
    let fetching = use_state(|| false);

    use_effect_with_deps(move |_| {
        let data = data.clone();
        let fetching = fetching.clone();
        spawn_local(async move {
            fetching.set(true);
            let response = Client::new().get("https://api.example.com/nyse_stock_data")
                .send().await.unwrap()
                .json::<StockData>().await.unwrap();
            data.set(Some(response));
            fetching.set(false);
        });
        || ()
    }, ());

    html! {
        <div>
            if *fetching {
                <p>{ "Loading..." }</p>
            } else {
                match &*data {
                    Some(stock_data) => html! {
                        <div>
                            <p>{ format!("Close Prices: {:?}", stock_data.close) }</p>
                        </div>
                    },
                    None => html! {
                        <p>{ "No data available" }</p>
                    },
                }
            }
        </div>
    }
}
