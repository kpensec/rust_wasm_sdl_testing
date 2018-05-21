// create canvas and bind it to cargo web

var Module = {};
var __cargo_web = {};
Object.defineProperty( Module, 'canvas', {
  get: function() {
    if( __cargo_web.canvas ) {
      return __cargo_web.canvas;
    }
    var canvas = document.createElement( 'canvas' );
    document.querySelector( 'body' ).appendChild( canvas );
    __cargo_web.canvas = canvas;
    return canvas;
  }
});

