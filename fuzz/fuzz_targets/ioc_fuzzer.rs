#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let parsed = sentinel_ioc::parse_ioc_bytes(data);
    let mut collection = sentinel_ioc::IocCollection::new();
    collection.extend(parsed.entries);
    let _ = collection.validate();
});
