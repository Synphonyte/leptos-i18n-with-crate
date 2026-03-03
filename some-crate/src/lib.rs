use leptos::prelude::*;

include!(concat!(env!("OUT_DIR"), "/i18n/mod.rs"));

use crate::i18n::*;

#[component]
pub fn SomeTranslatedComponent() -> impl IntoView {
    let i18n = use_i18n_scoped!(some_crate);

    view! {
        <button>{t!(i18n, new)}</button>
    }
}
