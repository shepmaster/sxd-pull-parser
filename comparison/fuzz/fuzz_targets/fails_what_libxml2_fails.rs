#![no_main]

use comparison::libxml2_sys;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|s: String| {
    if let Ok(_) = libxml2_sys::parse(&s) {
        return;
    };

    if let Ok(_) = comparison::parse(&s) {
        panic!("libxml2 failed to parse {:?}, but we didn't", s);
    }
});
