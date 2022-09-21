use yew::prelude::*;

#[function_component(BestFeature)]
pub fn best_feature() -> Html {
    html! {
        <section class="flex flex-col">
            <h1 class="text-center text-6xl mt-16">{"ここがイチオシ！"}</h1>
            <div class="grid grid-cols-3 gap-10 mx-auto my-16">
                <div class="rounded-3xl" style="width: 425px;height:450px;">
                    <div class="bg-[url('static/DummyImage425-240.png')] rounded-t-3xl" style="width: 425px; height: 240px;"/>
                    <div class="relative rounded-b-3xl" style="height:210px; background: #D9D9D9">
                        <div class="absolute text-center" style="top: 50%; left: 50%; transform: translate(-50%, -50%); font-size:32px; width:400px">
                            {"オンライン対戦ゲーム!"}
                        </div>
                    </div>
                </div>
                <div class="rounded-3xl" style="width: 425px;height:450px;">
                    <div class="bg-[url('static/DummyImage425-240.png')] rounded-t-3xl" style="width: 425px; height: 240px;"/>
                    <div class="relative rounded-b-3xl" style="height:210px; background: #D9D9D9">
                        <div class="absolute text-center" style="top: 50%; left: 50%; transform: translate(-50%, -50%); font-size:32px; width:400px">
                            {"レートマッチング！"}
                        </div>
                    </div>
                </div>
                <div class="rounded-3xl" style="width: 425px;height:450px;background:#E6E6E6">
                    <div class="bg-[url('static/BestFeature/Charactors.png')] bg-center w-full h-screen bg-cover rounded-t-3xl" style="width: 425px; height: 240px;"/>
                    <div class="relative rounded-b-3xl" style="height:210px; background: #D9D9D9">
                        <div class="absolute text-center" style="top: 50%; left: 50%; transform: translate(-50%, -50%); font-size:32px; width:400px">
                            {"ニッシー☆が描いたドット絵！"}
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
