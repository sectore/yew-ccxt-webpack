// Make `ccxt` accessible in `cctx.rs`
window.ccxt = require('ccxt');

import("./pkg").then(module => {
  module.run_app();
});
