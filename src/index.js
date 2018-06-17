// import {example} from './game/target/wasm32-unknown-emscripten/debug/rust-app.js'
//import './game/target/wasm32-unknown-emscripten/debug/rust-app.js'
//console.log("Hello World!");
// require('rust-emscripten-loader?target=wasm&outName=dist/out!./game/src/main.rs');
require('./game/target/wasm32-unknown-emscripten/release/rust_app.wasm');

//const script = document.createElement('script');
//script.text = `
//  if (typeof WebAssembly === 'object') {
//    var Module = {};
//    var req = new XMLHttpRequest();
//    req.open('GET', 'dist/out.wasm');
//    req.responseType = 'arraybuffer';
//    req.send();
//    req.onload = function() {
//      Module.wasmBinary = req.response;
//      var script = document.createElement('script');
//      script.src = 'dist/out.js';
//      document.body.appendChild(script);
//    };
//  } else {
//    document.getElementById('container').innerHTML = 'Your browser doesn\\'t support WebAssembly!';
//  }
//`;
//document.body.appendChild(script);
