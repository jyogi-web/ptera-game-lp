use crate::components::about_tech::*;
use crate::components::appeal::*;
use crate::components::best_feature::*;
use crate::components::download_links::*;
use crate::components::footer::*;
use crate::components::game_material_link::*;
use crate::components::game_play::*;
use crate::components::materials::*;
use crate::components::top::*;
use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::Routable;
use yew_router::Switch;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/materials")]
    Materials,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <>
            <Top />
            <BestFeature />
            <GamePlay/>
            <Appeal/>
            <DownloadLinks/>
            <AboutTech/>
            <GameMaterialLink/>
            <Footer/>
        </> },
        Route::Materials => html! {
            <Materials />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}
