use leptos::prelude::*;

use crate::components::variants::base::ClassVariant;

#[component]
pub fn MenuBar(
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] variant: MaybeProp<ClassVariant>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <nav
            id=id
            class=format!("{} {}", variant.get().unwrap_or_default(), class.get().unwrap_or_default())
            style=style
        >
            { children() }
        </nav>
    }
}

#[component]
pub fn MenuHeader(
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] variant: MaybeProp<ClassVariant>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <header
            id=id
            class=format!("{} {}", variant.get().unwrap_or_default(), class.get().unwrap_or_default())
            style=style
        >
            { children() }
        </header>
    }
}
