use yew::prelude::*;

#[function_component(Appeal)]
pub fn appeal() -> Html {
    html! {
        <section class="flex flex-col">
            <h1 class="text-center text-6xl mt-16">{"より有利に戦いたいですか？？"}</h1>
            <h1 class="text-center text-4xl mt-8">{"上のブラウザ版だとラグがひどいです。。"}</h1>
            <div class="bg-[url('static/Appeal/wizard.png')] bg-center w-full h-screen bg-cover mx-auto mt-16" style="width: 384px; height: 384px;"/>
            <h1 class="text-center text-4xl my-16">{"そんなあなたは、、！↓"}</h1>
        </section>
    }
}
