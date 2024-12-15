use leptos::prelude::*;
use leptos_meta::Title;
use leptos_tw_ui::components::{
    buttons::button::Button,
    container::{Container, ContainerFromProp, Main},
    theme::toggle::{theme_mode, ThemeToggleButton, ThemeToggleSwitch},
    typography::{Code, Span, H1, H2, H3, H4, H5, H6, P},
};

use crate::sections::hero::Hero;
use crate::theme::{ButtonVariant, ToggleSwitchClassVariant, TypographyClass};
use std::ops::Not;

#[component]
pub fn StyleGuidePage() -> impl IntoView {
    view! {
        <Title text="Starter | Style Guide"/>
        <Main id="main" class="max-w-[85rem] px-4 py-4 sm:px-6 lg:px-8 mx-auto pb-[100px] text-gray-900 dark:text-gray-400">
            <div>
                //   <!-- Announcement Banner -->
                <H1 class={TypographyClass::H1.get()}>Examples with Explanations</H1>
                //   <!-- End Announcement Banner -->
            </div>
            <HeroSection/>
            <DarkModeSection/>
            <ButtonSection/>
            <TypographySection/>
        </Main>
    }
}

#[component]
fn ButtonSection() -> impl IntoView {
    let (disabled, set_disabled) = signal(false);

    view! {
        <section class="max-w-[70rem] px-4 py-4 sm:px-6 lg:px-8 mx-auto">
            <H2 class={TypographyClass::H2.get()} >Buttons</H2>
            <div class="border rounded-xl shadow-sm border-blue-600 dark:bg-slate-800 dark:border-slate-600 p-5 mt-3">
                <H3 class={TypographyClass::H3.get()}>Inline Block</H3>
                <div class="p-4 inline-block">
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())}>{"Default (Unstyled)"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| web_sys::console::log_1(&e.target().unwrap())} variant={ButtonVariant::Solid.get()}>{"Solid"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Outline.get()}>{"Outline"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Ghost.get()}>{"Ghost"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Soft.get()}>{"Soft"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::White.get()}>{"White"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Link.get()}>{"Link"}</Button>
                </div>
                <H3 class={TypographyClass::H3.get()}>Grid (md:grid-cols-3)</H3>
                <div class="p-4 grid md:grid-cols-3 gap-4">
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())}>{"Default (Unstyled)"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| web_sys::console::log_1(&e.target().unwrap())} variant={ButtonVariant::Solid.get()}>{"Solid"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Outline.get()}>{"Outline"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Ghost.get()}>{"Ghost"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Soft.get()}>{"Soft"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::White.get()}>{"White"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Link.get()}>{"Link"}</Button>
                </div>
                <H3 class={TypographyClass::H3.get()}>Flex (flex-wrap)</H3>
                <div class="p-4 flex flex-wrap gap-2">
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())}>{"Default (Unstyled)"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| web_sys::console::log_1(&e.target().unwrap())} variant={ButtonVariant::Solid.get()}>{"Solid"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Outline.get()}>{"Outline"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Ghost.get()}>{"Ghost"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Soft.get()}>{"Soft"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::White.get()}>{"White"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Link.get()}>{"Link"}</Button>
                    <Button class="ml-0 mx-1" on_click=move |_| set_disabled(if !disabled.get_untracked() {true} else {false}) disabled=disabled variant={ButtonVariant::Solid.get()}>{move || if disabled.get() {"Disabled"} else {"Enabled"}}</Button>
                    <Button class="ml-0 mx-1" on_click=move |_| set_disabled.update(|value| *value = if *value {false} else {true}) disabled=disabled variant={ButtonVariant::Solid.get()}>{move || if disabled.get() {"Disabled"} else {"Enabled"}}</Button>
                    <Button class="ml-0 mx-1" on_click=move |_| set_disabled.update(|value| *value = value.not()) disabled=disabled variant={ButtonVariant::Solid.get()}>{move || if disabled.get() {"Disabled"} else {"Enabled"}}</Button>
                </div>
            </div>
        </section>
    }
}

#[component]
fn TypographySection() -> impl IntoView {
    view! {
        <section class="max-w-[70rem] px-4 py-4 sm:px-6 lg:px-8 mx-auto">
            <H2 class={TypographyClass::H2.get()}>Typography</H2>
            <div class="border rounded-xl shadow-sm  border-blue-600 dark:bg-slate-800 dark:border-slate-600 p-5 mt-3">
                <H1 class={TypographyClass::H1.get()}>H1</H1>
                <H2 class={TypographyClass::H2.get()}>H2</H2>
                <H3 class={TypographyClass::H3.get()}>H3</H3>
                <H4 class={TypographyClass::H4.get()}>H4</H4>
                <H5 class={TypographyClass::H5.get()}>H5</H5>
                <H6 class={TypographyClass::H6.get()}>H6</H6>
                <P class={TypographyClass::P.get()}>This is a Paragraph</P>
                <p>The word <Span class={TypographyClass::Span.get()}>span</Span> in this sentence is wrapped,
                also the following code
                <Code inline=false class={TypographyClass::Code.get()}>This is a Code block</Code>
                is wrapped.</p>
                <p>The word <Span inline=false class={TypographyClass::Span.get()}>span</Span> in this sentence is wrapped,
                also the following code
                <Code inline=false class={TypographyClass::Code.get()}>This is a Code block</Code>
                is wrapped but they are not inline.</p>
                <Code inline=false class={TypographyClass::Code.get()}>
                    <Span inline=false>let num: i32 = 100;</Span>
                    <Span inline=false>"println!{""{}"", num};"</Span>
                </Code>
                <ContainerFromProp render_view=|| view! { <p class="mt-1 font-medium">
                      {"This is content from a render_view property"}
                  </p>} />
            </div>
        </section>
    }
}

#[component]
fn DarkModeSection() -> impl IntoView {
    view! {
        <section class="max-w-[70rem] px-4 py-4 sm:px-6 lg:px-8 mx-auto">
            <H2 class={TypographyClass::H2.get()}>Dark/Light Theme Toggle</H2>
            <P class={TypographyClass::P.get()}>
                There are currently two options for toggle component (ThemeToggleButton and ThemeToggleSwitch). All variants use an on click event to
                change the mode of the theme from dark to light. The preference is stored into local storage for persistence between sessions.
                Icon switching is not set by stored state, but by the theme setting using tailwindcss (hidden). Initial defaults to dark if there is a system preference.
            </P>
            <div class="border rounded-xl shadow-sm border-blue-600 dark:bg-slate-800 dark:border-slate-600 p-5 mt-3">
                <Container class="py-2">
                    <ThemeToggleButton mode_fn={theme_mode} class="text-yellow-700 dark:text-primary-400 focus:outline-none text-sm p-1"
                        icon_light_class="dark:hidden w-9 h-9 fill-orange-300 hover:bg-yellow-200 rounded-2xl"
                        icon_dark_class="hidden dark:block w-9 h-9 fill-yellow-300 hover:fill-gray-800 hover:bg-yellow-300 rounded-2xl" />
                </Container>
                <Container class="py-2">
                    <ThemeToggleSwitch mode_fn={theme_mode} class={ToggleSwitchClassVariant::Knob.get()} />
                </Container>
                <Container class="py-2">
                    <ThemeToggleSwitch mode_fn={theme_mode} class={ToggleSwitchClassVariant::Encased.get()} />
                </Container>
            </div>
        </section>
    }
}

#[component]
fn HeroSection() -> impl IntoView {
    view! {
        <Hero/>
    }
}
