import wasm from '@rollup/plugin-wasm'
import rust from "rollup-plugin-rust";
export default {
  input: 'index.js',
  output: {
    file: 'dist/bundle.js',
    format: 'cjs'
  },
  plugins: [
    rust(),
    // wasm(),
  ],
}