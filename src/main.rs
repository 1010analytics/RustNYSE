use yew::prelude::*;

mod components;
use components::stock_analysis::StockAnalysis;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "NYSE Technical Analysis" }</h1>
            <StockAnalysis />
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
