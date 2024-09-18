use leptos_tw_ui::components::{
    theme::toggle::ThemeToggleSwitchClass, variants::base::ClassVariant,
};

pub enum ButtonVariant {
    Solid,
    Outline,
    Ghost,
    Soft,
    White,
    Link,
}

impl ButtonVariant {
    pub fn get(&self) -> ClassVariant {
        const SOLID: &'static [&'static str] = &[
            "px-4",
            "py-3",
            "justify-center",
            "items-center",
            "rounded-md",
            "border",
            "border-transparent",
            "bg-primary",
            "text-white",
            "hover:bg-opacity-50",
            "focus:outline-none",
            "transition-all",
            "text-md",
            "dark:focus:ring-offset-gray-800",
        ];
        const OUTLINE: &'static [&'static str] = &[
            "px-4",
            "py-3",
            "justify-center",
            "items-center",
            "rounded-md",
            "border-2",
            "border-gray-200",
            "font-semibold",
            "text-btn",
            "dark:text-white",
            "hover:text-white",
            "hover:bg-primary",
            "focus:outline-none",
            "transition-all",
            "text-sm",
            "dark:border-gray-700",
            "dark:hover:border-primary-500",
        ];
        const GHOST: &'static [&'static str] = &[
            "px-4",
            "py-3",
            "justify-center",
            "items-center",
            "rounded-md",
            "border",
            "border-transparent",
            "font-semibold",
            "text-btn",
            "hover:text-primary",
            "dark:text-white",
            "dark:hover:text-primary",
            "transition-all",
            "text-lg",
        ];
        const LINK: &'static [&'static str] = &[
            "px-4",
            "py-3",
            "justify-center",
            "items-center",
            "rounded-md",
            "transition-all",
            "text-lg",
            "bg-primary",
            "bg-opacity-70",
            "dark:bg-btn",
            "rounded-xl",
            "text-white",
            "hover:text-primary",
        ];
        match self {
            ButtonVariant::Solid => ClassVariant::Vec(SOLID),
            ButtonVariant::Outline => ClassVariant::Vec(OUTLINE),
            ButtonVariant::Ghost => ClassVariant::Vec(GHOST),
            ButtonVariant::Soft => ClassVariant::Str("p-3 justify-center items-center rounded-md bg-blue-100 border border-transparent font-semibold text-blue-500 hover:text-white hover:bg-blue-500 focus:outline-none focus:ring-2 ring-offset-white focus:ring-blue-500 focus:ring-offset-2 transition-all text-sm"),
            ButtonVariant::White => ClassVariant::Str("p-3 justify-center items-center rounded-md border font-medium bg-white text-gray-700 shadow-sm align-middle hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-white focus:ring-blue-600 transition-all text-sm dark:bg-slate-900 dark:hover:bg-slate-800 dark:border-gray-700 dark:text-gray-400 dark:hover:text-white dark:focus:ring-offset-gray-800"),
            ButtonVariant::Link => ClassVariant::Vec(LINK),
        }
    }
}

pub enum MenuHeaderVariant {
    Default,
}

impl MenuHeaderVariant {
    pub fn get(&self) -> ClassVariant {
        const DEFAULT: &'static [&'static str] = &[
            "flex",
            "flex-wrap",
            "sm:justify-start",
            "sm:flex-nowrap",
            "z-50",
            "w-full",
            "bg-white",
            "border-b",
            "text-lg",
            "py-2.5",
            "sm:py-4",
            "dark:bg-secondary",
            "dark:border-slate-900",
        ];
        match self {
            MenuHeaderVariant::Default => ClassVariant::Vec(DEFAULT),
        }
    }
}

pub enum MenuBarVariant {
    Default,
}

impl MenuBarVariant {
    pub fn get(&self) -> ClassVariant {
        const DEFAULT: &'static [&'static str] = &[
            "max-w-[85rem]",
            "w-full",
            "mx-auto",
            "px-4",
            "sm:flex",
            "sm:items-center",
            "sm:justify-between",
            "sm:px-6",
            "lg:px-8",
        ];
        match self {
            MenuBarVariant::Default => ClassVariant::Vec(DEFAULT),
        }
    }
}

pub struct ThemePageClass {
    pub wrapper: &'static str,
}

