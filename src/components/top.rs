use yew::prelude::*;

#[function_component(Top)]
pub fn top() -> Html {
    html! {
        <section>
            <div class="bg-[url('static/BackGroundImage.svg')] bg-center w-full h-screen bg-cover absolute"/>
            <div class="relative h-screen w-32">
                <h1 class="absolute text-9xl" style="background: linear-gradient(117.17deg, #FF0000 0%, #2400FF 97.74%);
                -webkit-background-clip: text;
                -webkit-text-fill-color: transparent;
                background-clip: text;
                text-fill-color: transparent;
                left:80px;
                bottom:100px">{"Ready\nEliminate\nDominator"}</h1>
            </div>
        </section>
    }
}
