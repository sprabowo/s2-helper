# S2 helper wasm

simple s2 helper run on browser through wasm to do such s2 cell id calculation given latitude, longitude, level and reverse s2 cell id to coordinates

## Modules (vite preferred)

### Instal
```sh
npm install s2-helper-wasm
```

### Usage
```js
import init, { calculate } from 's2-helper-wasm'

init().then(() => {
  const cellID = calculate(-6.228968465405475, 106.8071658857885, 13) // "3344469575738589184"
})
```

## Non modules

### Build
```
wasm-pack build --target no-modules
# copy pkg/s2_helper_wasm.js, pkg/s2_helper_wasm_bg.wasm to /public
cp pkg/s2_helper_wasm.js public
cp pkg/s2_helper_wasm_bg.wasm public

# in your index.html
<script src="/s2_helper_wasm.js"></script>
<script>
  const { calculate } = wasm_bindgen

  async function init() {
    await wasm_bindgen()
    const cellID = calculate(-6.228968465405475, 106.8071658857885, 13)
  }

  init()
</script>
```