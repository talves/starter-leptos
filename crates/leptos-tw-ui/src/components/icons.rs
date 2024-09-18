use leptos::*;

use crate::OptionMaybeSignal;

#[component]
pub fn IconSun(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: OptionMaybeSignal<String>,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView {
    view! {
        <svg
            id=id
            class=class.get()
            style=style
            viewBox="0 0 20 20"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                d="M10 2a1 1 0 011 1v1a1 1 0 11-2 0V3a1 1 0 011-1zm4 8a4 4 0 11-8 0 4 4 0 018 0zm-.464 4.95l.707.707a1 1 0 001.414-1.414l-.707-.707a1 1 0 00-1.414 1.414zm2.12-10.607a1 1 0 010 1.414l-.706.707a1 1 0 11-1.414-1.414l.707-.707a1 1 0 011.414 0zM17 11a1 1 0 100-2h-1a1 1 0 100 2h1zm-7 4a1 1 0 011 1v1a1 1 0 11-2 0v-1a1 1 0 011-1zM5.05 6.464A1 1 0 106.465 5.05l-.708-.707a1 1 0 00-1.414 1.414l.707.707zm1.414 8.486l-.707.707a1 1 0 01-1.414-1.414l.707-.707a1 1 0 011.414 1.414zM4 11a1 1 0 100-2H3a1 1 0 000 2h1z"
                fill-rule="evenodd"
                clip-rule="evenodd"></path>
        </svg>
    }
}

#[component]
pub fn IconMoon(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: OptionMaybeSignal<String>,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView {
    view! {
        <svg
        id=id
        class=class.get()
        style=style
        viewBox="0 0 24 24"
        xmlns="http://www.w3.org/2000/svg"
        >
            <path
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M17.715 15.15A6.5 6.5 0 0 1 9 6.035C6.106 6.922 4 9.645 4 12.867c0 3.94 3.153 7.136 7.042 7.136 3.101 0 5.734-2.032 6.673-4.853Z"
            ></path>
            <path
                d="m17.715 15.15.95.316a1 1 0 0 0-1.445-1.185l.495.869ZM9 6.035l.846.534a1 1 0 0 0-1.14-1.49L9 6.035Zm8.221 8.246a5.47 5.47 0 0 1-2.72.718v2a7.47 7.47 0 0 0 3.71-.98l-.99-1.738Zm-2.72.718A5.5 5.5 0 0 1 9 9.5H7a7.5 7.5 0 0 0 7.5 7.5v-2ZM9 9.5c0-1.079.31-2.082.845-2.93L8.153 5.5A7.47 7.47 0 0 0 7 9.5h2Zm-4 3.368C5 10.089 6.815 7.75 9.292 6.99L8.706 5.08C5.397 6.094 3 9.201 3 12.867h2Zm6.042 6.136C7.718 19.003 5 16.268 5 12.867H3c0 4.48 3.588 8.136 8.042 8.136v-2Zm5.725-4.17c-.81 2.433-3.074 4.17-5.725 4.17v2c3.552 0 6.553-2.327 7.622-5.537l-1.897-.632Z"
                class="fill-gray-200"></path>
            <path
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M17 3a1 1 0 0 1 1 1 2 2 0 0 0 2 2 1 1 0 1 1 0 2 2 2 0 0 0-2 2 1 1 0 1 1-2 0 2 2 0 0 0-2-2 1 1 0 1 1 0-2 2 2 0 0 0 2-2 1 1 0 0 1 1-1Z"
            ></path>
        </svg>
    }
}

#[component]
pub fn IconMenuOn(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: OptionMaybeSignal<String>,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView {
    view! {
        <svg
            id=id
            class=class.get()
            style=style
            viewBox="0 0 16 16"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708z"
                fill-rule="evenodd"
                clip-rule="evenodd"></path>
        </svg>
    }
}

#[component]
pub fn IconMenuOff(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: OptionMaybeSignal<String>,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView {
    view! {
        <svg
            id=id
            class=class.get()
            style=style
            viewBox="0 0 16 16"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                d="M2.5 12a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5zm0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5zm0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5z"
                fill-rule="evenodd"
                clip-rule="evenodd"></path>
        </svg>
    }
}

#[component]
pub fn IconGitHub(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: OptionMaybeSignal<String>,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView {
    view! {
        <svg
        id=id
        class=class.get()
        style=style
        viewBox="0 0 24 24"
        xmlns="http://www.w3.org/2000/svg"
        >
            <path
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M12 2C6.475 2 2 6.475 2 12a9.994 9.994 0 0 0 6.838 9.488c.5.087.687-.213.687-.476 0-.237-.013-1.024-.013-1.862-2.512.463-3.162-.612-3.362-1.175-.113-.288-.6-1.175-1.025-1.413-.35-.187-.85-.65-.013-.662.788-.013 1.35.725 1.538 1.025.9 1.512 2.338 1.087 2.912.825.088-.65.35-1.087.638-1.337-2.225-.25-4.55-1.113-4.55-4.938 0-1.088.387-1.987 1.025-2.688-.1-.25-.45-1.275.1-2.65 0 0 .837-.262 2.75 1.026a9.28 9.28 0 0 1 2.5-.338c.85 0 1.7.112 2.5.337 1.912-1.3 2.75-1.024 2.75-1.024.55 1.375.2 2.4.1 2.65.637.7 1.025 1.587 1.025 2.687 0 3.838-2.337 4.688-4.562 4.938.362.312.675.912.675 1.85 0 1.337-.013 2.412-.013 2.75 0 .262.188.574.688.474A10.016 10.016 0 0 0 22 12c0-5.525-4.475-10-10-10z"
            ></path>
        </svg>
    }
}

#[component]
pub fn IconLeptos(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: OptionMaybeSignal<String>,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView {
    view! {
        <svg
        id=id
        class=class.get()
        style=style
        viewBox="0 0 24 24"
        xmlns="http://www.w3.org/2000/svg"
        >
            <path
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M 23.726562 3.191406 C 23.722656 2.199219 23.21875 1.28125 22.382812 0.742188 C 21.910156 0.4375 21.359375 0.273438 20.796875 0.273438 C 19.179688 0.273438 17.867188 1.578125 17.863281 3.191406 C 17.863281 3.964844 18.171875 4.667969 18.667969 5.191406 C 18.066406 5.90625 17.582031 6.710938 17.234375 7.578125 C 16.972656 7.539062 16.707031 7.519531 16.441406 7.519531 C 13.554688 7.519531 11.203125 9.859375 11.203125 12.734375 C 11.203125 15.117188 12.816406 17.132812 15.015625 17.753906 C 13.910156 20.566406 11.160156 22.570312 7.941406 22.570312 C 5.199219 22.570312 2.792969 21.113281 1.457031 18.9375 C 1.046875 19.027344 0.625 19.074219 0.207031 19.082031 C 1.675781 21.84375 4.59375 23.726562 7.941406 23.726562 C 11.738281 23.726562 14.972656 21.308594 16.1875 17.945312 C 16.269531 17.949219 16.355469 17.949219 16.441406 17.949219 C 19.332031 17.949219 21.683594 15.609375 21.683594 12.734375 C 21.683594 10.535156 20.304688 8.648438 18.363281 7.882812 C 18.671875 7.148438 19.097656 6.464844 19.617188 5.859375 C 19.78125 5.933594 19.953125 5.988281 20.128906 6.03125 C 20.34375 6.078125 20.566406 6.109375 20.796875 6.109375 C 22.414062 6.105469 23.722656 4.800781 23.726562 3.191406 Z M 16.441406 16.792969 C 14.195312 16.792969 12.363281 14.972656 12.363281 12.734375 C 12.363281 10.496094 14.195312 8.675781 16.441406 8.675781 C 18.691406 8.675781 20.519531 10.496094 20.519531 12.734375 C 20.519531 14.972656 18.691406 16.792969 16.441406 16.792969 Z M 20.796875 1.429688 C 20.902344 1.429688 21.003906 1.441406 21.105469 1.460938 C 21.945312 1.609375 22.5625 2.335938 22.566406 3.191406 C 22.5625 4.164062 21.773438 4.953125 20.796875 4.953125 C 19.820312 4.953125 19.027344 4.164062 19.027344 3.191406 C 19.027344 2.21875 19.820312 1.433594 20.796875 1.433594 Z M 20.796875 1.429688"
            ></path>
        </svg>
    }
}

#[component]
pub fn IconCheckMark(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: OptionMaybeSignal<String>,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView {
    view! {
        <svg
        id=id
        class=class.get()
        style=style
        viewBox="0 0 24 24"
        xmlns="http://www.w3.org/2000/svg"
        >
            <path
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M22.903 2.828 20.075 0 6.641 13.435 3.102 9.09 0 11.616l6.338 7.779L22.903 2.828z"
            ></path>
        </svg>
    }
}
