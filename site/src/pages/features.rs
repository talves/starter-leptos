use leptos::*;
use leptos_meta::*;

use crate::sections::features::Features;

#[component]
pub fn FeaturesPage() -> impl IntoView {
    view! {
        <Title text="Starter | Features"/>
        <main>
            <div class="flex flex-col min-h-screen">
                <div class="flex flex-col items-center max-lg:px-6 justify-start min-h-[calc(100vh-80px)] pt-[80px]">
                    <h1 class="text-white text-center font-roboto text-[30px] sm:text-[40px] xl:text-[72px] font-extrabold leading-[110%] max-w-[994px]">
                        Features
                    </h1>
                    <img src="/assets/starter-trans.png" class="w-full max-w-[450px]" />
                    <Features />
                </div>
            </div>
        </main>
    }
}
