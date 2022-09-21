use yew::prelude::*;

#[function_component(AboutTech)]
pub fn about_tech() -> Html {
    html! {
        <section class="flex flex-col">
        <h1 class="text-center text-6xl mt-32">{"技術的な詳細や開発背景はこちら！"}</h1>
        <a href="https://topaz.dev/projects/0bdca801952a9a59bba7" class="relative mx-auto mt-16 mb-16" style="background: linear-gradient(90deg, #28A2BF 0%, #4CC6BB 100%);
            border-radius: 25px;
            width: 645px;
            height: 140px;">
            <div class="absolute" style="font-family: 'Inter';
                font-size: 64px;
                top: 50%;
                left: 50%;
                transform: translate(-50%, -50%);
                color: #FFFFFF;">
                {"Topa'zへ"}
            </div>
        </a>
        </section>
    }
}
