use leptos::*;
use leptos_tw_ui::components::{
    buttons::button::LinkButton,
    icons::{IconGitHub, IconLeptos},
};

use crate::theme::ButtonVariant;

#[component]
pub fn GitHubButton() -> impl IntoView {
    view! {
        <LinkButton class="flex max-lg:w-full" href={"https://github.com/talves/starter-leptos"} target="_blank" variant={ButtonVariant::Link.get()}>
            <div class="flex items-center justify-center gap-2">
                {"Contribute"}<IconGitHub class="w-5 fill-white" />
            </div>
        </LinkButton>
    }
}

#[component]
pub fn LeptosButton() -> impl IntoView {
    view! {
        <LinkButton class="flex max-lg:w-full" href={"https://leptos.dev"} target="_blank" variant={ButtonVariant::Link.get()}>
            <div class="flex items-center justify-center gap-2">
                {"Leptos"}<IconLeptos class="w-5 fill-white" />
            </div>
        </LinkButton>
    }
}

#[component]
pub fn StyleGuideButton() -> impl IntoView {
    view! {
        <LinkButton class="flex max-lg:w-full" href={"/styleguide"} variant={ButtonVariant::Solid.get()}>{"Style Guide"}</LinkButton>
    }
}
