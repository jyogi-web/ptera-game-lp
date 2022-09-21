use yew::prelude::*;
use crate::components::top::*;
use crate::components::game_play::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <Top />
            <GamePlay/>
        </>
    }
}
