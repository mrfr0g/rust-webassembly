fn main() {
	// ... This is ignored as far as I can tell
}

#[no_mangle]
pub fn mutate_array(data: *mut Vec<i32>, len: usize) {
	let mut user_data;

	unsafe {
		user_data = Vec::from_raw_parts(data as *mut u8, len, len);
	}

	for i in 0..len {
		user_data[i] += 1;
	}

	std::mem::forget(user_data);
}
