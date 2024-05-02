project template with actix clone with

for an Actix template without tailwind
cargo leptos new --git leptos-rs/start

# Tailwind setup

Create a tailwind.config.js file

```bash
pnpm dlx tailwindcss init
```

Modify content key in the tailwind.config.js

```js
 module.exports = {
   content: {
       files: ["*.html", "./src/**/*.rs"],
   },
   ...
 }
```

Create an input.css file in the root directory

```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

Change the style file in Cargo.toml

```js
style-file = "style/output.css"
```

Run the Tailwind process in a separate terminal:

```bash
pnpm dlx tailwindcss -i ./input.css -o ./style/output.css --watch
```

you can use "npx" instead "pnpm dlx" if you like

Done! 🥳
