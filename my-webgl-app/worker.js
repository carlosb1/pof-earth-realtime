importScripts("./pkg/my_webgl_app.js");

console.log("Initializing worker");

// In the worker, we have a different struct that we want to use as in
// `index.js`.
const { MyWorker } = wasm_bindgen;

async function init_wasm_in_worker() {
  await wasm_bindgen("./pkg/my_webgl_app_bg.wasm");

  var my_worker = MyWorker.new();

  self.onmessage = async (event) => {
    var worker_result = my_worker.hello_world("john");
    console.log("please");

    // Send response back to be handled by callback in main thread.
    //self.postMessage(worker_result);
  };
}

init_wasm_in_worker();
