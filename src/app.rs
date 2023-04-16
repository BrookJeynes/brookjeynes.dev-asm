use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::Home::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <div class="m-0 p-0 h-screen bg-black">
            <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
            <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
            <Router>
                <Routes>
                    <Route path="" view=  move |cx| view! { cx, <Home /> }/>
                </Routes>
            </Router>
        </div>
    }
}
