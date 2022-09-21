use crate::app::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(GameMaterialLink)]
pub fn game_material_link() -> Html {
    html! {
        <section class="flex flex-col">
            <h1 class="text-center text-6xl mt-16">{"おまけ"}</h1>
            <Link<Route> to={Route::Materials}>
                <div class="relative mx-auto mt-16 mb-16" style="background: #FF0000;
                    border-radius: 25px;
                    width: 645px;
                    height: 140px;">
                    <div class="absolute" style="font-family: 'Inter';
                        font-size: 40px;
                        top: 50%;
                        left: 50%;
                        transform: translate(-50%, -50%);
                        color: #FFFFFF;">
                        {"ゲームの素材集へ"}
                    </div>
                </div>
            </Link<Route>>
        </section>
    }
}
