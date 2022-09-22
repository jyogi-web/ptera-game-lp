use yew::prelude::*;

#[function_component(Description)]
pub fn description() -> Html {
    html! {
        <section class="flex">
            <div class="flex-none w-20"></div>
            <p class="mt-16 grow text-xl">{"ハッカソンでゲームを作りたい！オンラインゲームを作りたい！という思いからこのゲームは作られました。
            このゲーム(Ready Eliminate Dominator)はオンライン対戦型、1v1のタワーディフェンスです。3種のキャラクターを召喚し、相手の城を攻め落としましょう。"}</p>
            <div class="flex-none w-20"></div>
        </section>
    }
}
