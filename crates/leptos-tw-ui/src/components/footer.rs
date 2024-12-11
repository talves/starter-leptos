use leptos::prelude::*;

#[component]
pub fn Footer<F, IV>(
    /// Takes a function (type F) that returns anything that can be
    /// converted into a View (type IV)
    render_prop: F,
    /// `children` takes the `Children` type
    children: Children,
    #[prop(into, optional)] class: MaybeProp<String>,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
    <footer class=move || format!("{}", class.get().unwrap_or_default())>
        {render_prop()}
        {children()}
    </footer>
    }
}
