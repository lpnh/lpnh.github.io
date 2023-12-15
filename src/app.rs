use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="font-fira-mono flex flex-col bg-gray-100 min-h-screen text-base text-center text-gray-900">
            <div class="flex flex-col flex-1 items-center">
                <Header/> 
                    <div class="flex flex-1 sm:pt-4 pt-2 sm:pb-8 pb-12 md:w-full md:max-w-4xl mx-6 justify-center items-center">
                        <Router>
                            <Routes>
                                <Route path="" view=  move || view! { <Home/> }/>
                            </Routes>
                        </Router>
                    </div>
                <Footer/>
            </div>
        </div>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <Title text="Home" />
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h1 class="p-6 text-3xl font-bold">"Leptos + Trunk + Tailwind"</h1>
            <p class="text-lg">"This is a CSR Web Application Template made with Leptos, Trunk and Tailwind"</p>
        </div>
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <div class="tracking-widest mt-2 flex justify-center items-center inset-x-0 top-0 bottom-auto">"Hello, friend!"</div>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <div class="pb-0 m-auto left-0 right-0">
            <div class="text-xs">"© 2023 lpnh"</div>
        </div>
    }
}