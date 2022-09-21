use yew::prelude::*;
use crate::components::game_play::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <GamePlay/>
    }
}
