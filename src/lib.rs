use std::{
    ffi::{c_char, c_void, CStr, CString},
    io::ErrorKind,
    pin::Pin,
    ptr::{self, null_mut},
    sync::Arc,
    time::Duration,
};

// fn shared_runtime() -> &'static Runtime {
//     &GLOBAL
// }

// lazy_static::lazy_static! {
//     static ref GLOBAL: Runtime = tokio::runtime::Builder::new_multi_thread().worker_threads(2).enable_all().build().unwrap();
// }

use reqwest::Client;


struct ICD2ClientImpl {
    http_client: Client,
}
#[no_mangle]
fn icd2_client_destroy(i: *mut ICD2ClientImpl) {
    let client = unsafe { Box::from_raw(i) };
}
