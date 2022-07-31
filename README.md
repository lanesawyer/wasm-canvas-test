# WASM Canvas Test

This is a small experiment to compare the differences in speed between WASM and JS when drawing items on a `canvas`.

On my personal machine, I was getting the full WASM page to load in about 250 ms, while the JS one was around 290 ms. Not a huge difference, and maybe I have subtle differences in the code that I'm not seeing, but I was surprised WASM beat it!

## How to Test

1. Run `trunk build` to get a `dist` folder
2. Run `npx simple-server dist` to serve the WASM code folder
3. Run `npx simple-server .` to serve the JS items
4. Pull up each page
5. Open the network tab
6. Refresh and compare!
