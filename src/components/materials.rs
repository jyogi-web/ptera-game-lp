use yew::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct Material {
    pub title: String,
    pub msg: String,
}

impl Material {
    pub fn render(&self) -> Html {
        html! {
            <div class="rounded-3xl mx-auto" style="width: 425px;height:450px;background:#cccccc;margin-bottom:20px;">
                <div class="bg-[url('static/Material/wizard.png')] bg-center w-full h-screen bg-cover rounded-t-3xl mx-auto" style="width:384px;height:384px;"/>
                <div class="relative rounded-b-3xl" style="height:150px; background: #D9D9D9">
                    <div class="absolute text-center" style="top: 50%; left: 50%; transform: translate(-50%, -50%); font-size:32px; width:400px">
                        {"wizard"}
                        <br/>
                        {"一番最初に描いた絵！"}
                    </div>
                </div>
            </div>
        }
    }
}

#[function_component(Materials)]
pub fn materials() -> Html {
    html! {
        <section>
            <h1 class="text-center text-6xl mt-32">{"ゲームの素材集"}</h1>
            <div class="grid grid-cols-3 mx-auto mt-16 mb-32" style="gap:10em">
                <div class="rounded-3xl mx-auto" style="width: 425px;height:450px;background:#cccccc;margin-bottom:20px;">
                    <div class="bg-[url('static/Material/wizard.png')] bg-center w-full h-screen bg-cover rounded-t-3xl mx-auto" style="width:384px;height:384px;"></div>
                    <div class="relative rounded-b-3xl" style="height:150px; background: #D9D9D9">
                        <div class="absolute text-center" style="top: 50%; left: 50%; transform: translate(-50%, -50%); font-size:32px; width:400px">
                            {"wizard"}
                            <br/>
                            {"一番最初に描いた絵！"}
                        </div>
                    </div>
                </div>
                <div class="rounded-3xl mx-auto" style="width: 425px;height:450px;background:#cccccc">
                    <div class="bg-[url('static/Material/wizard_red.png')] bg-center w-full h-screen bg-cover rounded-t-3xl mx-auto" style="width:384px;height:384px;"/>
                    <div class="relative rounded-b-3xl" style="height:150px; background: #D9D9D9">
                        <div class="absolute text-center" style="top: 50%; left: 50%; transform: translate(-50%, -50%); font-size:32px; width:400px">
                            {"wizard_red"}
                            <br/>
                            {"wizardの赤アレンジ"}
                        </div>
                    </div>
                </div>
                <div class="rounded-3xl mx-auto" style="width: 425px;height:450px;background:#cccccc">
                    <div class="bg-[url('static/Material/wizard_blue.png')] bg-center w-full h-screen bg-cover rounded-t-3xl mx-auto" style="width:384px;height:384px;"/>
                    <div class="relative rounded-b-3xl" style="height:150px; background: #D9D9D9">
                        <div class="absolute text-center" style="top: 50%; left: 50%; transform: translate(-50%, -50%); font-size:32px; width:400px">
                            {"wizard_blue"}
                            <br/>
                            {"wizardの青アレンジ"}
                        </div>
                    </div>
                </div>
                <div class="rounded-3xl mx-auto" style="width: 425px;height:450px;background:#cccccc">
                    <div class="bg-[url('static/Material/infantry_red.png')] bg-center w-full h-screen bg-cover rounded-t-3xl mx-auto" style="width:384px;height:384px;"/>
                    <div class="relative rounded-b-3xl" style="height:150px; background: #D9D9D9">
                        <div class="absolute text-center" style="top: 50%; left: 50%; transform: translate(-50%, -50%); font-size:32px; width:400px">
                            {"infantry_red"}
                            <br/>
                            {"赤歩兵"}
                        </div>
                    </div>
                </div>
                <div class="rounded-3xl mx-auto" style="width: 425px;height:450px;background:#cccccc">
                    <div class="bg-[url('static/Material/infantry_blue.png')] bg-center w-full h-screen bg-cover rounded-t-3xl mx-auto" style="width:384px;height:384px;"/>
                    <div class="relative rounded-b-3xl" style="height:150px; background: #D9D9D9">
                        <div class="absolute text-center" style="top: 50%; left: 50%; transform: translate(-50%, -50%); font-size:32px; width:400px">
                            {"infantry_blue"}
                            <br/>
                            {"青歩兵"}
                        </div>
                    </div>
                </div>
                <div class="rounded-3xl mx-auto" style="width: 425px;height:450px;background:#cccccc">
                    <div class="bg-[url('static/Material/knight_red.png')] bg-center w-full h-screen bg-cover rounded-t-3xl mx-auto" style="width:384px;height:384px;"/>
                    <div class="relative rounded-b-3xl" style="height:150px; background: #D9D9D9">
                        <div class="absolute text-center" style="top: 50%; left: 50%; transform: translate(-50%, -50%); font-size:32px; width:400px">
                            {"knight_red"}
                            <br/>
                            {"赤騎士"}
                        </div>
                    </div>
                </div>
                <div class="rounded-3xl mx-auto" style="width: 425px;height:450px;background:#cccccc">
                    <div class="bg-[url('static/Material/knight_blue.png')] bg-center w-full h-screen bg-cover rounded-t-3xl mx-auto" style="width:384px;height:384px;"/>
                    <div class="relative rounded-b-3xl" style="height:150px; background: #D9D9D9">
                        <div class="absolute text-center" style="top: 50%; left: 50%; transform: translate(-50%, -50%); font-size:32px; width:400px">
                            {"knight_blue"}
                            <br/>
                            {"青騎士"}
                        </div>
                    </div>
                </div>
                <div class="rounded-3xl mx-auto" style="width: 425px;height:450px;background:#cccccc">
                    <div class="bg-[url('static/Material/tower.png')] bg-center w-full h-screen bg-cover rounded-t-3xl mx-auto" style="width:384px;height:384px;"/>
                    <div class="relative rounded-b-3xl" style="height:150px; background: #D9D9D9">
                        <div class="absolute text-center" style="top: 50%; left: 50%; transform: translate(-50%, -50%); font-size:32px; width:400px">
                            {"タワー"}
                            <br/>
                            {"sirin1001に作ってもらいました！"}
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
