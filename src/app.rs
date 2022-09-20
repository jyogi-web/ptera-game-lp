use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <iframe
                id="Jyogi Games"
                title="Jyogi Games"
                width="960"
                height="600"
                src="https://jyogi-games.yukinissie.com">
            </iframe>
        </main>
    }
}
