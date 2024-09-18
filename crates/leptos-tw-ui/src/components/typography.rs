use leptos::*;

use super::variants::base::ClassVariant;

pub enum TypographyVariant {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Paragraph,
    SpanInline,
    Span { inline: bool },
    CodeInline,
    Code { inline: bool },
}

#[component]
pub fn Typography(
    variant: TypographyVariant,
    #[prop(into, optional)] id: Option<AttributeValue>,
    /// Implement trait TypographyClass for TypographyClassVariant
    #[prop(into, optional)]
    class: Option<ClassVariant>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    match variant {
        TypographyVariant::H1 => H1(H1Props {
            id,
            class,
            style,
            children,
        })
        .into_view(),
        TypographyVariant::H2 => H2(H2Props {
            id,
            class,
            style,
            children,
        })
        .into_view(),
        TypographyVariant::H3 => H3(H3Props {
            id,
            class,
            style,
            children,
        })
        .into_view(),
        TypographyVariant::H4 => H4(H4Props {
            id,
            class,
            style,
            children,
        })
        .into_view(),
        TypographyVariant::H5 => H5(H5Props {
            id,
            class,
            style,
            children,
        })
        .into_view(),
        TypographyVariant::H6 => H6(H6Props {
            id,
            class,
            style,
            children,
        })
        .into_view(),
        TypographyVariant::Paragraph => P(PProps {
            id,
            class,
            style,
            children,
        })
        .into_view(),
        TypographyVariant::Span { inline } => Span(SpanProps {
            id,
            class,
            style,
            inline: Some(inline),
            children,
        })
        .into_view(),
        TypographyVariant::SpanInline => Span(SpanProps {
            id,
            class,
            style,
            inline: Some(true),
            children,
        })
        .into_view(),
        TypographyVariant::Code { inline } => Code(CodeProps {
            id,
            class,
            style,
            inline: Some(inline),
            children,
        })
        .into_view(),
        TypographyVariant::CodeInline => Code(CodeProps {
            id,
            class,
            style,
            inline: Some(true),
            children,
        })
        .into_view(),
    }
}

#[component]
pub fn H1(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<ClassVariant>,
    #[prop(into, optional)] style: Option<AttributeValue>,
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
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<ClassVariant>,
    #[prop(into, optional)] style: Option<AttributeValue>,
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
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<ClassVariant>,
    #[prop(into, optional)] style: Option<AttributeValue>,
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
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<ClassVariant>,
    #[prop(into, optional)] style: Option<AttributeValue>,
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
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<ClassVariant>,
    #[prop(into, optional)] style: Option<AttributeValue>,
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
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<ClassVariant>,
    #[prop(into, optional)] style: Option<AttributeValue>,
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
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<ClassVariant>,
    #[prop(into, optional)] style: Option<AttributeValue>,
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
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<ClassVariant>,
    #[prop(into, optional)] style: Option<AttributeValue>,
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
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<ClassVariant>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[prop(optional)] inline: Option<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <code id=id class=format!{"{} {}", class.unwrap_or(ClassVariant::Unstyled).to_string(), {if inline.is_some() {if inline.unwrap() {"inline-flex inline"} else {"block"}} else {"block"}}} style=style >
            {children()}
        </code>
    }
}
