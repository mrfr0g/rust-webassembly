use std::os::raw::c_char;
use std::ffi::CStr;

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
 */
pub fn main() {
	// Define a byte literal string which contains the JS we want to execute 
	// Interesting note here, attachment of functions to the `window` object throw
	// a compilation error. Perhaps this is to preserve the window?
	// i.e.,
	// window.foo = function () { alert(); }
	// window.foo(); 
	// TypeError: (intermediate value) is not a function

	let s = b"\
		alert('Hello World!');\
	\0";

	// Execute the script
	unsafe {
		emscripten_asm_const(
			// I'm a unsure of this, but, to me this appears to take the 
			// pointer of `s` and converts it to a pointer of the type `c_char` 
			s as *const _ as *const c_char
		);
	}

	// Call the `prompt` function to ask the user for their name, and log it
	
	let s1 = b"\
		var name = prompt('What is your name?');\
		return allocate(intArrayFromString(name), 'i8', ALLOC_STACK);\
	\0";

	unsafe {
		// Execute the script, and this time capture the returned i32 location
		// of the pointer
		let script_ptr = emscripten_asm_const_int(
			s1 as *const _ as *const c_char
		);

		// Capture the name of the user by converting the pointer location to a c_char reference,
		// converting it to an owned string
		let name = CStr::from_ptr(script_ptr as *const c_char).to_string_lossy().into_owned();

		// The println macro is mapped for us by emscripten to `console.log` 
		// Log out the name
		println!("Hello, {}!", name);
	}
}