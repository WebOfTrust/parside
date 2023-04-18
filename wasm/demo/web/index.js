// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
const wasm = import('parside-wasm');

wasm
  .then(parside => {
    result = parside.Message.from_stream_bytes([
       45,  65,  65,  66,  65,  65,  66, 103,  51, 113,  56, 117,
       78, 103,  49,  65,  50, 106, 104,  69,  65, 100,  98,  75,
       71, 102,  45,  81, 117, 112,  81, 104,  78, 110, 109,  90,
       81, 120,  51, 122,  73, 121,  80,  76,  87,  66, 101,  54,
      113, 113,  76,  84,  53, 121, 110, 121, 116, 105, 118, 102,
       57,  69, 119,  74, 104, 120, 121, 104, 121,  56,  55,  97,
       48, 120,  50,  99, 101, 122,  68, 100, 105, 108,  52,  83,
      115,  77,  50, 120, 120, 115,  48,  79
    ])
    document.write("<p>Message parsed</p>");
    document.write("<p>ControllerIdxSigs:</p>");
    document.write(result.message.group.controllerIdxSigs.qb64());
  })
  .catch(console.error);
