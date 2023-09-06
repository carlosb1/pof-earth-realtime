importScripts("./pkg/my_webgl_app.js");

console.log("Initializing worker");

// In the worker, we have a different struct that we want to use as in
// `index.js`.

async function init_wasm_in_worker() {
  self.onmessage = async (event) => {
    await wasm_bindgen("./pkg/my_webgl_app_bg.wasm");
    const { MyWorker } = wasm_bindgen;
    var my_worker = MyWorker.new();

    var hello = my_worker.hello_world("john");

    console.log("please ");
    console.log(hello);

    // Send response back to be handled by callback in main thread.
    self.postMessage("returns message");
  };
}

init_wasm_in_worker();
