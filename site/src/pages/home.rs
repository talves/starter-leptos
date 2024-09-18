use leptos::*;
use leptos_meta::*;
use leptos_tw_ui::components::container::Main;

use crate::sections::hero::Hero;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
      <Title text="Starter | Leptos + Tailwindcss"/>
      <Main id="content" class="max-w-[85rem] px-4 py-4 sm:px-6 lg:px-8 mx-auto">
              <div class="">
                <Hero/>
              </div>
      </Main>
      // An empty Fragment is not created on the DOM
      {Fragment::new(vec![])}
    }
}