pub fn default_page_class() -> ThemePageClass {
    ThemePageClass {
        wrapper: "font-roboto bg-btn dark:bg-secondary",
    }
}

pub fn default_switch_class() -> ThemeToggleSwitchClass {
    ThemeToggleSwitchClass {
        wrapper: "relative inline-flex h-[24px] w-[34px] shrink-0 cursor-pointer border-2 border-transparent focus:outline-none focus-visible:ring-2 focus-visible:ring-white focus-visible:ring-opacity-75",
        switch: "translate-x-0 dark:translate-x-2.5 shadow shadow-gray-700/10 border border-gray-200 bg-gray-100 dark:border-primary dark:bg-gray-700 p-[3px] pointer-events-none inline-block h-5 w-5 transform rounded-full ring-0 transition-transform duration-200 ease-in-out",
        bar: "bg-gray-300/60 dark:bg-white-300/10 rounded-full absolute left-0 right-0 h-[0.65rem] top-1/2 translate-y-[-50%]",
        sun_fill: "fill-yellow-600",
        moon_fill: "fill-yellow-400",
        sun: "dark:hidden",
        moon: "hidden dark:block",
    }
}
pub enum ToggleSwitchClassVariant {
    Knob,
    Encased,
}

impl ToggleSwitchClassVariant {
    pub fn get(&self) -> ThemeToggleSwitchClass {
        match self {
            ToggleSwitchClassVariant::Knob => default_switch_class(),
            ToggleSwitchClassVariant::Encased => {
                let mut encased = default_switch_class();
                encased.switch = "translate-x-0 dark:translate-x-2.5 shadow shadow-gray-700/10 border border-gray-200 bg-gray-100 dark:border-primary dark:bg-gray-700 p-[3px] pointer-events-none inline-block h-5 w-5 transform rounded-full ring-0 transition-transform duration-300 ease-in-out";
                encased.bar = "bg-gray-300/60 dark:bg-gray-200/40 rounded-full absolute left-0 right-0 h-6 top-1/2 translate-y-[-50%] ring-1";
                encased
            }
        }
    }
}

pub enum TypographyClass {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Paragraph,
    Span,
    Code,
}

impl TypographyClass {
    pub fn get(&self) -> ClassVariant {
        const H1: &'static [&'static str] = &[
            "font-weight-20",
            "text-3xl",
            "text-primary",
            "dark:text-gray-200",
        ];
        const H2: &'static [&'static str] = &[
            "font-weight-20",
            "text-2xl",
            "text-primary",
            "dark:text-gray-200",
        ];
        const H3: &'static [&'static str] = &[
            "font-weight-20",
            "text-xl",
            "text-primary",
            "dark:text-gray-200",
        ];
        const H4: &'static [&'static str] = &[
            "font-weight-20",
            "text-md",
            "text-primary",
            "dark:text-gray-200",
        ];
        const H5: &'static [&'static str] = &[
            "font-weight-20",
            "text-sm",
            "text-primary",
            "dark:text-gray-200",
        ];
        const H6: &'static [&'static str] = &[
            "font-weight-20",
            "text-xs",
            "text-primary",
            "dark:text-gray-200",
        ];
        const P: &'static [&'static str] = &["text-primary", "dark:text-gray-200"];
        const SPAN: &'static [&'static str] = &["flex-none", "text-primary", "dark:text-gray-200"];
        const CODE: &'static [&'static str] = &[
            "text-sm",
            "sm:text-base",
            "text-left",
            "items-center",
            "bg-gray-800",
            "dark:bg-gray-700",
            "text-white",
            "rounded-lg",
            "p-4",
            "pl-6",
        ];

        match self {
            TypographyClass::H1 => ClassVariant::Vec(H1),
            TypographyClass::H2 => ClassVariant::Vec(H2),
            TypographyClass::H3 => ClassVariant::Vec(H3),
            TypographyClass::H4 => ClassVariant::Vec(H4),
            TypographyClass::H5 => ClassVariant::Vec(H5),
            TypographyClass::H6 => ClassVariant::Vec(H6),
            TypographyClass::Paragraph => ClassVariant::Vec(P),
            TypographyClass::Span => ClassVariant::Vec(SPAN),
            TypographyClass::Code => ClassVariant::Vec(CODE),
        }
    }
}
