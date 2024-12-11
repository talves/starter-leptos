use crate::components::variants::base::ClassVariant;
use leptos::prelude::*;

#[component]
pub fn Container(
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] variant: MaybeProp<ClassVariant>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div id=id class={format!("{} {}", variant.get().unwrap_or_default(), class.get().unwrap_or_default())} style=style>
            {children()}
        </div>
    }
}

#[allow(unused_braces)]
#[component]
pub fn ContainerFromProp<F, IV>(
    /// Takes a function (type F) that returns anything that can be
    /// converted into a View (type IV)
    render_view: F,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <>{render_view()}</>
    }
}

#[component]
pub fn Main(
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <main id=id role="main" class=class style=style>
            {children()}
        </main>
    }
}
