use std::os::raw::c_char;

extern "C" {
	// emscripten.h#55
	// `EM_ASM` macro expands to this fn
	// Use this fn to make a call that returns `void`
	pub fn emscripten_asm_const(s: *const c_char);
	// emscripten.h#56
	// `EM_ASM_` macro expands to this fn
	// Use this fn to make a call that returns a ptr to a value
  pub fn emscripten_asm_const_int(s: *const c_char, ...) -> i32;
}

/**
 * Main is called during the initialization of the wasm module
 * 
 */
pub fn main() {
	// Define a byte literal string which contains the JS we want to execute 
	let s = b"\
		alert('Hello World!');\
	\0";

	unsafe {
		emscripten_asm_const(
			// I'm a unsure of this, but, to me this appears to take the 
			// pointer of `s` and converts it to a pointer of the type `c_char` 
			s as *const _ as *const c_char
		);
	}
}