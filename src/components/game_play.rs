use yew::prelude::*;

#[function_component(GamePlay)]
pub fn game_play() -> Html {
    html! {
        <section class="flex flex-col w-full">
            <h1 class="text-center text-6xl mt-16">{"さっそくプレイ！（ブラウザ版）"}</h1>
            <p class="text-center text-xl mt-16">{"黒い画面のときはロード中です。ダウンロード版ならすぐに遊べます。"}</p>
            <iframe
                id="RED"
                title="RED"
                width="960"
                height="600"
                src="https://jyogi-games.yukinissie.com"
                class="mx-auto mt-16 mb-16">
            </iframe>
        </section>
    }
}
