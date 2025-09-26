# Summary
This is a plugin for rendering Bevy applications in a Dioxus application. It is based on [`TauriPlugin`](https://github.com/sunxfancy/BevyTauriExample/blob/main/src-tauri/src/tauri_plugin.rs). 

> [!NOTE]
> We are first going to try the simple route through using a basic [`index.html`] with a targeted canvas, as referenced [here](https://github.com/DioxusLabs/dioxus/discussions/2880) wherein we add we [use](https://bevy-cheatbook.github.io/platforms/wasm/webpage.html) the custom `assets/ index.html mygame.js mygame_bg.wasm` to prepare the game ahead. 

> [!TIP]
> For the reverse problem, attempting to embed Dioxus applications within Bevy, Dioxus has begun working on a renderer called [Blitz](https://github.com/DioxusLabs/blitz).