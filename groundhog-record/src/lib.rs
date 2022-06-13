use std::fs::File;
use std::io::prelude::*;

use libc::c_char;
use std::ffi::CStr;

use chrono::offset::Utc;
use chrono::DateTime;
use std::time::SystemTime;

#[repr(u32)]
pub enum EventType {
    SEND = 1,
    RECV,
    TRACE
}

#[no_mangle]
pub extern "C" fn hello() {
	println!("Hello World!");
}

#[no_mangle]
pub extern "C" fn init(node_name: *const c_char) {
	let node_name_str = unsafe {
        assert!(!node_name.is_null());

        CStr::from_ptr(node_name)
    };
	
	let _file = File::create(node_name_str.to_str().unwrap()).unwrap();
}

// Simple logger to prove concept, not threadsafe and opening the file everytime is dumb
#[no_mangle]
pub extern "C" fn log_event(node_name: *const c_char, event_type: *const EventType, data: *const c_char) {
	let event_type_str = unsafe {
		match *event_type {
			EventType::SEND => "send",
			EventType::RECV => "recv",
			EventType::TRACE => "trace",
		}
	};
	
	let node_name_str = unsafe {
        assert!(!node_name.is_null());

        CStr::from_ptr(node_name)
    };
	
	let data_str = unsafe {
        assert!(!data.is_null());

        CStr::from_ptr(data)
    };

	let now = SystemTime::now();
	let datetime: DateTime<Utc> = now.into();
    let mut f = File::options().append(true).open(node_name_str.to_str().unwrap()).unwrap();
	if let Err(e) = writeln!(f, "{},{},{}", datetime.format("%d/%m/%Y %T"),
							 event_type_str, data_str.to_str().unwrap()) {
		eprintln!("Couldn't write to file: {}", e);
	}
}
