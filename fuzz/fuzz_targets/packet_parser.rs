#![no_main]
use libfuzzer_sys::fuzz_target;
use xarxa::wire::*;

fuzz_target!(|data: &[u8]| {
    format!(
        "{}",
        PrettyPrinter::<EthernetFrame<&'static [u8]>>::new("", &data)
    );
});
