use crate::i18n::*;
use leptos::context::Provider;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use some_crate::i18n::Locale as SubLocale;
use some_crate::SomeTranslatedComponent;

#[component]
fn I18nProvider(children: TypedChildren<impl IntoView + 'static>) -> impl IntoView {
    let i18n = leptos_i18n::context::init_i18n_context::<Locale>();
    let mapped_locale: Signal<SubLocale> = Memo::new(move |_| match i18n.get_locale() {
        Locale::en => SubLocale::en,
        Locale::de => SubLocale::de,
    })
    .into();
    let sub_context = leptos_i18n::context::init_i18n_subcontext(Some(mapped_locale));
    let children = children.into_inner();
    view! {
      <Provider value=i18n>
        <Provider value=sub_context>
          {children()}
        </Provider>
      </Provider>
    }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/main-proj.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <I18nProvider>
            <Router>
                <main>
                    <Routes fallback=|| "Page not found.".into_view()>
                        <Route path=StaticSegment("") view=HomePage/>
                    </Routes>
                </main>
            </Router>
        </I18nProvider>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let i18n = use_i18n();

    let switch_locale = move |_| {
        let new_locale = match i18n.get_locale_untracked() {
            Locale::en => Locale::de,
            Locale::de => Locale::en,
        };
        i18n.set_locale(new_locale);
    };

    view! {
        <h1>"Test"</h1>

        <SomeTranslatedComponent />

        <button on:click=switch_locale>{t!(i18n, switch_locale)}</button>
    }
}
