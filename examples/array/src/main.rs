use std::os::raw::c_char;
use std::ffi::CStr;
use std::ffi::CString;

fn main() {
	// ... This is ignored as far as I can tell
}

fn my_string_safe(i: *mut c_char) -> String {
  unsafe {
      CStr::from_ptr(i).to_string_lossy().into_owned()
  }
}

#[no_mangle]
pub fn fix_story(i: *mut c_char) -> *mut c_char {
	let data = my_string_safe(i);
	let f = data.replace("one", "once");

	CString::new(f.as_str())
		.unwrap()
		.into_raw()
}
