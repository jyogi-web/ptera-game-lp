use crate::app::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Materials)]
pub fn materials() -> Html {
    html! {
        <section>
            <h1>{"Materials"}</h1>
            <Link<Route> to={Route::Home}>{ "click here to go home" }</Link<Route>>
        </section>
    }
}
