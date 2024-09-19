use leptos::*;
use leptos_tw_ui::components::icons::IconCheckMark;

use crate::components::buttons::{GitHubButton, LeptosButton};

#[component]
pub fn Features() -> impl IntoView {
    let features = [
        "Single-page apps (SPAs) rendered entirely in the browser",
        "Multi-page apps (MPAs) rendered on the server (not on this starter)",
        "CSR (this starter) or SSR",
        "Client-side Routing ",
        "Optional to have single-page apps that are rendered on the server and then hydrated on the client (requires server hosting)",
        "Easily manage state without fighting the borrow checker with reactive signals.",
    ];

    view! {
      <div class="mb-[100px]">
        <div class="pt-[80px] flex flex-col items-center justify-center container mx-auto max-lg:px-6">
          <p class="text-white z-10 font-roboto text-[24px] md:text-[32px] lg:text-[48px] font-extrabold max-w-[900px] text-center leading-[110%]">
            Leptos is a full-stack framework for building web applications in Rust
          </p>
        </div>

        <div class="max-lg:py-[20px] py-[80px] flex gap-[80px] max-xl:items-center max-xl:justify-center lg:container lg:mx-auto max-lg:px-6">
          <div class="flex flex-col items-start justify-center">
            <p class="font-roboto text-[34px] lg:text-[48px] font-extrabold tracking-[110%] text-white">
              High Performance Web Framework.
            </p>
            <p class="text-secondary dark:text-primary font-roboto text-[34px] lg:text-[48px] font-extrabold tracking-[110%] -mt-4">
              Full stack, fully typed.
            </p>

            <p class="text-white font-roboto text-[18px] leading-[160%] opacity-65">
              Here are some of the key features of Leptos:
            </p>

            <div class="mt-4">
            {features.into_iter()
              .map(|feature| view! { <div class="flex items-center justify-start text-white gap-2">
                    <IconCheckMark class="w-4 fill-secondary dark:fill-primary" />
                    <p>{feature}</p>
                  </div> })
              .collect::<Vec<_>>()}
            </div>

            <div class="max-sm:w-full mt-[20px]">
              <LeptosButton />
            </div>
          </div>
        </div>

        <div class="flex flex-col items-center justify-center py-[80px] lg:container max-lg:px-6 lg:mx-auto">
          <p class="text-white font-roboto max-lg:text-center font-extrabold max-lg:text-[30px] lg:text-[46px] leading-[110%]">
            Want to start contributing?
          </p>
          <p class="text-secondary dark:text-primary font-roboto max-lg:text-center font-extrabold max-lg:text-[30px] lg:text-[46px] leading-[110%]">
            Check the code out on GitHub
          </p>
          <div class="max-sm:w-full mt-[20px]">
            <GitHubButton />
          </div>

        </div>
      </div>
    }
}
