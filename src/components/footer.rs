use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <section class="relative" style="background: #000000; height: 700px">
            <div class="absolute flex flex-row justify-center" style="top: 50%; left: 50%; transform: translate(-50%, -50%); width:100%;">
                <div class="bg-[url('static/Footer/jyogi.png')] bg-center w-full h-screen bg-cover" style="height:170px;width:170px;"/>
                <h1 class="text-9xl ml-20 block align-middle" style="background: linear-gradient(117.17deg, #FF0000 0%, #2400FF 97.74%);
                    -webkit-background-clip: text;
                    -webkit-text-fill-color: transparent;
                    background-clip: text;
                    text-fill-color: transparent;height:170px;line-height:170px">{"Presented By Jyogi."}</h1>
            </div>
        </section>
    }
}
