use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="font-fira-mono flex flex-col bg-stone-950 min-h-screen text-base text-center text-gray-300">
            <div class="flex-1 flex flex-col items-center">
                <div class="flex flex-1 md:pt-8 pt-6 md-8 md:pb-8 pb-12 md:w-full md:max-w-4xl justify-center">            
                    <Home/>
                </div>
                <Footer/>
            </div>
        </div>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center space-y-24">
            // Greeting
            <div class="md:space-y-8 space-y-6">
                <p class="tracking-widest md:text-2xl text-xl text-amber-200">"Hello, friend!"</p>
                <div class="md:w-full w-96 mx-8">
                    <p>"It looks like you've stumbled upon my personal website on the interwebs."</p>
                    <p>"It's still under construction, but feel free to explore."</p>
                </div>
            </div>
            // Main Content
            <div
                class="flex flex-grow mb-auto justify-center items-center"
            >
                <div class="flex md:flex-row flex-col justify-center items-center md:items-end md:space-x-12 space-x-0 md:space-y-0 space-y-12 transition-colors duration-200">
                    <a
                        class="flex flex-col items-center space-y-4 md:hover:text-orange-400 active:text-orange-400 transition-colors duration-200"
                        href="/blog"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 512 512">
                            <path fill="currentColor" d="M131.3 20.35c-14.6.1-28.1 10-31.93 24.82c-2.33 9.13-.55 18.4 4.13 25.84c-7.67 4.26-13.69 11.53-16.03 20.66c-2.32 9.13-.56 18.33 4.1 25.83a32.687 32.687 0 0 0-15.96 20.6c-2.34 9.1-.54 18.4 4.18 25.8c-7.72 4.3-13.75 11.5-16.09 20.7c-2.33 9.1-.54 18.4 4.19 25.8c-7.72 4.3-13.75 11.5-16.09 20.7c-2.34 9.1-.54 18.4 4.18 25.8c-7.72 4.3-13.75 11.5-16.08 20.7c-2.34 9.1-.54 18.4 4.18 25.8c-7.72 4.3-13.75 11.5-16.09 20.7c-2.35 9.2-.51 18.5 4.3 26a32.915 32.915 0 0 0-16.28 20.8c-4.48 17.5 6.25 35.6 23.79 40.1l.1-.2l31.71 8.2l-1.47 5.7l261.56 67L374 326.5l-22.4 21.2l-87.8 26.5l15.5-42.5l-151.7-38.8l4.4-17.4l153.5 39.3l9.7-26.7l15.3-14.4l-167-42.8l4.4-17.4l178 45.6l39.6-37.4l-206.1-52.8l4.4-17.4L380.7 207l-.1.4l31.5-29.8l18.3-71.4l-261.6-67.04l-4.8 18.66c2.2-16.32-8.1-32.27-24.5-36.44c-2.7-.7-5.5-1.04-8.2-1.03zm.3 17.99c1.2 0 2.4.19 3.5.48c8.1 2.09 12.9 10.13 10.8 18.27l17.2 4.4l-11 42.81c2.2-16.35-8.2-32.26-24.5-36.43l-.6-.15c-7.8-2.34-12.2-10.15-10.2-18.07c1.7-6.61 7.3-11 13.7-11.3h1.1zm-11.9 46.51c.9 0 1.9.14 2.9.36l.6.15c8.1 2.08 12.9 10.12 10.8 18.24l17.2 4.4l-11 43c2.4-16.4-8-32.6-24.4-36.7c-.7-.2-1.3-.4-1.9-.5c-7-2.7-10.9-10.1-9-17.62c1.7-6.97 7.9-11.45 14.8-11.29zm59.9 4.59l217 55.66l-4.4 17.4l-217-55.6zm-72.9 41.86h1.3c.5 0 .9 0 1.4.1c.6.2 1.2.3 1.8.5l.1-.2c8.1 2.1 12.9 10.1 10.8 18.3l17.2 4.4l-11 43c2.3-16.3-8.1-32.4-24.4-36.6c-8.18-2.1-12.94-10.1-10.85-18.3c1.69-6.6 7.25-10.9 13.65-11.2zM465.4 152l-10.2 9.6l31.6 33.5l10.2-9.6zm-23.3 22L315.7 293.5l31.5 33.5l126.5-119.5zm-347.23 3.7c1.48 0 3 .1 4.53.5c8.1 2.1 12.9 10.1 10.8 18.3l17.2 4.4l-11 43c2.3-16.4-8.1-32.4-24.44-36.6c-8.14-2.1-12.9-10.1-10.82-18.3c1.7-6.6 7.32-11 13.73-11.3zm-11.91 46.5c1.48 0 3 .1 4.53.5c8.14 2.1 12.91 10.1 10.81 18.3l17.2 4.4l-11 42.9c2.3-16.3-8.1-32.3-24.45-36.5c-8.14-2.1-12.89-10.1-10.81-18.3c1.69-6.6 7.31-11 13.72-11.3zm-11.9 46.5c1.48 0 3 .1 4.53.5c8.13 2.1 12.89 10.1 10.81 18.3l17.2 4.3l-10.94 42.8c2.16-16.3-8.25-32.1-24.51-36.3c-8.14-2.1-12.9-10.1-10.82-18.3c1.7-6.6 7.32-11 13.73-11.3zm235.34 39.2L293 346.6l37.4-11.3zm-247.25 7.3c1.48 0 3 .1 4.53.5c8.14 2.1 12.9 10.1 10.81 18.3l17.21 4.3l-11 43c2.1-16.2-8.3-32-24.53-36.2l.1-.3c-8.16-2.1-12.92-10.1-10.84-18.3c1.69-6.6 7.31-11 13.72-11.3zm56.95 20.3L333.2 393l-4.4 17.4l-217.1-55.5zM47.18 364c1.48 0 3 .1 4.52.5c8.14 2.1 12.9 10.1 10.82 18.3l17.2 4.3l-3.69 14.4l-31.92-8.2v.2c-8.01-2.2-12.67-10.1-10.61-18.2c1.7-6.6 7.32-11 13.73-11.3z"/>
                        </svg>
                        <div class="md:space-y-2 space-y-1">
                            <p class="tracking-wide md:w-64 md:text-3xl text-2xl">"Blog"</p>
                            <p>"personal blog"</p>
                        </div>
                    </a>
                    <a
                        class="flex flex-col items-center space-y-4 md:hover:text-orange-400 active:text-orange-400 transition-colors duration-200"
                        href="https://veredas.shuttleapp.rs/"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 512 512">
                            <path fill="currentColor" d="M212.125 20.156V59.72H39.53v128.84h30.316V293H39.623v190.03h165.064v-12.936H382v-45.5h95.281V264.53h-25.655V74.813H384.03V20.157H212.126zm18.688 18.688h134.53l-.03 35.968H297v79.126h81.28v46.625h18.69V135.25h-81.283V93.5h117.25v171.03H342v-63.06H224.656v9.343l-.03 70.187v9.344h75.467v-18.688h-56.75v-51.5h79.97l-.002 126.78l-185.937-.03V252.72h41.656v53.686h18.72V174.062h66.875v-18.687h-85.563v78.654h-60.375V293H88.53V188.562h57.44v-18.687H58.22v-91.47h153.905v30.94H111.437v18.686h119.375V38.845zM342 283.22h116.594v122.686H301.97v18.688h61.31v26.812H204.69v-48.812H110v18.687h76v43.064H58.312V311.72h60.376v53.874h9.343l114.782.03v65H261.5v-65H342v-31.529h66.75v33.844h18.688v-52.533H342V283.22z"/>
                        </svg>
                        <div class="md:space-y-2 space-y-1">
                            <p class="tracking-wide md:w-64 md:text-3xl text-2xl">"Veredas"</p>
                            <p>"notpron-like game"</p>
                        </div>
                    </a>
                    <a
                        class="flex flex-col items-center space-y-4 md:hover:text-orange-400 active:text-orange-400 transition-colors duration-200"
                        href="/why-rust"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24">
                            <path fill="currentColor" d="m21.9 11.7l-.9-.5V11l.7-.7c.1-.1.1-.3 0-.4l-.1-.1l-.9-.3c0-.1 0-.2-.1-.2l.6-.8c.1-.1.1-.3-.1-.4c0 0-.1 0-.1-.1l-1-.2c0-.1-.1-.1-.1-.2l.4-.9v-.3c-.1-.1-.2-.1-.3-.1h-1s0-.1-.1-.1l.2-1c0-.2-.1-.3-.2-.3h-.1l-1 .2c0-.1-.1-.1-.2-.2v-1c0-.2-.1-.3-.3-.3h-.1l-.9.4h-.1L16 3c0-.2-.2-.3-.3-.2h-.1l-.8.6c-.1 0-.2 0-.2-.1l-.3-.9c-.1-.1-.2-.2-.4-.2c0 0-.1 0-.1.1L13 3h-.2l-.5-.8c-.1-.2-.3-.2-.5-.2l-.1.1l-.5.9H11l-.7-.7c-.1-.1-.3-.1-.4 0l-.1.1l-.3.9c-.1 0-.2 0-.2.1l-.8-.6c-.2-.1-.4-.1-.5.1V3l-.2 1s-.1 0-.2.1l-.9-.4c-.1-.1-.3 0-.4.1v1.1c0 .1-.1.1-.1.2l-1-.2c-.2-.1-.3 0-.3.2v.1l.2 1c-.1 0-.1.1-.2.1h-1c-.2 0-.3.1-.3.3v.1l.4.9v.2L3 8c-.2 0-.3.2-.3.3v.1l.6.8c0 .1 0 .2-.1.2l-.8.4c-.1.1-.2.2-.2.4c0 0 0 .1.1.1l.7.7v.2l-.8.5c-.2.1-.2.3-.2.4l.1.1l.9.6v.2l-.7.7c-.1.1-.1.3 0 .4l.1.1l.9.3c0 .1 0 .2.1.2l-.6.8c-.1.1-.1.3.1.4c0 0 .1 0 .1.1l1 .2c0 .1.1.1.1.2l-.4.9c-.1.1 0 .3.1.4h1.1c.1 0 .1.1.2.1l-.2 1c0 .2.1.3.2.3h.1l1-.2c0 .1.1.1.2.2v1c0 .2.1.3.3.3h.1l.9-.4h.1l.2 1c0 .2.2.3.3.2h.1l.8-.6c.1 0 .2 0 .2.1l.3.9c.1.1.2.2.4.2c0 0 .1 0 .1-.1l.8-.7h.2l.5.8c.1.1.3.2.4.1l.1-.1l.5-.8h.2l.7.7c.1.1.3.1.4 0l.1-.1l.3-.9c.1 0 .2 0 .2-.1l.8.6c.1.1.3.1.4-.1c0 0 0-.1.1-.1l.2-1c.1 0 .1-.1.2-.1l.9.4c.1.1.3 0 .4-.1v-1.1l.2-.2l1 .2c.2 0 .3-.1.3-.2v-.1l-.2-1l.2-.2h1c.2 0 .3-.1.3-.3v-.1l-.4-.9c0-.1.1-.1.1-.2l1-.2c.2 0 .3-.2.2-.3v-.1l-.6-.8l.1-.2l.9-.3c.1-.1.2-.2.2-.4c0 0 0-.1-.1-.1L21 13v-.2l.8-.5c.2-.1.2-.3.1-.6c0 .1 0 .1 0 0m-5.7 7c-.3-.1-.5-.4-.5-.7c.1-.3.4-.5.7-.5c.3.1.5.4.5.7c0 .4-.3.6-.7.5m-.2-1.9c-.3-.1-.6.1-.6.4l-.4 1.4c-.9.4-1.9.6-3 .6s-2.1-.2-3.1-.7l-.3-1.4c-.1-.3-.3-.5-.6-.4l-1.2.3c-.2-.2-.4-.5-.6-.7h6c.1 0 .1 0 .1-.1v-2.1c0-.1 0-.1-.1-.1h-1.7v-1.3h1.9c.2 0 .9 0 1.2 1c.1.3.2 1.3.4 1.6c.1.3.6 1 1.1 1h3.1c-.2.3-.4.5-.7.8l-1.5-.3m-8.3 1.9c-.3.1-.6-.1-.7-.5c-.1-.3.1-.6.5-.7s.6.1.7.5c0 .3-.2.6-.5.7M5.4 9.5c.1.3 0 .7-.3.8c-.3.1-.7 0-.8-.3c-.1-.3 0-.7.3-.8c.4-.1.7 0 .8.3m-.7 1.6l1.3-.5c.3-.1.4-.4.3-.7L6 9.3h1V14H5c-.3-1-.4-1.9-.3-2.9m5.6-.4V9.3h2.5c.1 0 .9.1.9.7c0 .5-.6.7-1.1.7h-2.3m9 1.2v.5h-.8c-.1 0-.1 0-.1.1v.3c0 .8-.5 1-.9 1s-.8-.2-.9-.4c-.2-1.3-.6-1.5-1.2-2c.7-.5 1.5-1.2 1.5-2.1c0-1-.7-1.6-1.1-1.9c-.7-.4-1.4-.5-1.6-.5H6.6c1.1-1.2 2.5-2 4.1-2.3l.9 1c.2.2.5.2.8 0l1-1c2.1.4 3.9 1.7 5 3.6l-.7 1.6c-.1.3 0 .6.3.7l1.3.6v.8m-7.7-8c.2-.2.6-.2.8 0c.2.2.2.6 0 .8c-.3.3-.6.3-.9 0c-.2-.2-.1-.5.1-.8m6.9 5.6c.1-.3.5-.4.8-.3c.3.1.4.5.3.8c-.1.3-.5.4-.8.3c-.3-.1-.4-.5-.3-.8Z"/>
                        </svg>
                        <div class="md:space-y-2 space-y-1">
                            <p class="tracking-wide md:w-64 md:text-3xl text-2xl">"Why Rust"</p>
                            <p>"awesome articles"</p>
                        </div>
                    </a>
                </div>
            </div>
            // Contact
            <div class="md:mb-16 mb-10 md:w-96">
                <h3 class="md:text-lg text-base">"Socials"</h3>
                <div class="mt-4 flex flex-row justify-around items-center space-x-6">
                    <a href="https://github.com/lpnh" target="_blank" class="justify-center items-center flex flex-col space-y-2 md:hover:text-rose-400 active:text-rose-400 transition-colors duration-200">
                        <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24">
                            <path fill="currentColor" d="M12 2A10 10 0 0 0 2 12c0 4.42 2.87 8.17 6.84 9.5c.5.08.66-.23.66-.5v-1.69c-2.77.6-3.36-1.34-3.36-1.34c-.46-1.16-1.11-1.47-1.11-1.47c-.91-.62.07-.6.07-.6c1 .07 1.53 1.03 1.53 1.03c.87 1.52 2.34 1.07 2.91.83c.09-.65.35-1.09.63-1.34c-2.22-.25-4.55-1.11-4.55-4.92c0-1.11.38-2 1.03-2.71c-.1-.25-.45-1.29.1-2.64c0 0 .84-.27 2.75 1.02c.79-.22 1.65-.33 2.5-.33c.85 0 1.71.11 2.5.33c1.91-1.29 2.75-1.02 2.75-1.02c.55 1.35.2 2.39.1 2.64c.65.71 1.03 1.6 1.03 2.71c0 3.82-2.34 4.66-4.57 4.91c.36.31.69.92.69 1.85V21c0 .27.16.59.67.5C19.14 20.16 22 16.42 22 12A10 10 0 0 0 12 2Z"/>
                        </svg>
                        <p class="md:text-base text-sm">"github"</p>
                    </a>
                    <a href="https://www.linkedin.com/in/lpnh/" target="_blank" class="justify-center items-center flex flex-col space-y-2 md:hover:text-rose-400 active:text-rose-400 transition-colors duration-200">
                        <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24">
                            <path fill="currentColor" d="M19 3a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14m-.5 15.5v-5.3a3.26 3.26 0 0 0-3.26-3.26c-.85 0-1.84.52-2.32 1.3v-1.11h-2.79v8.37h2.79v-4.93c0-.77.62-1.4 1.39-1.4a1.4 1.4 0 0 1 1.4 1.4v4.93h2.79M6.88 8.56a1.68 1.68 0 0 0 1.68-1.68c0-.93-.75-1.69-1.68-1.69a1.69 1.69 0 0 0-1.69 1.69c0 .93.76 1.68 1.69 1.68m1.39 9.94v-8.37H5.5v8.37h2.77Z"/>
                        </svg>
                        <p class="md:text-base text-sm">"linkedin"</p>
                    </a>
                    <a href="https://discordapp.com/users/371617418285416448" target="_blank" class="justify-center items-center flex flex-col space-y-2 md:hover:text-rose-400 active:text-rose-400 transition-colors duration-200">
                        <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24">
                            <path fill="currentColor" d="m22 24l-5.25-5l.63 2H4.5A2.5 2.5 0 0 1 2 18.5v-15A2.5 2.5 0 0 1 4.5 1h15A2.5 2.5 0 0 1 22 3.5V24M12 6.8c-2.68 0-4.56 1.15-4.56 1.15c1.03-.92 2.83-1.45 2.83-1.45l-.17-.17c-1.69.03-3.22 1.2-3.22 1.2c-1.72 3.59-1.61 6.69-1.61 6.69c1.4 1.81 3.48 1.68 3.48 1.68l.71-.9c-1.25-.27-2.04-1.38-2.04-1.38S9.3 14.9 12 14.9s4.58-1.28 4.58-1.28s-.79 1.11-2.04 1.38l.71.9s2.08.13 3.48-1.68c0 0 .11-3.1-1.61-6.69c0 0-1.53-1.17-3.22-1.2l-.17.17s1.8.53 2.83 1.45c0 0-1.88-1.15-4.56-1.15m-2.07 3.79c.65 0 1.18.57 1.17 1.27c0 .69-.52 1.27-1.17 1.27c-.64 0-1.16-.58-1.16-1.27c0-.7.51-1.27 1.16-1.27m4.17 0c.65 0 1.17.57 1.17 1.27c0 .69-.52 1.27-1.17 1.27c-.64 0-1.16-.58-1.16-1.27c0-.7.51-1.27 1.16-1.27Z"/>
                        </svg>
                        <p class="md:text-base text-sm">"discord"</p>
                    </a>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="pb-0 m-auto left-0 right-0">
            <div class="text-xs">"© 2023 lpnh"</div>
        </footer>
    }
}