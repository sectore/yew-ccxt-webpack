# `Yew` + `ccxt` + `Webpack` 

Original based on [`Yew`](https://github.com/yewstack/yew)'s ["npm_and_rest"](https://github.com/yewstack/yew/tree/master/examples/npm_and_rest) example. But instead of loading [`ccxt`](https://github.com/ccxt/ccxt/) from https://unpkg.com, it's bundled with `Webpack` using [`yew-wasm-pack-template`](https://github.com/yewstack/yew-wasm-pack-template). 

It might be helpfull for [yew/issues/682](https://github.com/yewstack/yew/issues/682).

```bash
# install depencencies
yarn
# build
yarn run build
# serve app locally
yarn run start:dev
```
