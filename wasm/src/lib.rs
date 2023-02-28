// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Aleo library.

// The Aleo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Aleo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Aleo library. If not, see <https://www.gnu.org/licenses/>.

pub mod account;

pub use account::*;

pub mod record;
pub use record::*;

pub mod program;
pub use program::*;

pub(crate) mod types;
pub(crate) use types::*;

// Messing with WebWorkers
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{console, HtmlElement, HtmlInputElement, MessageEvent, Worker};

#[wasm_bindgen(module = "./testworker.js")]
extern "C" {
    // #[wasm_bindgen(js_name = startWorker)]
    fn start_workers(module: JsValue) -> Promise;
}

#[wasm_bindgen]
pub unsafe fn start_up_worker() {
    let worker_handle = Rc::new(RefCell::new(Worker::new("./testworker.js").unwrap()));
    console::log_1(&"Created a new worker from within WASM".into());

    setup_input_oninput_callback(worker_handle.clone());
}

unsafe fn setup_input_oninput_callback(worker: Rc<RefCell<web_sys::Worker>>) {
    console::log_1(&"before create document".into());
    let document = web_sys::window().unwrap().document().unwrap();

    // If our `onmessage` callback should stay valid after exiting from the
    // `oninput` closure scope, we need to either forget it (so it is not
    // destroyed) or store it somewhere. To avoid leaking memory every time we
    // want to receive a response from the worker, we move a handle into the
    // `oninput` closure to which we will always attach the last `onmessage`
    // callback. The initial value will not be used and we silence the warning.
    #[allow(unused_assignments)]
    let mut persistent_callback_handle = get_on_msg_callback();

    console::log_1(&"before create callback".into());
    let callback = Closure::new(move |number: MessageEvent| {
        console::log_1(&"oninput callback triggered".into());

        // Access worker behind shared handle, following the interior
        // mutability pattern.
        let worker_handle = &*worker.borrow();
        let _ = worker_handle.post_message(&number.into());
        persistent_callback_handle = get_on_msg_callback();

        // Since the worker returns the message asynchronously, we
        // attach a callback to be triggered when the worker returns.
        worker_handle
            .set_onmessage(Some(persistent_callback_handle.as_ref().unchecked_ref()));
    });

    // Attach the closure as `oninput` callback to the input field.
    console::log_1(&"before create input listener".into());
    document
        .get_element_by_id("inputNumber")
        .expect("#inputNumber should exist")
        .dyn_ref::<HtmlInputElement>()
        .expect("#inputNumber should be a HtmlInputElement")
        .set_oninput(Some(callback.as_ref().unchecked_ref()));

    // Leaks memory.
    console::log_1(&"before callback forget".into());
    callback.forget();
}

unsafe fn get_on_msg_callback() -> Closure<dyn FnMut(MessageEvent)> {
    let callback = Closure::new(move |event: MessageEvent| {
        console::log_2(&"Received response: ".into(), &event.data().into());

        console::log_1(&event.data().as_string().into());
    });

    callback
}
