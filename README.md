<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Leptos Axum Workspace

This is a template for use with the [Leptos](https://github.com/leptos-rs/leptos) web framework and the [cargo-leptos](https://github.com/akesson/cargo-leptos) tool using [Axum](https://github.com/tokio-rs/axum).

## Run it

Install npm packages:

```bash
npm install -D tailwindcss
```

Start leptos server

```bash
cargo leptos watch
```

## Development:

Start the server:

```bash
cargo leptos watch
```

Compile tailwindcss:

```bash
npm run watch
```

### Set Colorscheme:

If you want to change the color, you can modified `input.css` file and
`tailwind.config.js`

Install tailwindcss and these plugins set at `tailwind.config.js`, for my case:

```bash
npm install -D tailwindcss
npm install -D @catppuccin/tailwindcss
```

Compile it:

```bash
npx tailwindcss -i ./tailwind.css -o ./style/output.css
```

or:

```bash
npm run watch
```

## TODO
