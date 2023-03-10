use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="2" view=|cx| view! { cx, <HomePage2/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let inc = move |_| set_count.update(|count| *count -= 1);
    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=inc>"Click Me: " {count}</button>
    }
}

#[component]
fn HomePage2(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let inc = move |_| set_count.update(|count| *count -= 1);
    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=inc>"Dont Click Me: " {count}</button>
    }
}