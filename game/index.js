// import {BoardW} from '../crumble-wasm/src/lib.rs'
const {BoardW} = require('../../crumble-wasm/pkg')

const board = BoardW.new_starting()

console.log(board.pieces())