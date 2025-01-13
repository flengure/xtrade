// src/webui/src/lib.rs

use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <h1>{ "Welcome to xTrade Web UI" }</h1>
        </div>
    }
}

/// Public function to initialize the Yew application
pub fn start_webui() {
    yew::Renderer::<App>::new().render();
}
