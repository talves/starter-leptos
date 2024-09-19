use leptos::*;

use crate::components::buttons::{GitHubButton, StyleGuideButton};

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center max-lg:px-6 justify-start min-h-[calc(100vh-160px)] pt-[80px] bg-primary dark:bg-secondary">
          <h1 class="text-white text-center font-roboto text-[30px] sm:text-[40px] xl:text-[72px] font-extrabold leading-[110%] max-w-[994px]">
            Leptos Starter
          </h1>
          <h2 class="text-white py-[25px] font-roboto text-[16px] lg:text-[20px] tracking-[160%] opacity-65 max-w-[798px] text-center">
            This starter uses Tailwind and Leptos written in Rust.
          </h2>

          <div class="flex max-lg:flex-col items-center gap-4">
            <StyleGuideButton />

            <GitHubButton />
          </div>

          <img src="/assets/starter-trans.webp" class="w-full max-w-[450px]" />
        </div>
    }
}
