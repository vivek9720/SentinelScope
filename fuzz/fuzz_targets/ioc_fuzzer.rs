#![no_main]

fn run(data: &[u8]) {
    let parsed = sentinel_ioc::parse_ioc_bytes(data);
    let mut collection = sentinel_ioc::IocCollection::new();
    collection.extend(parsed.entries);
    let _ = collection.validate();
}

#[no_mangle]
pub extern "C" fn LLVMFuzzerTestOneInput(data: *const u8, size: usize) -> i32 {
    if data.is_null() {
        return 0;
    }
    // libFuzzer passes a raw pointer/length pair through its C ABI.
    let bytes = unsafe { std::slice::from_raw_parts(data, size) };
    run(bytes);
    0
}
