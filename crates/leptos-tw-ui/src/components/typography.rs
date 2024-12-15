use leptos::prelude::*;

use super::variants::base::ClassVariant;

#[component]
pub fn H1(
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] class: Option<ClassVariant>,
    #[prop(into, optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <h1 id=id class=class.unwrap_or(ClassVariant::Unstyled).to_string() style=style>
            {children()}
        </h1>
    }
}

#[component]
pub fn H2(
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] class: Option<ClassVariant>,
    #[prop(into, optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <h2 id=id class=class.unwrap_or(ClassVariant::Unstyled).to_string() style=style>
            {children()}
        </h2>
    }
}

#[component]
pub fn H3(
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] class: Option<ClassVariant>,
    #[prop(into, optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <h3 id=id class=class.unwrap_or(ClassVariant::Unstyled).to_string() style=style>
            {children()}
        </h3>
    }
}

#[component]
pub fn H4(
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] class: Option<ClassVariant>,
    #[prop(into, optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <h4 id=id class=class.unwrap_or(ClassVariant::Unstyled).to_string() style=style>
            {children()}
        </h4>
    }
}

#[component]
pub fn H5(
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] class: Option<ClassVariant>,
    #[prop(into, optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <h5 id=id class=class.unwrap_or(ClassVariant::Unstyled).to_string() style=style>
            {children()}
        </h5>
    }
}

#[component]
pub fn H6(
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] class: Option<ClassVariant>,
    #[prop(into, optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <h6 id=id class=class.unwrap_or(ClassVariant::Unstyled).to_string() style=style>
            {children()}
        </h6>
    }
}

#[component]
pub fn P(
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] class: Option<ClassVariant>,
    #[prop(into, optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <p id=id class=class.unwrap_or(ClassVariant::Unstyled).to_string() style=style>
            {children()}
        </p>
    }
}

#[component]
pub fn Span(
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] class: Option<ClassVariant>,
    #[prop(into, optional)] style: Option<String>,
    #[prop(optional)] inline: Option<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <span id=id class=format!{"{} {}", class.unwrap_or(ClassVariant::Unstyled).to_string(), {if inline.is_some() {if inline.unwrap() {"inline-flex inline"} else {"block"}} else {"block"}}} style=style>
            {children()}
        </span>
    }
}

#[component]
pub fn Code(
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] class: Option<ClassVariant>,
    #[prop(into, optional)] style: Option<String>,
    #[prop(optional)] inline: Option<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <code id=id class=format!{"{} {}", class.unwrap_or(ClassVariant::Unstyled).to_string(), {if inline.is_some() {if inline.unwrap() {"inline-flex inline"} else {"block"}} else {"block"}}} style=style >
            {children()}
        </code>
    }
}
