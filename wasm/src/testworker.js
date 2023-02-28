// The worker has its own scope and no direct access to functions/objects of the
// global scope. We import the generated JS file to make `wasm_bindgen`
// available which we need to initialize our WASM code.
importScripts('./pkg/aleo_wasm.js');

console.log('Initializing worker');

async function init_wasm_in_worker() {
    // Load the wasm file by awaiting the Promise returned by `wasm_bindgen`.
    await wasm_bindgen('./pkg/aleo_wasm_bg.wasm');

    // Set callback to handle messages passed to the worker.
    self.onmessage = async event => {
        // By using methods of a struct as reaction to messages passed to the
        // worker, we can preserve our state between messages.
        console.log(event);
        console.log(event.data);

        // Send response back to be handled by callback in main thread.
        // A simple pass through
        self.postMessage(event.data);
    };
};

init_wasm_in_worker();