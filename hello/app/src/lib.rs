use crate::error_template::{AppError, ErrorTemplate};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use logging::log;

pub mod error_template;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[server]
pub async fn print_on_server(content: String) -> Result<i32, ServerFnError> {
    println!("{}", content);
    Ok(1)
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    // let on_click = move |_| set_count.update(|count| *count += 1);

    let clj = move |_| {
        spawn_local(async move {
            let res = print_on_server("hello world".to_owned()).await.unwrap();
            log!("server says I should incr by {}", res);
            set_count.update(|count| *count += res);
            ()
        });
    };

    view! {
        <Script src="https://cdn.tailwindcss.com" />
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=clj>"Click Me: " {count}</button>
    }
}
