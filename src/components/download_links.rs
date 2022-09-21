use yew::prelude::*;

#[function_component(DownloadLinks)]
pub fn download_links() -> Html {
    html! {
        <section class="flex flex-col" style="background: #cccccc">
            <h1 class="text-center text-6xl mt-16">{"デスクトップ版を今すぐダウンロード！"}</h1>
            <div class="flex flex-row justify-center mt-16 mb-16 text-2xl" style="height:200px">
                <div class="flex flex-col justify-center border-r-4 pr-20" style="height:200px">
                    <a href="https://jyogi-games-webgl.s3.ap-northeast-1.amazonaws.com/Build-StandaloneWindows64.zip" class="underline underline-offset-auto">{"windows x64"}</a>
                </div>
                <div class="flex flex-col justify-center pl-20" style="height:200px">
                    <a href="https://jyogi-games-webgl.s3.ap-northeast-1.amazonaws.com/intel-mac-build.app.zip" class="underline underline-offset-auto">{"macOS (intel)"}</a>
                    <a href="https://jyogi-games-webgl.s3.ap-northeast-1.amazonaws.com/silicon-mac-build.app.zip" class="mt-6 underline underline-offset-auto">{"macOS (silicon)"}</a>
                </div>
            </div>
        </section>
    }
}
