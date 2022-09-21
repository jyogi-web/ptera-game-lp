use crate::components::about_tech::*;
use crate::components::game_play::*;
use crate::components::top::*;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <Top />
            <GamePlay/>
            <AboutTech/>
        </>
    }
}
