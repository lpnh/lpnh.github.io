# taileptrun

A CSR Web App template using TailwindCSS, Leptos and Trunk. 🦀

## Leptos

### cargo-leptos

Install Leptos' [build tool](https://github.com/leptos-rs/cargo-leptos).

```no_rust
cargo install cargo-leptos
```

Start with a new template:

```no_rust
cargo leptos new --git https://github.com/leptos-rs/start
```

### Compiling to WebAssembly

To be able to compile the code for WebAssembly:

```no_rust
rustup target add wasm32-unknown-unknown
```

To be able to use the `nightly` Rust version:

```no_rust
rustup toolchain install nightly
```

## Trunk

### Installing

Install [Trunk](https://github.com/trunk-rs/trunk) using cargo:

```no_rust
cargo install trunk
```

## Tailwind (standalone CLI)

### Installation

Get the [latest release](https://github.com/tailwindlabs/tailwindcss/releases/latest) and give it executable permissions.

Example for Linux:

```no_rust
curl -LO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64
chmod +x tailwindcss-linux-x64
mv tailwindcss-linux-x64 tailwindcss
```

### Usage

Creating a tailwind.config.js file:

```no_rust
./tailwindcss init
```

Starting a watcher:

```no_rust
./tailwindcss -i input.css -o output.css --watch
```

Compiling and minifying your CSS for production:

```no_rust
./tailwindcss -i input.css -o style/output.css --minify
```

## Setup

### Assets

Create new `public` and `style` directories:

```no_rust
mkdir public
mkdir style
```

The former for storing favicon, etc. The latter for the tailwind css file output.

### Tailwind

Create the `tailwind.config.js` file, using the tailwind CLI:

```no_rust
./tailwindcss init
```

Update it to accept both `.html` and `.rs` files:

```no_rust
/** @type {import('tailwindcss').Config} */
module.exports = {
    content: { 
        files: ["*.html", "./src/**/*.rs"],
    },
    theme: {
        extend: {},
        fontFamily: {
            'fira-mono': ['"Fira Mono"', 'monospace'],
        },
    },
    plugins: [],
}

```

*Note: the current template also includes the Fira Mono font.*

Add new `input.css` file on the project's root:

```css
@import url('https://fonts.googleapis.com/css2?family=Fira+Mono&display=swap');
@tailwind base;
@tailwind components;
@tailwind utilities;
```

### Trunk

Add the following `index.html` file in the project's root:

```
<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="utf-8">
    <link data-trunk rel="rust" data-wasm-opt="z" />
    <link data-trunk rel="icon" type="image/ico" href="/public/favicon.ico" />
    <link data-trunk rel="css" href="/style/output.css" />
    <title>Leptos • Counter with Tailwind</title>
</head>

<body></body>

</html>
```

Add a `Trunk.toml` file:

```toml
[[hooks]]
stage = "pre_build"
command = "sh"
command_arguments = ["-c", "./tailwindcss -i input.css -o style/output.css"]
```

You can see a `Trunk.toml` config file example [here](https://github.com/trunk-rs/trunk/blob/main/Trunk.toml)

## Run

Install `cargo-make`:

```no_rust
sudo pacman -S cargo-make
```

Run in development mode:

```no_rust
cargo make run-dev
```