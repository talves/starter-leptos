use leptos::*;
use leptos_meta::*;

use crate::theme::ButtonVariant;
use leptos_tw_ui::components::buttons::button::{Button, LinkButton};

#[component]
pub fn CounterPage() -> impl IntoView {
    let (value, set_value) = create_signal(0);

    // thanks to https://tailwindcomponents.com/component/secondary-buttons-example for the showcase layout
    view! {
        <Title text="Leptos + Tailwindcss | Counter"/>
        <main>
            <div class="bg-gradient-to-tl from-slate-800 to-slate-500 text-white font-mono flex flex-col min-h-screen">
                <LinkButton href="/" variant={ButtonVariant::Solid.get()}>
                    Click for Home
                </LinkButton>
                <div class="flex flex-row-reverse flex-wrap m-auto">
                    <Button on_click=move |_| set_value.update(|value| *value += 1) class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-slate-700 border-slate-800 text-white">
                        "+"
                    </Button>
                    <LinkButton href="#" class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-slate-800 border-slate-900 text-white">
                        {value}
                    </LinkButton>
                    <Button on_click=move |_| set_value.update(|value| *value -= 1) class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-slate-700 border-slate-800 text-white">
                        "-"
                    </Button>
                </div>
            </div>
        </main>
    }
}
