use leptos::*;

#[allow(clippy::module_name_repetitions)]
#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="font-fira-mono flex flex-col bg-stone-950 min-h-screen text-base text-center text-gray-300">
            <div class="flex-1 flex flex-col items-center">
                <div class="flex flex-1 md:pt-8 pt-6 md-8 md:pb-8 pb-12 md:w-full md:max-w-4xl justify-center">
                    <Home/>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center">
            // Greeting
            <div class="flex flex-col md:space-y-10 space-y-8">
                <h1 class="tracking-widest md:text-xl text-lg text-rose-300">"hello, friend!"</h1>
                <div class="flex flex-col px-10 md:space-y-4 space-y-2">
                    <p>"it looks like you've stumbled upon my personal website"</p>
                    <p>"i'm currently rethinking its purpose, so there’s not much to see here for now"</p>
                    <p>"but hey, I can’t let you leave empty-handed, so here’s a puppy for you"</p>
                    <p>"hope you have a great day"</p>
                </div>
            </div>
            // Main Content
            <div
                class="flex flex-grow mb-auto justify-start items-center"
            >
                <img class="w-96 aspect-square object-cover m-auto rounded-xl" src="puppy.jpg" />
            </div>
            <p class="m-auto md:w-full w-72 text-xs opacity-40">"photo by "<a href="https://unsplash.com/@joshuadan?utm_content=creditCopyText&utm_medium=referral&utm_source=unsplash">"JOSHUA DANIEL"</a>" on "<a href="https://unsplash.com/photos/yellow-labrador-retriever-puppy-on-clear-glass-bowl-RO6GhNH14jg?utm_content=creditCopyText&utm_medium=referral&utm_source=unsplash">"Unsplash"</a></p>
        </div>
    }
}
