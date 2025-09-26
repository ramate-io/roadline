# Summary
To run a development server:

```shell
dx serve -p roadline-site-dioxus-bevy --bin roadline-site-dioxus-bevy --platform web
```

> [!NOTE]
> This wraps in a build script which ensures the Bevy application is built and bundled as described [here](https://bevy-cheatbook.github.io/platforms/wasm/webpage.html).
>
> ```
> cargo build --release --target wasm32-unknown-unknown
> wasm-bindgen --no-typescript --target web \
>    --out-dir ./out/ \
>    --out-name "mygame" \
>    ./target/wasm32-unknown-unknown/release/mygame.wasm
> ```
> 
> In our case this build script will need to use the [`roadline-bevy-for-canvas`](/core/renderer/bevy/bin/roadline-bevy-for-canvas.rs) target.

To bundle, build, and ship, see [Dioxus Learn: Deploy](https://dioxuslabs.com/learn/0.6/guide/deploy).