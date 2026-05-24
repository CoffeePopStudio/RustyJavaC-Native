use std::ffi::{c_char, c_int, CStr};

use rusty_javac::config::CompilerConfig;
use rusty_javac::pipeline::compile;

#[unsafe(no_mangle)]
pub extern "C" fn rustyjavac_compile(
    source_files: *const *const c_char,
    source_count: c_int,
    output_dir: *const c_char,
    java_version: c_int,
) -> c_int {
    let sources = match read_string_array(source_files, source_count) {
        Ok(s) if !s.is_empty() => s,
        _ => {
            eprintln!("rustyjavac-native: no source files provided");
            return 1;
        }
    };

    let output = read_c_str(output_dir).unwrap_or_else(|| ".".to_string());

    let mut config = CompilerConfig::new();
    config.source_files = sources;
    config.output_dir = output;
    config.java_version = java_version.max(0) as u32;

    match compile(config) {
        Ok(()) => 0,
        Err(errors) => {
            for error in errors {
                eprintln!("{}", error);
            }
            1
        }
    }
}

fn read_c_str(ptr: *const c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    unsafe { CStr::from_ptr(ptr) }.to_str().ok().map(|s| s.to_string())
}

fn read_string_array(
    ptr: *const *const c_char,
    count: c_int,
) -> Result<Vec<String>, String> {
    if ptr.is_null() || count <= 0 {
        return Ok(Vec::new());
    }

    let mut out = Vec::with_capacity(count as usize);
    for i in 0..count as isize {
        let item = unsafe { *ptr.offset(i) };
        match read_c_str(item) {
            Some(s) if !s.is_empty() => out.push(s),
            _ => return Err(format!("invalid string at index {}", i)),
        }
    }
    Ok(out)
}
