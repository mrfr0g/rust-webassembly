fn main() {
	// ... This is ignored as far as I can tell
}

/**
 * In this case the JS `number` type is converted to an i32 primitive.
 * AFAIK it's not a pointer to an i32 primitive, so we can operate on it normally
 */

#[no_mangle]
pub fn math_pow(number: i32, power: i32) -> i32 {
  let result: i32 = number.pow(power as u32); // `pow` accepts u32, so cast the i32

  result
}
