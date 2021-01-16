use proxy_wasm::{self, types::LogLevel};
use proxy_wasm::traits::*;
use log::info;
use std::time::Duration;

struct HelloWorld;

impl Context for HelloWorld {}

impl RootContext for HelloWorld {
    fn on_vm_start(&mut self, _: usize) -> bool {
        info!("on vm start");
        self.set_tick_period(Duration::from_secs(5));
        true
    }
}

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(HelloWorld)
    });
}
