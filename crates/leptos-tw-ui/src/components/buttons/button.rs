use leptos::{ev::MouseEvent, prelude::*};

use crate::components::variants::base::ClassVariant;

#[component]
pub fn Button<F>(
    on_click: F,
    #[prop(into, optional)] variant: MaybeProp<ClassVariant>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    children: Children,
) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! {
        <button
            type="button"
            id=id
            class=move || format!("{} {}", variant.get().unwrap_or_default(), class.get().unwrap_or_default())
            style=style.get().unwrap_or_default()
            aria-disabled=move || disabled.get().unwrap_or_default()
            on:click=move |e| {
                // The question here is do we always allow the click for usability and let the outside function
                //  handle the click validation of the button? https://css-tricks.com/making-disabled-buttons-more-inclusive
                // if !disabled.get_untracked() {
                    e.stop_propagation();
                    on_click(e);
                // }
            }
        >
            { children() }
        </button>
    }
}

#[component]
pub fn LinkButton(
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] variant: MaybeProp<ClassVariant>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] style: Option<String>,
    #[prop(into, optional)] target: Option<String>,
    href: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <a
            id=id
            href=href // format!("{}", )
            class=format!("{} {}", variant.get().unwrap_or_default(), class.get().unwrap_or_default())
            style=style
            target=target
            aria-disabled=move || disabled.get().unwrap_or_default()
        >
            { children() }
        </a>
    }
}
