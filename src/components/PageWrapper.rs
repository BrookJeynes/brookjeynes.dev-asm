use leptos::*;

#[component]
pub fn PageWrapper(cx: Scope, title: String, children: Children) -> impl IntoView {
    view! {
        cx,
        <div class="pt-20 mb-10 mx-auto max-w-[90ch] text-white">
            <h1 class="text-4xl font-semibold mb-16 text-center">
                <span class="text-green">"❯ "</span>"./"{title}" "
                <span class="animate-cursor-blink">"|"</span>
            </h1>

            <div class="">
                {children(cx)}
            </div>

            <div class="text-center">
                <p class="text-blue">"brookjeynes.dev"</p>
            </div>
        </div>
    }
}
