Setup:
- Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- `cd www && npm link ../pkg`

Development:
- `cd www && npm install && npm run start`
- `find src/ | entr wasm-pack build`

Release:
