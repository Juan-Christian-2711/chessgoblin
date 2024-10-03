# chessgoblin
chessgoblin is a chess game analysis tool.

It uses Svelte Kit for the frontend and Rust for the backend using tauri.

## Developing
First change the directory to /src

```bash
cd src
```

Once you've installed dependencies with `npm install`, start a development server:

```bash
npm run tauri dev
```

## Building

To create a production version of chessgoblin

```bash
npm run build
```

You can preview the production build with `npm run preview`.

> To deploy chessgoblin, you may need to install an [adapter](https://kit.svelte.dev/docs/adapters) for your target environment.
