use vec_clock::*;

#[test]
fn new_test() {
	new(3, 1).unwrap();
}

#[test]
fn new_by_empty_test() {
	new_by::<u8, _>(&[], 0).unwrap_err();
	new_by::<u16, _>(&[], 0).unwrap_err();
	new_by::<u32, _>(&[], 0).unwrap_err();
	new_by::<u64, _>(&[], 0).unwrap_err();
	new_by::<char, _>(&[], 0).unwrap_err();
	new_by::<bool, _>(&[], 0).unwrap_err();
}

#[test]
fn new_by_zero_test() {
	new_by(&[0u8; 3], 1).unwrap();
	new_by(&[0u16; 3], 1).unwrap();
	new_by(&[0u32; 3], 1).unwrap();
	new_by(&[0u64; 3], 1).unwrap();
	new_by(&[char::from(0); 3], 1).unwrap();
	new_by(&[false; 3], 1).unwrap();
}

#[test]
fn new_by_one_test() {
	new_by(&[1u8; 3], 1).unwrap();
	new_by(&[1u16; 3], 1).unwrap();
	new_by(&[1u32; 3], 1).unwrap();
	new_by(&[1u64; 3], 1).unwrap();
	new_by(&[char::from(1); 3], 1).unwrap();
	new_by(&[true; 3], 1).unwrap();
}

#[test]
fn new_by_u8_max_test() {
	new_by(&[u8::MAX; 3], 1).unwrap();
	new_by(&[u8::MAX as u8; 3], 1).unwrap();
	new_by(&[u8::MAX as u16; 3], 1).unwrap();
	new_by(&[u8::MAX as u32; 3], 1).unwrap();
	new_by(&[u8::MAX as u64; 3], 1).unwrap();
}

#[test]
fn new_by_u16_max_test() {
	new_by(&[u16::MAX; 3], 1).unwrap();
	new_by(&[u16::MAX as u8; 3], 1).unwrap();
	new_by(&[u16::MAX as u16; 3], 1).unwrap();
	new_by(&[u16::MAX as u32; 3], 1).unwrap();
	new_by(&[u16::MAX as u64; 3], 1).unwrap();
}

#[test]
fn new_by_u32_max_test() {
	new_by(&[u32::MAX; 3], 1).unwrap();
	new_by(&[u32::MAX as u8; 3], 1).unwrap();
	new_by(&[u32::MAX as u16; 3], 1).unwrap();
	new_by(&[u32::MAX as u32; 3], 1).unwrap();
	new_by(&[u32::MAX as u64; 3], 1).unwrap();
}

#[test]
fn new_by_u64_max_test() {
	new_by(&[u64::MAX; 3], 1).unwrap();
	new_by(&[u64::MAX as u8; 3], 1).unwrap();
	new_by(&[u64::MAX as u16; 3], 1).unwrap();
	new_by(&[u64::MAX as u32; 3], 1).unwrap();
	new_by(&[u64::MAX as u64; 3], 1).unwrap();
}

#[test]
fn new_by_i8_max_test() {
	new_by(&[i8::MAX as u8; 3], 1).unwrap();
	new_by(&[i8::MAX as u16; 3], 1).unwrap();
	new_by(&[i8::MAX as u32; 3], 1).unwrap();
	new_by(&[i8::MAX as u64; 3], 1).unwrap();
}

#[test]
fn new_by_i16_max_test() {
	new_by(&[i16::MAX as u8; 3], 1).unwrap();
	new_by(&[i16::MAX as u16; 3], 1).unwrap();
	new_by(&[i16::MAX as u32; 3], 1).unwrap();
	new_by(&[i16::MAX as u64; 3], 1).unwrap();
}

#[test]
fn new_by_i32_max_test() {
	new_by(&[i32::MAX as u8; 3], 1).unwrap();
	new_by(&[i32::MAX as u16; 3], 1).unwrap();
	new_by(&[i32::MAX as u32; 3], 1).unwrap();
	new_by(&[i32::MAX as u64; 3], 1).unwrap();
}

#[test]
fn new_by_i64_max_test() {
	new_by(&[i64::MAX as u8; 3], 1).unwrap();
	new_by(&[i64::MAX as u16; 3], 1).unwrap();
	new_by(&[i64::MAX as u32; 3], 1).unwrap();
	new_by(&[i64::MAX as u64; 3], 1).unwrap();
}

#[test]
fn new_by_char_max_test() {
	new_by(&[char::MAX; 3], 1).unwrap();
	new_by(&[char::MAX as u8; 3], 1).unwrap();
	new_by(&[char::MAX as u16; 3], 1).unwrap();
	new_by(&[char::MAX as u32; 3], 1).unwrap();
	new_by(&[char::MAX as u64; 3], 1).unwrap();
}

#[test]
fn new_by_try_from_empty_test() {
	new_by_try_from::<i8, _>(&[], 0).unwrap_err();
	new_by_try_from::<i16, _>(&[], 0).unwrap_err();
	new_by_try_from::<i32, _>(&[], 0).unwrap_err();
	new_by_try_from::<i64, _>(&[], 0).unwrap_err();
	new_by_try_from::<u8, _>(&[], 0).unwrap_err();
	new_by_try_from::<u16, _>(&[], 0).unwrap_err();
	new_by_try_from::<u32, _>(&[], 0).unwrap_err();
	new_by_try_from::<u64, _>(&[], 0).unwrap_err();
	new_by_try_from::<char, _>(&[], 0).unwrap_err();
	new_by_try_from::<bool, _>(&[], 0).unwrap_err();
}

#[test]
fn new_by_try_from_zero_test() {
	new_by_try_from(&[0; 3], 1).unwrap();
	new_by_try_from(&[0u8; 3], 1).unwrap();
	new_by_try_from(&[0u16; 3], 1).unwrap();
	new_by_try_from(&[0u32; 3], 1).unwrap();
	new_by_try_from(&[0u64; 3], 1).unwrap();
	new_by_try_from(&[0i8; 3], 1).unwrap();
	new_by_try_from(&[0i16; 3], 1).unwrap();
	new_by_try_from(&[0i32; 3], 1).unwrap();
	new_by_try_from(&[0i64; 3], 1).unwrap();
	new_by_try_from(&[char::from(0); 3], 1).unwrap();
	new_by_try_from(&[false; 3], 1).unwrap();
}

#[test]
fn new_by_try_from_one_test() {
	new_by_try_from(&[1; 3], 1).unwrap();
	new_by_try_from(&[1u8; 3], 1).unwrap();
	new_by_try_from(&[1u16; 3], 1).unwrap();
	new_by_try_from(&[1u32; 3], 1).unwrap();
	new_by_try_from(&[1u64; 3], 1).unwrap();
	new_by_try_from(&[1i8; 3], 1).unwrap();
	new_by_try_from(&[1i16; 3], 1).unwrap();
	new_by_try_from(&[1i32; 3], 1).unwrap();
	new_by_try_from(&[1i64; 3], 1).unwrap();
	new_by_try_from(&[char::from(1); 3], 1).unwrap();
	new_by_try_from(&[true; 3], 1).unwrap();
}

#[test]
fn new_by_try_from_u8_max_test() {
	new_by_try_from(&[u8::MAX; 3], 1).unwrap();
	new_by_try_from(&[u8::MAX as u8; 3], 1).unwrap();
	new_by_try_from(&[u8::MAX as u16; 3], 1).unwrap();
	new_by_try_from(&[u8::MAX as u32; 3], 1).unwrap();
	new_by_try_from(&[u8::MAX as u64; 3], 1).unwrap();
	new_by_try_from(&[u8::MAX as i8; 3], 1).unwrap_err();
	new_by_try_from(&[u8::MAX as i16; 3], 1).unwrap();
	new_by_try_from(&[u8::MAX as i32; 3], 1).unwrap();
	new_by_try_from(&[u8::MAX as i64; 3], 1).unwrap();
	new_by_try_from(&[u8::MAX as char; 3], 1).unwrap();
}

#[test]
fn new_by_try_from_u16_max_test() {
	new_by_try_from(&[u16::MAX; 3], 1).unwrap();
	new_by_try_from(&[u16::MAX as u8; 3], 1).unwrap();
	new_by_try_from(&[u16::MAX as u16; 3], 1).unwrap();
	new_by_try_from(&[u16::MAX as u32; 3], 1).unwrap();
	new_by_try_from(&[u16::MAX as u64; 3], 1).unwrap();
	new_by_try_from(&[u16::MAX as i8; 3], 1).unwrap_err();
	new_by_try_from(&[u16::MAX as i16; 3], 1).unwrap_err();
	new_by_try_from(&[u16::MAX as i32; 3], 1).unwrap();
	new_by_try_from(&[u16::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_by_try_from_u32_max_test() {
	new_by_try_from(&[u32::MAX; 3], 1).unwrap();
	new_by_try_from(&[u32::MAX as u8; 3], 1).unwrap();
	new_by_try_from(&[u32::MAX as u16; 3], 1).unwrap();
	new_by_try_from(&[u32::MAX as u32; 3], 1).unwrap();
	new_by_try_from(&[u32::MAX as u64; 3], 1).unwrap();
	new_by_try_from(&[u32::MAX as i8; 3], 1).unwrap_err();
	new_by_try_from(&[u32::MAX as i16; 3], 1).unwrap_err();
	new_by_try_from(&[u32::MAX as i32; 3], 1).unwrap_err();
	new_by_try_from(&[u32::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_by_try_from_u64_max_test() {
	new_by_try_from(&[u64::MAX; 3], 1).unwrap();
	new_by_try_from(&[u64::MAX as u8; 3], 1).unwrap();
	new_by_try_from(&[u64::MAX as u16; 3], 1).unwrap();
	new_by_try_from(&[u64::MAX as u32; 3], 1).unwrap();
	new_by_try_from(&[u64::MAX as u64; 3], 1).unwrap();
	new_by_try_from(&[u64::MAX as i8; 3], 1).unwrap_err();
	new_by_try_from(&[u64::MAX as i16; 3], 1).unwrap_err();
	new_by_try_from(&[u64::MAX as i32; 3], 1).unwrap_err();
	new_by_try_from(&[u64::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_by_try_from_i8_max_test() {
	new_by_try_from(&[i8::MAX; 3], 1).unwrap();
	new_by_try_from(&[i8::MAX as u8; 3], 1).unwrap();
	new_by_try_from(&[i8::MAX as u16; 3], 1).unwrap();
	new_by_try_from(&[i8::MAX as u32; 3], 1).unwrap();
	new_by_try_from(&[i8::MAX as u64; 3], 1).unwrap();
	new_by_try_from(&[i8::MAX as i8; 3], 1).unwrap();
	new_by_try_from(&[i8::MAX as i16; 3], 1).unwrap();
	new_by_try_from(&[i8::MAX as i32; 3], 1).unwrap();
	new_by_try_from(&[i8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_by_try_from_i16_max_test() {
	new_by_try_from(&[i16::MAX; 3], 1).unwrap();
	new_by_try_from(&[i16::MAX as u8; 3], 1).unwrap();
	new_by_try_from(&[i16::MAX as u16; 3], 1).unwrap();
	new_by_try_from(&[i16::MAX as u32; 3], 1).unwrap();
	new_by_try_from(&[i16::MAX as u64; 3], 1).unwrap();
	new_by_try_from(&[i16::MAX as i8; 3], 1).unwrap_err();
	new_by_try_from(&[i16::MAX as i16; 3], 1).unwrap();
	new_by_try_from(&[i16::MAX as i32; 3], 1).unwrap();
	new_by_try_from(&[i16::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_by_try_from_i32_max_test() {
	new_by_try_from(&[i32::MAX; 3], 1).unwrap();
	new_by_try_from(&[i32::MAX as u8; 3], 1).unwrap();
	new_by_try_from(&[i32::MAX as u16; 3], 1).unwrap();
	new_by_try_from(&[i32::MAX as u32; 3], 1).unwrap();
	new_by_try_from(&[i32::MAX as u64; 3], 1).unwrap();
	new_by_try_from(&[i32::MAX as i8; 3], 1).unwrap_err();
	new_by_try_from(&[i32::MAX as i16; 3], 1).unwrap_err();
	new_by_try_from(&[i32::MAX as i32; 3], 1).unwrap();
	new_by_try_from(&[i32::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_by_try_from_i64_max_test() {
	new_by_try_from(&[i64::MAX; 3], 1).unwrap();
	new_by_try_from(&[i64::MAX as u8; 3], 1).unwrap();
	new_by_try_from(&[i64::MAX as u16; 3], 1).unwrap();
	new_by_try_from(&[i64::MAX as u32; 3], 1).unwrap();
	new_by_try_from(&[i64::MAX as u64; 3], 1).unwrap();
	new_by_try_from(&[i64::MAX as i8; 3], 1).unwrap_err();
	new_by_try_from(&[i64::MAX as i16; 3], 1).unwrap_err();
	new_by_try_from(&[i64::MAX as i32; 3], 1).unwrap_err();
	new_by_try_from(&[i64::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_by_try_from_char_max_test() {
	new_by_try_from(&[char::MAX; 3], 1).unwrap();
	new_by_try_from(&[char::MAX as u8; 3], 1).unwrap();
	new_by_try_from(&[char::MAX as u16; 3], 1).unwrap();
	new_by_try_from(&[char::MAX as u32; 3], 1).unwrap();
	new_by_try_from(&[char::MAX as u64; 3], 1).unwrap();
	new_by_try_from(&[char::MAX as i8; 3], 1).unwrap_err();
	new_by_try_from(&[char::MAX as i16; 3], 1).unwrap_err();
	new_by_try_from(&[char::MAX as i32; 3], 1).unwrap();
	new_by_try_from(&[char::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_by_try_from_minus_test() {
	new_by_try_from(&[-1; 3], 1).unwrap_err();
	new_by_try_from(&[-1i8; 3], 1).unwrap_err();
	new_by_try_from(&[-1i16; 3], 1).unwrap_err();
	new_by_try_from(&[-1i32; 3], 1).unwrap_err();
	new_by_try_from(&[-1i64; 3], 1).unwrap_err();
	new_by_try_from(&[i8::MIN; 3], 1).unwrap_err();
	new_by_try_from(&[i16::MIN; 3], 1).unwrap_err();
	new_by_try_from(&[i32::MIN; 3], 1).unwrap_err();
	new_by_try_from(&[i64::MIN; 3], 1).unwrap_err();
}

#[test]
fn new_as_test() {
	new_as::<u8>(3, 1).unwrap();
	new_as::<u16>(3, 1).unwrap();
	new_as::<u32>(3, 1).unwrap();
	new_as::<u64>(3, 1).unwrap();
	new_as::<i8>(3, 1).unwrap();
	new_as::<i16>(3, 1).unwrap();
	new_as::<i32>(3, 1).unwrap();
	new_as::<i64>(3, 1).unwrap();

	new_as::<u8>(3, 3).unwrap_err();
	new_as::<u16>(3, 3).unwrap_err();
	new_as::<u32>(3, 3).unwrap_err();
	new_as::<u64>(3, 3).unwrap_err();
	new_as::<i8>(3, 3).unwrap_err();
	new_as::<i16>(3, 3).unwrap_err();
	new_as::<i32>(3, 3).unwrap_err();
	new_as::<i64>(3, 3).unwrap_err();

	new_as::<u8>(0, 0).unwrap_err();
	new_as::<u16>(0, 0).unwrap_err();
	new_as::<u32>(0, 0).unwrap_err();
	new_as::<u64>(0, 0).unwrap_err();
	new_as::<i8>(0, 0).unwrap_err();
	new_as::<i16>(0, 0).unwrap_err();
	new_as::<i32>(0, 0).unwrap_err();
	new_as::<i64>(0, 0).unwrap_err();
}

#[test]
fn new_as_by_empty_test() {
	new_as_by::<u8, u8, _>(&[], 0).unwrap_err();
	new_as_by::<u8, bool, _>(&[], 0).unwrap_err();

	new_as_by::<u16, u8, _>(&[], 0).unwrap_err();
	new_as_by::<u16, u16, _>(&[], 0).unwrap_err();
	new_as_by::<u16, bool, _>(&[], 0).unwrap_err();

	new_as_by::<u32, u8, _>(&[], 0).unwrap_err();
	new_as_by::<u32, u16, _>(&[], 0).unwrap_err();
	new_as_by::<u32, u32, _>(&[], 0).unwrap_err();
	new_as_by::<u32, char, _>(&[], 0).unwrap_err();
	new_as_by::<u32, bool, _>(&[], 0).unwrap_err();

	new_as_by::<u64, u8, _>(&[], 0).unwrap_err();
	new_as_by::<u64, u16, _>(&[], 0).unwrap_err();
	new_as_by::<u64, u32, _>(&[], 0).unwrap_err();
	new_as_by::<u64, u64, _>(&[], 0).unwrap_err();
	new_as_by::<u64, char, _>(&[], 0).unwrap_err();
	new_as_by::<u64, bool, _>(&[], 0).unwrap_err();

	new_as_by::<i8, i8, _>(&[], 0).unwrap_err();
	new_as_by::<i8, bool, _>(&[], 0).unwrap_err();

	new_as_by::<i16, u8, _>(&[], 0).unwrap_err();
	new_as_by::<i16, i8, _>(&[], 0).unwrap_err();
	new_as_by::<i16, i16, _>(&[], 0).unwrap_err();
	new_as_by::<i16, bool, _>(&[], 0).unwrap_err();

	new_as_by::<i32, u8, _>(&[], 0).unwrap_err();
	new_as_by::<i32, u16, _>(&[], 0).unwrap_err();
	new_as_by::<i32, i8, _>(&[], 0).unwrap_err();
	new_as_by::<i32, i16, _>(&[], 0).unwrap_err();
	new_as_by::<i32, i32, _>(&[], 0).unwrap_err();
	new_as_by::<i32, bool, _>(&[], 0).unwrap_err();

	new_as_by::<i64, u8, _>(&[], 0).unwrap_err();
	new_as_by::<i64, u16, _>(&[], 0).unwrap_err();
	new_as_by::<i64, u32, _>(&[], 0).unwrap_err();
	new_as_by::<i64, i8, _>(&[], 0).unwrap_err();
	new_as_by::<i64, i16, _>(&[], 0).unwrap_err();
	new_as_by::<i64, i32, _>(&[], 0).unwrap_err();
	new_as_by::<i64, i64, _>(&[], 0).unwrap_err();
	new_as_by::<i64, bool, _>(&[], 0).unwrap_err();
}

#[test]
fn new_as_by_zero_test() {
	new_as_by::<u8, u8, _>(&[0; 3], 1).unwrap();
	new_as_by::<u8, bool, _>(&[false; 3], 1).unwrap();

	new_as_by::<u16, u8, _>(&[0; 3], 1).unwrap();
	new_as_by::<u16, u16, _>(&[0; 3], 1).unwrap();
	new_as_by::<u16, bool, _>(&[false; 3], 1).unwrap();

	new_as_by::<u32, u8, _>(&[0; 3], 1).unwrap();
	new_as_by::<u32, u16, _>(&[0; 3], 1).unwrap();
	new_as_by::<u32, u32, _>(&[0; 3], 1).unwrap();
	new_as_by::<u32, char, _>(&[char::from(0); 3], 1).unwrap();
	new_as_by::<u32, bool, _>(&[false; 3], 1).unwrap();

	new_as_by::<u64, u8, _>(&[0; 3], 1).unwrap();
	new_as_by::<u64, u16, _>(&[0; 3], 1).unwrap();
	new_as_by::<u64, u32, _>(&[0; 3], 1).unwrap();
	new_as_by::<u64, u64, _>(&[0; 3], 1).unwrap();
	new_as_by::<u64, char, _>(&[char::from(0); 3], 1).unwrap();
	new_as_by::<u64, bool, _>(&[false; 3], 1).unwrap();

	new_as_by::<i8, i8, _>(&[0; 3], 1).unwrap();
	new_as_by::<i8, bool, _>(&[false; 3], 1).unwrap();

	new_as_by::<i16, u8, _>(&[0; 3], 1).unwrap();
	new_as_by::<i16, i8, _>(&[0; 3], 1).unwrap();
	new_as_by::<i16, i16, _>(&[0; 3], 1).unwrap();
	new_as_by::<i16, bool, _>(&[false; 3], 1).unwrap();

	new_as_by::<i32, u8, _>(&[0; 3], 1).unwrap();
	new_as_by::<i32, u16, _>(&[0; 3], 1).unwrap();
	new_as_by::<i32, i8, _>(&[0; 3], 1).unwrap();
	new_as_by::<i32, i16, _>(&[0; 3], 1).unwrap();
	new_as_by::<i32, i32, _>(&[0; 3], 1).unwrap();
	new_as_by::<i32, bool, _>(&[false; 3], 1).unwrap();

	new_as_by::<i64, u8, _>(&[0; 3], 1).unwrap();
	new_as_by::<i64, u16, _>(&[0; 3], 1).unwrap();
	new_as_by::<i64, u32, _>(&[0; 3], 1).unwrap();
	new_as_by::<i64, i8, _>(&[0; 3], 1).unwrap();
	new_as_by::<i64, i16, _>(&[0; 3], 1).unwrap();
	new_as_by::<i64, i32, _>(&[0; 3], 1).unwrap();
	new_as_by::<i64, i64, _>(&[0; 3], 1).unwrap();
	new_as_by::<i64, bool, _>(&[false; 3], 1).unwrap();
}

#[test]
fn new_as_by_one_test() {
	new_as_by::<u8, u8, _>(&[1; 3], 1).unwrap();
	new_as_by::<u8, bool, _>(&[true; 3], 1).unwrap();

	new_as_by::<u16, u8, _>(&[1; 3], 1).unwrap();
	new_as_by::<u16, u16, _>(&[1; 3], 1).unwrap();
	new_as_by::<u16, bool, _>(&[true; 3], 1).unwrap();

	new_as_by::<u32, u8, _>(&[1; 3], 1).unwrap();
	new_as_by::<u32, u16, _>(&[1; 3], 1).unwrap();
	new_as_by::<u32, u32, _>(&[1; 3], 1).unwrap();
	new_as_by::<u32, char, _>(&[char::from(1); 3], 1).unwrap();
	new_as_by::<u32, bool, _>(&[true; 3], 1).unwrap();

	new_as_by::<u64, u8, _>(&[1; 3], 1).unwrap();
	new_as_by::<u64, u16, _>(&[1; 3], 1).unwrap();
	new_as_by::<u64, u32, _>(&[1; 3], 1).unwrap();
	new_as_by::<u64, u64, _>(&[1; 3], 1).unwrap();
	new_as_by::<u64, char, _>(&[char::from(1); 3], 1).unwrap();
	new_as_by::<u64, bool, _>(&[true; 3], 1).unwrap();

	new_as_by::<i8, i8, _>(&[1; 3], 1).unwrap();
	new_as_by::<i8, bool, _>(&[true; 3], 1).unwrap();

	new_as_by::<i16, u8, _>(&[1; 3], 1).unwrap();
	new_as_by::<i16, i8, _>(&[1; 3], 1).unwrap();
	new_as_by::<i16, i16, _>(&[1; 3], 1).unwrap();
	new_as_by::<i16, bool, _>(&[true; 3], 1).unwrap();

	new_as_by::<i32, u8, _>(&[1; 3], 1).unwrap();
	new_as_by::<i32, u16, _>(&[1; 3], 1).unwrap();
	new_as_by::<i32, i8, _>(&[1; 3], 1).unwrap();
	new_as_by::<i32, i16, _>(&[1; 3], 1).unwrap();
	new_as_by::<i32, i32, _>(&[1; 3], 1).unwrap();
	new_as_by::<i32, bool, _>(&[true; 3], 1).unwrap();

	new_as_by::<i64, u8, _>(&[1; 3], 1).unwrap();
	new_as_by::<i64, u16, _>(&[1; 3], 1).unwrap();
	new_as_by::<i64, u32, _>(&[1; 3], 1).unwrap();
	new_as_by::<i64, i8, _>(&[1; 3], 1).unwrap();
	new_as_by::<i64, i16, _>(&[1; 3], 1).unwrap();
	new_as_by::<i64, i32, _>(&[1; 3], 1).unwrap();
	new_as_by::<i64, i64, _>(&[1; 3], 1).unwrap();
	new_as_by::<i64, bool, _>(&[true; 3], 1).unwrap();
}

#[test]
fn new_as_by_u8_max_test() {
	new_as_by::<u8, u8, _>(&[u8::MAX as u8; 3], 1).unwrap();

	new_as_by::<u16, u8, _>(&[u8::MAX as u8; 3], 1).unwrap();
	new_as_by::<u16, u16, _>(&[u8::MAX as u16; 3], 1).unwrap();

	new_as_by::<u32, u8, _>(&[u8::MAX as u8; 3], 1).unwrap();
	new_as_by::<u32, u16, _>(&[u8::MAX as u16; 3], 1).unwrap();
	new_as_by::<u32, u32, _>(&[u8::MAX as u32; 3], 1).unwrap();

	new_as_by::<u64, u8, _>(&[u8::MAX as u8; 3], 1).unwrap();
	new_as_by::<u64, u16, _>(&[u8::MAX as u16; 3], 1).unwrap();
	new_as_by::<u64, u32, _>(&[u8::MAX as u32; 3], 1).unwrap();
	new_as_by::<u64, u64, _>(&[u8::MAX as u64; 3], 1).unwrap();

	new_as_by::<i8, i8, _>(&[u8::MAX as i8; 3], 1).unwrap();

	new_as_by::<i16, u8, _>(&[u8::MAX as u8; 3], 1).unwrap();
	new_as_by::<i16, i8, _>(&[u8::MAX as i8; 3], 1).unwrap();
	new_as_by::<i16, i16, _>(&[u8::MAX as i16; 3], 1).unwrap();

	new_as_by::<i32, u8, _>(&[u8::MAX as u8; 3], 1).unwrap();
	new_as_by::<i32, u16, _>(&[u8::MAX as u16; 3], 1).unwrap();
	new_as_by::<i32, i8, _>(&[u8::MAX as i8; 3], 1).unwrap();
	new_as_by::<i32, i16, _>(&[u8::MAX as i16; 3], 1).unwrap();
	new_as_by::<i32, i32, _>(&[u8::MAX as i32; 3], 1).unwrap();

	new_as_by::<i64, u8, _>(&[u8::MAX as u8; 3], 1).unwrap();
	new_as_by::<i64, u16, _>(&[u8::MAX as u16; 3], 1).unwrap();
	new_as_by::<i64, u32, _>(&[u8::MAX as u32; 3], 1).unwrap();
	new_as_by::<i64, i8, _>(&[u8::MAX as i8; 3], 1).unwrap();
	new_as_by::<i64, i16, _>(&[u8::MAX as i16; 3], 1).unwrap();
	new_as_by::<i64, i32, _>(&[u8::MAX as i32; 3], 1).unwrap();
	new_as_by::<i64, i64, _>(&[u8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_u16_max_test() {
	new_as_by::<u8, u8, _>(&[u16::MAX as u8; 3], 1).unwrap();

	new_as_by::<u16, u8, _>(&[u16::MAX as u8; 3], 1).unwrap();
	new_as_by::<u16, u16, _>(&[u16::MAX as u16; 3], 1).unwrap();

	new_as_by::<u32, u8, _>(&[u16::MAX as u8; 3], 1).unwrap();
	new_as_by::<u32, u16, _>(&[u16::MAX as u16; 3], 1).unwrap();
	new_as_by::<u32, u32, _>(&[u16::MAX as u32; 3], 1).unwrap();

	new_as_by::<u64, u8, _>(&[u16::MAX as u8; 3], 1).unwrap();
	new_as_by::<u64, u16, _>(&[u16::MAX as u16; 3], 1).unwrap();
	new_as_by::<u64, u32, _>(&[u16::MAX as u32; 3], 1).unwrap();
	new_as_by::<u64, u64, _>(&[u16::MAX as u64; 3], 1).unwrap();

	new_as_by::<i8, i8, _>(&[u16::MAX as i8; 3], 1).unwrap();

	new_as_by::<i16, u8, _>(&[u16::MAX as u8; 3], 1).unwrap();
	new_as_by::<i16, i8, _>(&[u16::MAX as i8; 3], 1).unwrap();
	new_as_by::<i16, i16, _>(&[u16::MAX as i16; 3], 1).unwrap();

	new_as_by::<i32, u8, _>(&[u16::MAX as u8; 3], 1).unwrap();
	new_as_by::<i32, u16, _>(&[u16::MAX as u16; 3], 1).unwrap();
	new_as_by::<i32, i8, _>(&[u16::MAX as i8; 3], 1).unwrap();
	new_as_by::<i32, i16, _>(&[u16::MAX as i16; 3], 1).unwrap();
	new_as_by::<i32, i32, _>(&[u16::MAX as i32; 3], 1).unwrap();

	new_as_by::<i64, u8, _>(&[u16::MAX as u8; 3], 1).unwrap();
	new_as_by::<i64, u16, _>(&[u16::MAX as u16; 3], 1).unwrap();
	new_as_by::<i64, u32, _>(&[u16::MAX as u32; 3], 1).unwrap();
	new_as_by::<i64, i8, _>(&[u16::MAX as i8; 3], 1).unwrap();
	new_as_by::<i64, i16, _>(&[u16::MAX as i16; 3], 1).unwrap();
	new_as_by::<i64, i32, _>(&[u16::MAX as i32; 3], 1).unwrap();
	new_as_by::<i64, i64, _>(&[u16::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_u32_max_test() {
	new_as_by::<u8, u8, _>(&[u32::MAX as u8; 3], 1).unwrap();

	new_as_by::<u16, u8, _>(&[u32::MAX as u8; 3], 1).unwrap();
	new_as_by::<u16, u16, _>(&[u32::MAX as u16; 3], 1).unwrap();

	new_as_by::<u32, u8, _>(&[u32::MAX as u8; 3], 1).unwrap();
	new_as_by::<u32, u16, _>(&[u32::MAX as u16; 3], 1).unwrap();
	new_as_by::<u32, u32, _>(&[u32::MAX as u32; 3], 1).unwrap();

	new_as_by::<u64, u8, _>(&[u32::MAX as u8; 3], 1).unwrap();
	new_as_by::<u64, u16, _>(&[u32::MAX as u16; 3], 1).unwrap();
	new_as_by::<u64, u32, _>(&[u32::MAX as u32; 3], 1).unwrap();
	new_as_by::<u64, u64, _>(&[u32::MAX as u64; 3], 1).unwrap();

	new_as_by::<i8, i8, _>(&[u32::MAX as i8; 3], 1).unwrap();

	new_as_by::<i16, u8, _>(&[u32::MAX as u8; 3], 1).unwrap();
	new_as_by::<i16, i8, _>(&[u32::MAX as i8; 3], 1).unwrap();
	new_as_by::<i16, i16, _>(&[u32::MAX as i16; 3], 1).unwrap();

	new_as_by::<i32, u8, _>(&[u32::MAX as u8; 3], 1).unwrap();
	new_as_by::<i32, u16, _>(&[u32::MAX as u16; 3], 1).unwrap();
	new_as_by::<i32, i8, _>(&[u32::MAX as i8; 3], 1).unwrap();
	new_as_by::<i32, i16, _>(&[u32::MAX as i16; 3], 1).unwrap();
	new_as_by::<i32, i32, _>(&[u32::MAX as i32; 3], 1).unwrap();

	new_as_by::<i64, u8, _>(&[u32::MAX as u8; 3], 1).unwrap();
	new_as_by::<i64, u16, _>(&[u32::MAX as u16; 3], 1).unwrap();
	new_as_by::<i64, u32, _>(&[u32::MAX as u32; 3], 1).unwrap();
	new_as_by::<i64, i8, _>(&[u32::MAX as i8; 3], 1).unwrap();
	new_as_by::<i64, i16, _>(&[u32::MAX as i16; 3], 1).unwrap();
	new_as_by::<i64, i32, _>(&[u32::MAX as i32; 3], 1).unwrap();
	new_as_by::<i64, i64, _>(&[u32::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_u64_max_test() {
	new_as_by::<u8, u8, _>(&[u64::MAX as u8; 3], 1).unwrap();

	new_as_by::<u16, u8, _>(&[u64::MAX as u8; 3], 1).unwrap();
	new_as_by::<u16, u16, _>(&[u64::MAX as u16; 3], 1).unwrap();

	new_as_by::<u32, u8, _>(&[u64::MAX as u8; 3], 1).unwrap();
	new_as_by::<u32, u16, _>(&[u64::MAX as u16; 3], 1).unwrap();
	new_as_by::<u32, u32, _>(&[u64::MAX as u32; 3], 1).unwrap();

	new_as_by::<u64, u8, _>(&[u64::MAX as u8; 3], 1).unwrap();
	new_as_by::<u64, u16, _>(&[u64::MAX as u16; 3], 1).unwrap();
	new_as_by::<u64, u32, _>(&[u64::MAX as u32; 3], 1).unwrap();
	new_as_by::<u64, u64, _>(&[u64::MAX as u64; 3], 1).unwrap();

	new_as_by::<i8, i8, _>(&[u64::MAX as i8; 3], 1).unwrap();

	new_as_by::<i16, u8, _>(&[u64::MAX as u8; 3], 1).unwrap();
	new_as_by::<i16, i8, _>(&[u64::MAX as i8; 3], 1).unwrap();
	new_as_by::<i16, i16, _>(&[u64::MAX as i16; 3], 1).unwrap();

	new_as_by::<i32, u8, _>(&[u64::MAX as u8; 3], 1).unwrap();
	new_as_by::<i32, u16, _>(&[u64::MAX as u16; 3], 1).unwrap();
	new_as_by::<i32, i8, _>(&[u64::MAX as i8; 3], 1).unwrap();
	new_as_by::<i32, i16, _>(&[u64::MAX as i16; 3], 1).unwrap();
	new_as_by::<i32, i32, _>(&[u64::MAX as i32; 3], 1).unwrap();

	new_as_by::<i64, u8, _>(&[u64::MAX as u8; 3], 1).unwrap();
	new_as_by::<i64, u16, _>(&[u64::MAX as u16; 3], 1).unwrap();
	new_as_by::<i64, u32, _>(&[u64::MAX as u32; 3], 1).unwrap();
	new_as_by::<i64, i8, _>(&[u64::MAX as i8; 3], 1).unwrap();
	new_as_by::<i64, i16, _>(&[u64::MAX as i16; 3], 1).unwrap();
	new_as_by::<i64, i32, _>(&[u64::MAX as i32; 3], 1).unwrap();
	new_as_by::<i64, i64, _>(&[u64::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_i8_max_test() {
	new_as_by::<u8, u8, _>(&[i8::MAX as u8; 3], 1).unwrap();

	new_as_by::<u16, u8, _>(&[i8::MAX as u8; 3], 1).unwrap();
	new_as_by::<u16, u16, _>(&[i8::MAX as u16; 3], 1).unwrap();

	new_as_by::<u32, u8, _>(&[i8::MAX as u8; 3], 1).unwrap();
	new_as_by::<u32, u16, _>(&[i8::MAX as u16; 3], 1).unwrap();
	new_as_by::<u32, u32, _>(&[i8::MAX as u32; 3], 1).unwrap();

	new_as_by::<u64, u8, _>(&[i8::MAX as u8; 3], 1).unwrap();
	new_as_by::<u64, u16, _>(&[i8::MAX as u16; 3], 1).unwrap();
	new_as_by::<u64, u32, _>(&[i8::MAX as u32; 3], 1).unwrap();
	new_as_by::<u64, u64, _>(&[i8::MAX as u64; 3], 1).unwrap();

	new_as_by::<i8, i8, _>(&[i8::MAX as i8; 3], 1).unwrap();

	new_as_by::<i16, u8, _>(&[i8::MAX as u8; 3], 1).unwrap();
	new_as_by::<i16, i8, _>(&[i8::MAX as i8; 3], 1).unwrap();
	new_as_by::<i16, i16, _>(&[i8::MAX as i16; 3], 1).unwrap();

	new_as_by::<i32, u8, _>(&[i8::MAX as u8; 3], 1).unwrap();
	new_as_by::<i32, u16, _>(&[i8::MAX as u16; 3], 1).unwrap();
	new_as_by::<i32, i8, _>(&[i8::MAX as i8; 3], 1).unwrap();
	new_as_by::<i32, i16, _>(&[i8::MAX as i16; 3], 1).unwrap();
	new_as_by::<i32, i32, _>(&[i8::MAX as i32; 3], 1).unwrap();

	new_as_by::<i64, u8, _>(&[i8::MAX as u8; 3], 1).unwrap();
	new_as_by::<i64, u16, _>(&[i8::MAX as u16; 3], 1).unwrap();
	new_as_by::<i64, u32, _>(&[i8::MAX as u32; 3], 1).unwrap();
	new_as_by::<i64, i8, _>(&[i8::MAX as i8; 3], 1).unwrap();
	new_as_by::<i64, i16, _>(&[i8::MAX as i16; 3], 1).unwrap();
	new_as_by::<i64, i32, _>(&[i8::MAX as i32; 3], 1).unwrap();
	new_as_by::<i64, i64, _>(&[i8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_i16_max_test() {
	new_as_by::<u8, u8, _>(&[i16::MAX as u8; 3], 1).unwrap();

	new_as_by::<u16, u8, _>(&[i16::MAX as u8; 3], 1).unwrap();
	new_as_by::<u16, u16, _>(&[i16::MAX as u16; 3], 1).unwrap();

	new_as_by::<u32, u8, _>(&[i16::MAX as u8; 3], 1).unwrap();
	new_as_by::<u32, u16, _>(&[i16::MAX as u16; 3], 1).unwrap();
	new_as_by::<u32, u32, _>(&[i16::MAX as u32; 3], 1).unwrap();

	new_as_by::<u64, u8, _>(&[i16::MAX as u8; 3], 1).unwrap();
	new_as_by::<u64, u16, _>(&[i16::MAX as u16; 3], 1).unwrap();
	new_as_by::<u64, u32, _>(&[i16::MAX as u32; 3], 1).unwrap();
	new_as_by::<u64, u64, _>(&[i16::MAX as u64; 3], 1).unwrap();

	new_as_by::<i8, i8, _>(&[i16::MAX as i8; 3], 1).unwrap();

	new_as_by::<i16, u8, _>(&[i16::MAX as u8; 3], 1).unwrap();
	new_as_by::<i16, i8, _>(&[i16::MAX as i8; 3], 1).unwrap();
	new_as_by::<i16, i16, _>(&[i16::MAX as i16; 3], 1).unwrap();

	new_as_by::<i32, u8, _>(&[i16::MAX as u8; 3], 1).unwrap();
	new_as_by::<i32, u16, _>(&[i16::MAX as u16; 3], 1).unwrap();
	new_as_by::<i32, i8, _>(&[i16::MAX as i8; 3], 1).unwrap();
	new_as_by::<i32, i16, _>(&[i16::MAX as i16; 3], 1).unwrap();
	new_as_by::<i32, i32, _>(&[i16::MAX as i32; 3], 1).unwrap();

	new_as_by::<i64, u8, _>(&[i16::MAX as u8; 3], 1).unwrap();
	new_as_by::<i64, u16, _>(&[i16::MAX as u16; 3], 1).unwrap();
	new_as_by::<i64, u32, _>(&[i16::MAX as u32; 3], 1).unwrap();
	new_as_by::<i64, i8, _>(&[i16::MAX as i8; 3], 1).unwrap();
	new_as_by::<i64, i16, _>(&[i16::MAX as i16; 3], 1).unwrap();
	new_as_by::<i64, i32, _>(&[i16::MAX as i32; 3], 1).unwrap();
	new_as_by::<i64, i64, _>(&[i16::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_i32_max_test() {
	new_as_by::<u8, u8, _>(&[i32::MAX as u8; 3], 1).unwrap();

	new_as_by::<u16, u8, _>(&[i32::MAX as u8; 3], 1).unwrap();
	new_as_by::<u16, u16, _>(&[i32::MAX as u16; 3], 1).unwrap();

	new_as_by::<u32, u8, _>(&[i32::MAX as u8; 3], 1).unwrap();
	new_as_by::<u32, u16, _>(&[i32::MAX as u16; 3], 1).unwrap();
	new_as_by::<u32, u32, _>(&[i32::MAX as u32; 3], 1).unwrap();

	new_as_by::<u64, u8, _>(&[i32::MAX as u8; 3], 1).unwrap();
	new_as_by::<u64, u16, _>(&[i32::MAX as u16; 3], 1).unwrap();
	new_as_by::<u64, u32, _>(&[i32::MAX as u32; 3], 1).unwrap();
	new_as_by::<u64, u64, _>(&[i32::MAX as u64; 3], 1).unwrap();

	new_as_by::<i8, i8, _>(&[i32::MAX as i8; 3], 1).unwrap();

	new_as_by::<i16, u8, _>(&[i32::MAX as u8; 3], 1).unwrap();
	new_as_by::<i16, i8, _>(&[i32::MAX as i8; 3], 1).unwrap();
	new_as_by::<i16, i16, _>(&[i32::MAX as i16; 3], 1).unwrap();

	new_as_by::<i32, u8, _>(&[i32::MAX as u8; 3], 1).unwrap();
	new_as_by::<i32, u16, _>(&[i32::MAX as u16; 3], 1).unwrap();
	new_as_by::<i32, i8, _>(&[i32::MAX as i8; 3], 1).unwrap();
	new_as_by::<i32, i16, _>(&[i32::MAX as i16; 3], 1).unwrap();
	new_as_by::<i32, i32, _>(&[i32::MAX as i32; 3], 1).unwrap();

	new_as_by::<i64, u8, _>(&[i32::MAX as u8; 3], 1).unwrap();
	new_as_by::<i64, u16, _>(&[i32::MAX as u16; 3], 1).unwrap();
	new_as_by::<i64, u32, _>(&[i32::MAX as u32; 3], 1).unwrap();
	new_as_by::<i64, i8, _>(&[i32::MAX as i8; 3], 1).unwrap();
	new_as_by::<i64, i16, _>(&[i32::MAX as i16; 3], 1).unwrap();
	new_as_by::<i64, i32, _>(&[i32::MAX as i32; 3], 1).unwrap();
	new_as_by::<i64, i64, _>(&[i32::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_i64_max_test() {
	new_as_by::<u8, u8, _>(&[i64::MAX as u8; 3], 1).unwrap();

	new_as_by::<u16, u8, _>(&[i64::MAX as u8; 3], 1).unwrap();
	new_as_by::<u16, u16, _>(&[i64::MAX as u16; 3], 1).unwrap();

	new_as_by::<u32, u8, _>(&[i64::MAX as u8; 3], 1).unwrap();
	new_as_by::<u32, u16, _>(&[i64::MAX as u16; 3], 1).unwrap();
	new_as_by::<u32, u32, _>(&[i64::MAX as u32; 3], 1).unwrap();

	new_as_by::<u64, u8, _>(&[i64::MAX as u8; 3], 1).unwrap();
	new_as_by::<u64, u16, _>(&[i64::MAX as u16; 3], 1).unwrap();
	new_as_by::<u64, u32, _>(&[i64::MAX as u32; 3], 1).unwrap();
	new_as_by::<u64, u64, _>(&[i64::MAX as u64; 3], 1).unwrap();

	new_as_by::<i8, i8, _>(&[i64::MAX as i8; 3], 1).unwrap();

	new_as_by::<i16, u8, _>(&[i64::MAX as u8; 3], 1).unwrap();
	new_as_by::<i16, i8, _>(&[i64::MAX as i8; 3], 1).unwrap();
	new_as_by::<i16, i16, _>(&[i64::MAX as i16; 3], 1).unwrap();

	new_as_by::<i32, u8, _>(&[i64::MAX as u8; 3], 1).unwrap();
	new_as_by::<i32, u16, _>(&[i64::MAX as u16; 3], 1).unwrap();
	new_as_by::<i32, i8, _>(&[i64::MAX as i8; 3], 1).unwrap();
	new_as_by::<i32, i16, _>(&[i64::MAX as i16; 3], 1).unwrap();
	new_as_by::<i32, i32, _>(&[i64::MAX as i32; 3], 1).unwrap();

	new_as_by::<i64, u8, _>(&[i64::MAX as u8; 3], 1).unwrap();
	new_as_by::<i64, u16, _>(&[i64::MAX as u16; 3], 1).unwrap();
	new_as_by::<i64, u32, _>(&[i64::MAX as u32; 3], 1).unwrap();
	new_as_by::<i64, i8, _>(&[i64::MAX as i8; 3], 1).unwrap();
	new_as_by::<i64, i16, _>(&[i64::MAX as i16; 3], 1).unwrap();
	new_as_by::<i64, i32, _>(&[i64::MAX as i32; 3], 1).unwrap();
	new_as_by::<i64, i64, _>(&[i64::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_char_max_test() {
	new_as_by::<u8, u8, _>(&[char::MAX as u8; 3], 1).unwrap();

	new_as_by::<u16, u8, _>(&[char::MAX as u8; 3], 1).unwrap();
	new_as_by::<u16, u16, _>(&[char::MAX as u16; 3], 1).unwrap();

	new_as_by::<u32, u8, _>(&[char::MAX as u8; 3], 1).unwrap();
	new_as_by::<u32, u16, _>(&[char::MAX as u16; 3], 1).unwrap();
	new_as_by::<u32, u32, _>(&[char::MAX as u32; 3], 1).unwrap();

	new_as_by::<u64, u8, _>(&[char::MAX as u8; 3], 1).unwrap();
	new_as_by::<u64, u16, _>(&[char::MAX as u16; 3], 1).unwrap();
	new_as_by::<u64, u32, _>(&[char::MAX as u32; 3], 1).unwrap();
	new_as_by::<u64, u64, _>(&[char::MAX as u64; 3], 1).unwrap();

	new_as_by::<i8, i8, _>(&[char::MAX as i8; 3], 1).unwrap();

	new_as_by::<i16, u8, _>(&[char::MAX as u8; 3], 1).unwrap();
	new_as_by::<i16, i8, _>(&[char::MAX as i8; 3], 1).unwrap();
	new_as_by::<i16, i16, _>(&[char::MAX as i16; 3], 1).unwrap();

	new_as_by::<i32, u8, _>(&[char::MAX as u8; 3], 1).unwrap();
	new_as_by::<i32, u16, _>(&[char::MAX as u16; 3], 1).unwrap();
	new_as_by::<i32, i8, _>(&[char::MAX as i8; 3], 1).unwrap();
	new_as_by::<i32, i16, _>(&[char::MAX as i16; 3], 1).unwrap();
	new_as_by::<i32, i32, _>(&[char::MAX as i32; 3], 1).unwrap();

	new_as_by::<i64, u8, _>(&[char::MAX as u8; 3], 1).unwrap();
	new_as_by::<i64, u16, _>(&[char::MAX as u16; 3], 1).unwrap();
	new_as_by::<i64, u32, _>(&[char::MAX as u32; 3], 1).unwrap();
	new_as_by::<i64, i8, _>(&[char::MAX as i8; 3], 1).unwrap();
	new_as_by::<i64, i16, _>(&[char::MAX as i16; 3], 1).unwrap();
	new_as_by::<i64, i32, _>(&[char::MAX as i32; 3], 1).unwrap();
	new_as_by::<i64, i64, _>(&[char::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_minus_test() {
	new_as_by::<i8, i8, _>(&[-1; 3], 1).unwrap();
	new_as_by::<i8, i8, _>(&[i8::MIN; 3], 1).unwrap();

	new_as_by::<i16, i8, _>(&[-1; 3], 1).unwrap();
	new_as_by::<i16, i16, _>(&[-1; 3], 1).unwrap();
	new_as_by::<i16, i8, _>(&[i8::MIN; 3], 1).unwrap();
	new_as_by::<i16, i16, _>(&[i16::MIN; 3], 1).unwrap();

	new_as_by::<i32, i8, _>(&[-1; 3], 1).unwrap();
	new_as_by::<i32, i16, _>(&[-1; 3], 1).unwrap();
	new_as_by::<i32, i32, _>(&[-1; 3], 1).unwrap();
	new_as_by::<i32, i8, _>(&[i8::MIN; 3], 1).unwrap();
	new_as_by::<i32, i16, _>(&[i16::MIN; 3], 1).unwrap();
	new_as_by::<i32, i32, _>(&[i32::MIN; 3], 1).unwrap();

	new_as_by::<i64, i8, _>(&[-1; 3], 1).unwrap();
	new_as_by::<i64, i16, _>(&[-1; 3], 1).unwrap();
	new_as_by::<i64, i32, _>(&[-1; 3], 1).unwrap();
	new_as_by::<i64, i64, _>(&[-1; 3], 1).unwrap();
	new_as_by::<i64, i8, _>(&[i8::MIN; 3], 1).unwrap();
	new_as_by::<i64, i16, _>(&[i16::MIN; 3], 1).unwrap();
	new_as_by::<i64, i32, _>(&[i32::MIN; 3], 1).unwrap();
	new_as_by::<i64, i64, _>(&[i64::MIN; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u8_empty_test() {
	new_as_by_try_from::<u8, u8, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u8, u16, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u8, u32, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u8, u64, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u8, i8, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u8, i16, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u8, i32, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u8, i64, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u8, char, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u8, bool, _>(&[], 0).unwrap_err();
}

#[test]
fn new_as_by_try_from_u16_empty_test() {
	new_as_by_try_from::<u16, u8, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u16, u16, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u16, u32, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u16, u64, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u16, i8, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u16, i16, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u16, i32, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u16, i64, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u16, char, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u16, bool, _>(&[], 0).unwrap_err();
}

#[test]
fn new_as_by_try_from_u32_empty_test() {
	new_as_by_try_from::<u32, u8, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u32, u16, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u32, u32, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u32, u64, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u32, i8, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u32, i16, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u32, i32, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u32, i64, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u32, char, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u32, bool, _>(&[], 0).unwrap_err();
}

#[test]
fn new_as_by_try_from_u64_empty_test() {
	new_as_by_try_from::<u64, u8, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u64, u16, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u64, u32, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u64, u64, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u64, i8, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u64, i16, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u64, i32, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u64, i64, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u64, char, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<u64, bool, _>(&[], 0).unwrap_err();
}

#[test]
fn new_as_by_try_from_i8_empty_test() {
	new_as_by_try_from::<i8, u8, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<i8, u16, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<i8, u32, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<i8, u64, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<i8, i8, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<i8, i16, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<i8, i32, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<i8, i64, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<i8, bool, _>(&[], 0).unwrap_err();
}

#[test]
fn new_as_by_try_from_i16_empty_test() {
	new_as_by_try_from::<i16, u8, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<i16, u16, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<i16, u32, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<i16, u64, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<i16, i8, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<i16, i16, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<i16, i32, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<i16, i64, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<i16, bool, _>(&[], 0).unwrap_err();
}

#[test]
fn new_as_by_try_from_i32_empty_test() {
	new_as_by_try_from::<i32, u8, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<i32, u16, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<i32, u32, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<i32, u64, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<i32, i8, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<i32, i16, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<i32, i32, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<i32, i64, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<i32, bool, _>(&[], 0).unwrap_err();
}

#[test]
fn new_as_by_try_from_i64_empty_test() {
	new_as_by_try_from::<i64, u8, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<i64, u16, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<i64, u32, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<i64, u64, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<i64, i8, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<i64, i16, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<i64, i32, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<i64, i64, _>(&[], 0).unwrap_err();
	new_as_by_try_from::<i64, bool, _>(&[], 0).unwrap_err();
}

#[test]
fn new_as_by_try_from_u8_zero_test() {
	new_as_by_try_from::<u8, u8, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u8, u16, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u8, u32, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u8, u64, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u8, i8, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u8, i16, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u8, i32, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u8, i64, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u8, char, _>(&[char::from(0); 3], 1).unwrap();
	new_as_by_try_from::<u8, bool, _>(&[false; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u16_zero_test() {
	new_as_by_try_from::<u16, u8, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u16, u16, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u16, u32, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u16, u64, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u16, i8, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u16, i16, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u16, i32, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u16, i64, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u16, char, _>(&[char::from(0); 3], 1).unwrap();
	new_as_by_try_from::<u16, bool, _>(&[false; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u32_zero_test() {
	new_as_by_try_from::<u32, u8, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u32, u16, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u32, u32, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u32, u64, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u32, i8, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u32, i16, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u32, i32, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u32, i64, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u32, char, _>(&[char::from(0); 3], 1).unwrap();
	new_as_by_try_from::<u32, bool, _>(&[false; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u64_zero_test() {
	new_as_by_try_from::<u64, u8, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u64, u16, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u64, u32, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u64, u64, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u64, i8, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u64, i16, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u64, i32, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u64, i64, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u64, char, _>(&[char::from(0); 3], 1).unwrap();
	new_as_by_try_from::<u64, bool, _>(&[false; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i8_zero_test() {
	new_as_by_try_from::<i8, u8, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i8, u16, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i8, u32, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i8, u64, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i8, i8, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i8, i16, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i8, i32, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i8, i64, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i8, bool, _>(&[false; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i16_zero_test() {
	new_as_by_try_from::<i16, u8, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i16, u16, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i16, u32, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i16, u64, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i16, i8, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i16, i16, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i16, i32, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i16, i64, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i16, bool, _>(&[false; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i32_zero_test() {
	new_as_by_try_from::<i32, u8, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i32, u16, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i32, u32, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i32, u64, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i32, i8, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i32, i16, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i32, i32, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i32, i64, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i32, bool, _>(&[false; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i64_zero_test() {
	new_as_by_try_from::<i64, u8, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i64, u16, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i64, u32, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i64, u64, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i64, i8, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i64, i16, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i64, i32, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i64, i64, _>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i64, bool, _>(&[false; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u8_one_test() {
	new_as_by_try_from::<u8, u8, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u8, u16, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u8, u32, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u8, u64, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u8, i8, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u8, i16, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u8, i32, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u8, i64, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u8, char, _>(&[char::from(1); 3], 1).unwrap();
	new_as_by_try_from::<u8, bool, _>(&[true; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u16_one_test() {
	new_as_by_try_from::<u16, u8, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u16, u16, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u16, u32, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u16, u64, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u16, i8, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u16, i16, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u16, i32, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u16, i64, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u16, char, _>(&[char::from(1); 3], 1).unwrap();
	new_as_by_try_from::<u16, bool, _>(&[true; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u32_one_test() {
	new_as_by_try_from::<u32, u8, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u32, u16, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u32, u32, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u32, u64, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u32, i8, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u32, i16, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u32, i32, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u32, i64, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u32, char, _>(&[char::from(1); 3], 1).unwrap();
	new_as_by_try_from::<u32, bool, _>(&[true; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u64_one_test() {
	new_as_by_try_from::<u64, u8, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u64, u16, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u64, u32, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u64, u64, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u64, i8, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u64, i16, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u64, i32, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u64, i64, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u64, char, _>(&[char::from(1); 3], 1).unwrap();
	new_as_by_try_from::<u64, bool, _>(&[true; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i8_one_test() {
	new_as_by_try_from::<i8, u8, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i8, u16, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i8, u32, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i8, u64, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i8, i8, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i8, i16, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i8, i32, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i8, i64, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i8, bool, _>(&[true; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i16_one_test() {
	new_as_by_try_from::<i16, u8, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i16, u16, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i16, u32, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i16, u64, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i16, i8, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i16, i16, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i16, i32, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i16, i64, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i16, bool, _>(&[true; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i32_one_test() {
	new_as_by_try_from::<i32, u8, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i32, u16, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i32, u32, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i32, u64, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i32, i8, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i32, i16, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i32, i32, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i32, i64, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i32, bool, _>(&[true; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i64_one_test() {
	new_as_by_try_from::<i64, u8, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i64, u16, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i64, u32, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i64, u64, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i64, i8, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i64, i16, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i64, i32, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i64, i64, _>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i64, bool, _>(&[true; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u8_u8_max_test() {
	new_as_by_try_from::<u8, u8, _>(&[u8::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u8, u16, _>(&[u8::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u8, u32, _>(&[u8::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u8, u64, _>(&[u8::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u8, i8, _>(&[u8::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i16, _>(&[u8::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<u8, i32, _>(&[u8::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u8, i64, _>(&[u8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u8_u16_max_test() {
	new_as_by_try_from::<u8, u8, _>(&[u16::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u8, u16, _>(&[u16::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, u32, _>(&[u16::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, u64, _>(&[u16::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i8, _>(&[u16::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i16, _>(&[u16::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i32, _>(&[u16::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i64, _>(&[u16::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u8_u32_max_test() {
	new_as_by_try_from::<u8, u8, _>(&[u32::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u8, u16, _>(&[u32::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, u32, _>(&[u32::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, u64, _>(&[u32::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i8, _>(&[u32::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i16, _>(&[u32::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i32, _>(&[u32::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i64, _>(&[u32::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u8_u64_max_test() {
	new_as_by_try_from::<u8, u8, _>(&[u64::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u8, u16, _>(&[u64::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, u32, _>(&[u64::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, u64, _>(&[u64::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i8, _>(&[u64::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i16, _>(&[u64::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i32, _>(&[u64::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i64, _>(&[u64::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u8_i8_max_test() {
	new_as_by_try_from::<u8, u8, _>(&[i8::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u8, u16, _>(&[i8::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u8, u32, _>(&[i8::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u8, u64, _>(&[i8::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u8, i8, _>(&[i8::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<u8, i16, _>(&[i8::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<u8, i32, _>(&[i8::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u8, i64, _>(&[i8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u8_i16_max_test() {
	new_as_by_try_from::<u8, u8, _>(&[i16::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u8, u16, _>(&[i16::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, u32, _>(&[i16::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, u64, _>(&[i16::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i8, _>(&[i16::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i16, _>(&[i16::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i32, _>(&[i16::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i64, _>(&[i16::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u8_i32_max_test() {
	new_as_by_try_from::<u8, u8, _>(&[i32::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u8, u16, _>(&[i32::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, u32, _>(&[i32::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, u64, _>(&[i32::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i8, _>(&[i32::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i16, _>(&[i32::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i32, _>(&[i32::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i64, _>(&[i32::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u8_i64_max_test() {
	new_as_by_try_from::<u8, u8, _>(&[i64::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u8, u16, _>(&[i64::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, u32, _>(&[i64::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, u64, _>(&[i64::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i8, _>(&[i64::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i16, _>(&[i64::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i32, _>(&[i64::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i64, _>(&[i64::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u8_char_max_test() {
	new_as_by_try_from::<u8, u8, _>(&[char::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u8, u16, _>(&[char::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, u32, _>(&[char::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, u64, _>(&[char::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i8, _>(&[char::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i16, _>(&[char::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i32, _>(&[char::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i64, _>(&[char::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u16_u8_max_test() {
	new_as_by_try_from::<u16, u8, _>(&[u8::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u16, u16, _>(&[u8::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u16, u32, _>(&[u8::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u16, u64, _>(&[u8::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u16, i8, _>(&[u8::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i16, _>(&[u8::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<u16, i32, _>(&[u8::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u16, i64, _>(&[u8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u16_u16_max_test() {
	new_as_by_try_from::<u16, u8, _>(&[u16::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u16, u16, _>(&[u16::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u16, u32, _>(&[u16::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u16, u64, _>(&[u16::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u16, i8, _>(&[u16::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i16, _>(&[u16::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i32, _>(&[u16::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u16, i64, _>(&[u16::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u16_u32_max_test() {
	new_as_by_try_from::<u16, u8, _>(&[u32::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u16, u16, _>(&[u32::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u16, u32, _>(&[u32::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, u64, _>(&[u32::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i8, _>(&[u32::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i16, _>(&[u32::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i32, _>(&[u32::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i64, _>(&[u32::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u16_u64_max_test() {
	new_as_by_try_from::<u16, u8, _>(&[u64::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u16, u16, _>(&[u64::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u16, u32, _>(&[u64::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, u64, _>(&[u64::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i8, _>(&[u64::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i16, _>(&[u64::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i32, _>(&[u64::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i64, _>(&[u64::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u16_i8_max_test() {
	new_as_by_try_from::<u16, u8, _>(&[i8::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u16, u16, _>(&[i8::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u16, u32, _>(&[i8::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u16, u64, _>(&[i8::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u16, i8, _>(&[i8::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<u16, i16, _>(&[i8::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<u16, i32, _>(&[i8::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u16, i64, _>(&[i8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u16_i16_max_test() {
	new_as_by_try_from::<u16, u8, _>(&[i16::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u16, u16, _>(&[i16::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u16, u32, _>(&[i16::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u16, u64, _>(&[i16::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u16, i8, _>(&[i16::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i16, _>(&[i16::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<u16, i32, _>(&[i16::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u16, i64, _>(&[i16::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u16_i32_max_test() {
	new_as_by_try_from::<u16, u8, _>(&[i32::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u16, u16, _>(&[i32::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u16, u32, _>(&[i32::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, u64, _>(&[i32::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i8, _>(&[i32::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i16, _>(&[i32::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i32, _>(&[i32::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i64, _>(&[i32::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u16_i64_max_test() {
	new_as_by_try_from::<u16, u8, _>(&[i64::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u16, u16, _>(&[i64::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u16, u32, _>(&[i64::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, u64, _>(&[i64::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i8, _>(&[i64::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i16, _>(&[i64::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i32, _>(&[i64::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i64, _>(&[i64::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u16_char_max_test() {
	new_as_by_try_from::<u16, u8, _>(&[char::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u16, u16, _>(&[char::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u16, u32, _>(&[char::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, u64, _>(&[char::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i8, _>(&[char::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i16, _>(&[char::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i32, _>(&[char::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i64, _>(&[char::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u32_u8_max_test() {
	new_as_by_try_from::<u32, u8, _>(&[u8::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u32, u16, _>(&[u8::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u32, u32, _>(&[u8::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u32, u64, _>(&[u8::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u32, i8, _>(&[u8::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i16, _>(&[u8::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<u32, i32, _>(&[u8::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u32, i64, _>(&[u8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u32_u16_max_test() {
	new_as_by_try_from::<u32, u8, _>(&[u16::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u32, u16, _>(&[u16::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u32, u32, _>(&[u16::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u32, u64, _>(&[u16::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u32, i8, _>(&[u16::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i16, _>(&[u16::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i32, _>(&[u16::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u32, i64, _>(&[u16::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u32_u32_max_test() {
	new_as_by_try_from::<u32, u8, _>(&[u32::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u32, u16, _>(&[u32::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u32, u32, _>(&[u32::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u32, u64, _>(&[u32::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u32, i8, _>(&[u32::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i16, _>(&[u32::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i32, _>(&[u32::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i64, _>(&[u32::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u32_u64_max_test() {
	new_as_by_try_from::<u32, u8, _>(&[u64::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u32, u16, _>(&[u64::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u32, u32, _>(&[u64::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u32, u64, _>(&[u64::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i8, _>(&[u64::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i16, _>(&[u64::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i32, _>(&[u64::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i64, _>(&[u64::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u32_i8_max_test() {
	new_as_by_try_from::<u32, u8, _>(&[i8::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u32, u16, _>(&[i8::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u32, u32, _>(&[i8::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u32, u64, _>(&[i8::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u32, i8, _>(&[i8::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<u32, i16, _>(&[i8::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<u32, i32, _>(&[i8::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u32, i64, _>(&[i8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u32_i16_max_test() {
	new_as_by_try_from::<u32, u8, _>(&[i16::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u32, u16, _>(&[i16::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u32, u32, _>(&[i16::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u32, u64, _>(&[i16::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u32, i8, _>(&[i16::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i16, _>(&[i16::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<u32, i32, _>(&[i16::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u32, i64, _>(&[i16::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u32_i32_max_test() {
	new_as_by_try_from::<u32, u8, _>(&[i32::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u32, u16, _>(&[i32::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u32, u32, _>(&[i32::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u32, u64, _>(&[i32::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u32, i8, _>(&[i32::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i16, _>(&[i32::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i32, _>(&[i32::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u32, i64, _>(&[i32::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u32_i64_max_test() {
	new_as_by_try_from::<u32, u8, _>(&[i64::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u32, u16, _>(&[i64::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u32, u32, _>(&[i64::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u32, u64, _>(&[i64::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i8, _>(&[i64::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i16, _>(&[i64::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i32, _>(&[i64::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i64, _>(&[i64::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u32_char_max_test() {
	new_as_by_try_from::<u32, u8, _>(&[char::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u32, u16, _>(&[char::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u32, u32, _>(&[char::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u32, u64, _>(&[char::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u32, i8, _>(&[char::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i16, _>(&[char::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i32, _>(&[char::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u32, i64, _>(&[char::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u64_u8_max_test() {
	new_as_by_try_from::<u64, u8, _>(&[u8::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u64, u16, _>(&[u8::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u64, u32, _>(&[u8::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u64, u64, _>(&[u8::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u64, i8, _>(&[u8::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i16, _>(&[u8::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<u64, i32, _>(&[u8::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u64, i64, _>(&[u8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u64_u16_max_test() {
	new_as_by_try_from::<u64, u8, _>(&[u16::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u64, u16, _>(&[u16::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u64, u32, _>(&[u16::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u64, u64, _>(&[u16::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u64, i8, _>(&[u16::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i16, _>(&[u16::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i32, _>(&[u16::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u64, i64, _>(&[u16::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u64_u32_max_test() {
	new_as_by_try_from::<u64, u8, _>(&[u32::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u64, u16, _>(&[u32::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u64, u32, _>(&[u32::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u64, u64, _>(&[u32::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u64, i8, _>(&[u32::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i16, _>(&[u32::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i32, _>(&[u32::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i64, _>(&[u32::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u64_u64_max_test() {
	new_as_by_try_from::<u64, u8, _>(&[u64::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u64, u16, _>(&[u64::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u64, u32, _>(&[u64::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u64, u64, _>(&[u64::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u64, i8, _>(&[u64::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i16, _>(&[u64::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i32, _>(&[u64::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i64, _>(&[u64::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u64_i8_max_test() {
	new_as_by_try_from::<u64, u8, _>(&[i8::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u64, u16, _>(&[i8::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u64, u32, _>(&[i8::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u64, u64, _>(&[i8::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u64, i8, _>(&[i8::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<u64, i16, _>(&[i8::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<u64, i32, _>(&[i8::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u64, i64, _>(&[i8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u64_i16_max_test() {
	new_as_by_try_from::<u64, u8, _>(&[i16::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u64, u16, _>(&[i16::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u64, u32, _>(&[i16::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u64, u64, _>(&[i16::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u64, i8, _>(&[i16::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i16, _>(&[i16::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<u64, i32, _>(&[i16::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u64, i64, _>(&[i16::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u64_i32_max_test() {
	new_as_by_try_from::<u64, u8, _>(&[i32::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u64, u16, _>(&[i32::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u64, u32, _>(&[i32::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u64, u64, _>(&[i32::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u64, i8, _>(&[i32::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i16, _>(&[i32::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i32, _>(&[i32::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u64, i64, _>(&[i32::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u64_i64_max_test() {
	new_as_by_try_from::<u64, u8, _>(&[i64::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u64, u16, _>(&[i64::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u64, u32, _>(&[i64::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u64, u64, _>(&[i64::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u64, i8, _>(&[i64::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i16, _>(&[i64::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i32, _>(&[i64::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i64, _>(&[i64::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u64_char_max_test() {
	new_as_by_try_from::<u64, u8, _>(&[char::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u64, u16, _>(&[char::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u64, u32, _>(&[char::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u64, u64, _>(&[char::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u64, i8, _>(&[char::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i16, _>(&[char::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i32, _>(&[char::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u64, i64, _>(&[char::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i8_u8_max_test() {
	new_as_by_try_from::<i8, u8, _>(&[u8::MAX as u8; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u16, _>(&[u8::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u32, _>(&[u8::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u64, _>(&[u8::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i8, _>(&[u8::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i8, i16, _>(&[u8::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i32, _>(&[u8::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i64, _>(&[u8::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i8_u16_max_test() {
	new_as_by_try_from::<i8, u8, _>(&[u16::MAX as u8; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u16, _>(&[u16::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u32, _>(&[u16::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u64, _>(&[u16::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i8, _>(&[u16::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i8, i16, _>(&[u16::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i8, i32, _>(&[u16::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i64, _>(&[u16::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i8_u32_max_test() {
	new_as_by_try_from::<i8, u8, _>(&[u32::MAX as u8; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u16, _>(&[u32::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u32, _>(&[u32::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u64, _>(&[u32::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i8, _>(&[u32::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i8, i16, _>(&[u32::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i8, i32, _>(&[u32::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i8, i64, _>(&[u32::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i8_u64_max_test() {
	new_as_by_try_from::<i8, u8, _>(&[u64::MAX as u8; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u16, _>(&[u64::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u32, _>(&[u64::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u64, _>(&[u64::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i8, _>(&[u64::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i8, i16, _>(&[u64::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i8, i32, _>(&[u64::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i8, i64, _>(&[u64::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i8_i8_max_test() {
	new_as_by_try_from::<i8, u8, _>(&[i8::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i8, u16, _>(&[i8::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i8, u32, _>(&[i8::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i8, u64, _>(&[i8::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i8, i8, _>(&[i8::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i8, i16, _>(&[i8::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i8, i32, _>(&[i8::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i8, i64, _>(&[i8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i8_i16_max_test() {
	new_as_by_try_from::<i8, u8, _>(&[i16::MAX as u8; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u16, _>(&[i16::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u32, _>(&[i16::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u64, _>(&[i16::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i8, _>(&[i16::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i8, i16, _>(&[i16::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i32, _>(&[i16::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i64, _>(&[i16::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i8_i32_max_test() {
	new_as_by_try_from::<i8, u8, _>(&[i32::MAX as u8; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u16, _>(&[i32::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u32, _>(&[i32::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u64, _>(&[i32::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i8, _>(&[i32::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i8, i16, _>(&[i32::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i8, i32, _>(&[i32::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i64, _>(&[i32::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i8_i64_max_test() {
	new_as_by_try_from::<i8, u8, _>(&[i64::MAX as u8; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u16, _>(&[i64::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u32, _>(&[i64::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u64, _>(&[i64::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i8, _>(&[i64::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i8, i16, _>(&[i64::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i8, i32, _>(&[i64::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i8, i64, _>(&[i64::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i8_char_max_test() {
	new_as_by_try_from::<i8, u8, _>(&[char::MAX as u8; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u16, _>(&[char::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u32, _>(&[char::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u64, _>(&[char::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i8, _>(&[char::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i8, i16, _>(&[char::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i8, i32, _>(&[char::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i64, _>(&[char::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i16_u8_max_test() {
	new_as_by_try_from::<i16, u8, _>(&[u8::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i16, u16, _>(&[u8::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i16, u32, _>(&[u8::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i16, u64, _>(&[u8::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i16, i8, _>(&[u8::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i16, i16, _>(&[u8::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i16, i32, _>(&[u8::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i16, i64, _>(&[u8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i16_u16_max_test() {
	new_as_by_try_from::<i16, u8, _>(&[u16::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i16, u16, _>(&[u16::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, u32, _>(&[u16::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, u64, _>(&[u16::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, i8, _>(&[u16::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i16, i16, _>(&[u16::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i16, i32, _>(&[u16::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, i64, _>(&[u16::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i16_u32_max_test() {
	new_as_by_try_from::<i16, u8, _>(&[u32::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i16, u16, _>(&[u32::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, u32, _>(&[u32::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, u64, _>(&[u32::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, i8, _>(&[u32::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i16, i16, _>(&[u32::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i16, i32, _>(&[u32::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i16, i64, _>(&[u32::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i16_u64_max_test() {
	new_as_by_try_from::<i16, u8, _>(&[u64::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i16, u16, _>(&[u64::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, u32, _>(&[u64::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, u64, _>(&[u64::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, i8, _>(&[u64::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i16, i16, _>(&[u64::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i16, i32, _>(&[u64::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i16, i64, _>(&[u64::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i16_i8_max_test() {
	new_as_by_try_from::<i16, u8, _>(&[i8::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i16, u16, _>(&[i8::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i16, u32, _>(&[i8::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i16, u64, _>(&[i8::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i16, i8, _>(&[i8::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i16, i16, _>(&[i8::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i16, i32, _>(&[i8::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i16, i64, _>(&[i8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i16_i16_max_test() {
	new_as_by_try_from::<i16, u8, _>(&[i16::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i16, u16, _>(&[i16::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i16, u32, _>(&[i16::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i16, u64, _>(&[i16::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i16, i8, _>(&[i16::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i16, i16, _>(&[i16::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i16, i32, _>(&[i16::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i16, i64, _>(&[i16::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i16_i32_max_test() {
	new_as_by_try_from::<i16, u8, _>(&[i32::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i16, u16, _>(&[i32::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, u32, _>(&[i32::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, u64, _>(&[i32::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, i8, _>(&[i32::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i16, i16, _>(&[i32::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i16, i32, _>(&[i32::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, i64, _>(&[i32::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i16_i64_max_test() {
	new_as_by_try_from::<i16, u8, _>(&[i64::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i16, u16, _>(&[i64::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, u32, _>(&[i64::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, u64, _>(&[i64::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, i8, _>(&[i64::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i16, i16, _>(&[i64::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i16, i32, _>(&[i64::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i16, i64, _>(&[i64::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i16_char_max_test() {
	new_as_by_try_from::<i16, u8, _>(&[char::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i16, u16, _>(&[char::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, u32, _>(&[char::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, u64, _>(&[char::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, i8, _>(&[char::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i16, i16, _>(&[char::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i16, i32, _>(&[char::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, i64, _>(&[char::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i32_u8_max_test() {
	new_as_by_try_from::<i32, u8, _>(&[u8::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i32, u16, _>(&[u8::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i32, u32, _>(&[u8::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i32, u64, _>(&[u8::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i32, i8, _>(&[u8::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i32, i16, _>(&[u8::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i32, i32, _>(&[u8::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i32, i64, _>(&[u8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i32_u16_max_test() {
	new_as_by_try_from::<i32, u8, _>(&[u16::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i32, u16, _>(&[u16::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i32, u32, _>(&[u16::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i32, u64, _>(&[u16::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i32, i8, _>(&[u16::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i32, i16, _>(&[u16::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i32, i32, _>(&[u16::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i32, i64, _>(&[u16::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i32_u32_max_test() {
	new_as_by_try_from::<i32, u8, _>(&[u32::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i32, u16, _>(&[u32::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i32, u32, _>(&[u32::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i32, u64, _>(&[u32::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i32, i8, _>(&[u32::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i32, i16, _>(&[u32::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i32, i32, _>(&[u32::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i32, i64, _>(&[u32::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i32_u64_max_test() {
	new_as_by_try_from::<i32, u8, _>(&[u64::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i32, u16, _>(&[u64::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i32, u32, _>(&[u64::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i32, u64, _>(&[u64::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i32, i8, _>(&[u64::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i32, i16, _>(&[u64::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i32, i32, _>(&[u64::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i32, i64, _>(&[u64::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i32_i8_max_test() {
	new_as_by_try_from::<i32, u8, _>(&[i8::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i32, u16, _>(&[i8::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i32, u32, _>(&[i8::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i32, u64, _>(&[i8::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i32, i8, _>(&[i8::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i32, i16, _>(&[i8::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i32, i32, _>(&[i8::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i32, i64, _>(&[i8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i32_i16_max_test() {
	new_as_by_try_from::<i32, u8, _>(&[i16::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i32, u16, _>(&[i16::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i32, u32, _>(&[i16::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i32, u64, _>(&[i16::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i32, i8, _>(&[i16::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i32, i16, _>(&[i16::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i32, i32, _>(&[i16::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i32, i64, _>(&[i16::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i32_i32_max_test() {
	new_as_by_try_from::<i32, u8, _>(&[i32::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i32, u16, _>(&[i32::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i32, u32, _>(&[i32::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i32, u64, _>(&[i32::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i32, i8, _>(&[i32::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i32, i16, _>(&[i32::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i32, i32, _>(&[i32::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i32, i64, _>(&[i32::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i32_i64_max_test() {
	new_as_by_try_from::<i32, u8, _>(&[i64::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i32, u16, _>(&[i64::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i32, u32, _>(&[i64::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i32, u64, _>(&[i64::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i32, i8, _>(&[i64::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i32, i16, _>(&[i64::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i32, i32, _>(&[i64::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i32, i64, _>(&[i64::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i32_char_max_test() {
	new_as_by_try_from::<i32, u8, _>(&[char::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i32, u16, _>(&[char::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i32, u32, _>(&[char::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i32, u64, _>(&[char::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i32, i8, _>(&[char::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i32, i16, _>(&[char::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i32, i32, _>(&[char::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i32, i64, _>(&[char::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i64_u8_max_test() {
	new_as_by_try_from::<i64, u8, _>(&[u8::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i64, u16, _>(&[u8::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i64, u32, _>(&[u8::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i64, u64, _>(&[u8::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i64, i8, _>(&[u8::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i64, i16, _>(&[u8::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i64, i32, _>(&[u8::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i64, i64, _>(&[u8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i64_u16_max_test() {
	new_as_by_try_from::<i64, u8, _>(&[u16::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i64, u16, _>(&[u16::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i64, u32, _>(&[u16::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i64, u64, _>(&[u16::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i64, i8, _>(&[u16::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i64, i16, _>(&[u16::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i64, i32, _>(&[u16::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i64, i64, _>(&[u16::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i64_u32_max_test() {
	new_as_by_try_from::<i64, u8, _>(&[u32::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i64, u16, _>(&[u32::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i64, u32, _>(&[u32::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i64, u64, _>(&[u32::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i64, i8, _>(&[u32::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i64, i16, _>(&[u32::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i64, i32, _>(&[u32::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i64, i64, _>(&[u32::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i64_u64_max_test() {
	new_as_by_try_from::<i64, u8, _>(&[u64::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i64, u16, _>(&[u64::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i64, u32, _>(&[u64::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i64, u64, _>(&[u64::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i64, i8, _>(&[u64::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i64, i16, _>(&[u64::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i64, i32, _>(&[u64::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i64, i64, _>(&[u64::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i64_i8_max_test() {
	new_as_by_try_from::<i64, u8, _>(&[i8::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i64, u16, _>(&[i8::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i64, u32, _>(&[i8::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i64, u64, _>(&[i8::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i64, i8, _>(&[i8::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i64, i16, _>(&[i8::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i64, i32, _>(&[i8::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i64, i64, _>(&[i8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i64_i16_max_test() {
	new_as_by_try_from::<i64, u8, _>(&[i16::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i64, u16, _>(&[i16::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i64, u32, _>(&[i16::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i64, u64, _>(&[i16::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i64, i8, _>(&[i16::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i64, i16, _>(&[i16::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i64, i32, _>(&[i16::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i64, i64, _>(&[i16::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i64_i32_max_test() {
	new_as_by_try_from::<i64, u8, _>(&[i32::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i64, u16, _>(&[i32::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i64, u32, _>(&[i32::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i64, u64, _>(&[i32::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i64, i8, _>(&[i32::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i64, i16, _>(&[i32::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i64, i32, _>(&[i32::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i64, i64, _>(&[i32::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i64_i64_max_test() {
	new_as_by_try_from::<i64, u8, _>(&[i64::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i64, u16, _>(&[i64::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i64, u32, _>(&[i64::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i64, u64, _>(&[i64::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i64, i8, _>(&[i64::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i64, i16, _>(&[i64::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i64, i32, _>(&[i64::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i64, i64, _>(&[i64::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i64_char_max_test() {
	new_as_by_try_from::<i64, u8, _>(&[char::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i64, u16, _>(&[char::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i64, u32, _>(&[char::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i64, u64, _>(&[char::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i64, i8, _>(&[char::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i64, i16, _>(&[char::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i64, i32, _>(&[char::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i64, i64, _>(&[char::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u8_minus_test() {
	new_as_by_try_from::<u8, i8, _>(&[-1; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i16, _>(&[-1; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i32, _>(&[-1; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i64, _>(&[-1; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i8, _>(&[i8::MIN; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i16, _>(&[i16::MIN; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i32, _>(&[i32::MIN; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i64, _>(&[i64::MIN; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u16_minus_test() {
	new_as_by_try_from::<u16, i8, _>(&[-1; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i16, _>(&[-1; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i32, _>(&[-1; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i64, _>(&[-1; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i8, _>(&[i8::MIN; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i16, _>(&[i16::MIN; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i32, _>(&[i32::MIN; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i64, _>(&[i64::MIN; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u32_minus_test() {
	new_as_by_try_from::<u32, i8, _>(&[-1; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i16, _>(&[-1; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i32, _>(&[-1; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i64, _>(&[-1; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i8, _>(&[i8::MIN; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i16, _>(&[i16::MIN; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i32, _>(&[i32::MIN; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i64, _>(&[i64::MIN; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u64_minus_test() {
	new_as_by_try_from::<u64, i8, _>(&[-1; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i16, _>(&[-1; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i32, _>(&[-1; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i64, _>(&[-1; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i8, _>(&[i8::MIN; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i16, _>(&[i16::MIN; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i32, _>(&[i32::MIN; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i64, _>(&[i64::MIN; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i8_minus_test() {
	new_as_by_try_from::<i8, i8, _>(&[-1; 3], 1).unwrap();
	new_as_by_try_from::<i8, i16, _>(&[-1; 3], 1).unwrap();
	new_as_by_try_from::<i8, i32, _>(&[-1; 3], 1).unwrap();
	new_as_by_try_from::<i8, i64, _>(&[-1; 3], 1).unwrap();
	new_as_by_try_from::<i8, i8, _>(&[i8::MIN; 3], 1).unwrap();
	new_as_by_try_from::<i8, i16, _>(&[i16::MIN; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i32, _>(&[i32::MIN; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i64, _>(&[i64::MIN; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i16_minus_test() {
	new_as_by_try_from::<i16, i8, _>(&[-1; 3], 1).unwrap();
	new_as_by_try_from::<i16, i16, _>(&[-1; 3], 1).unwrap();
	new_as_by_try_from::<i16, i32, _>(&[-1; 3], 1).unwrap();
	new_as_by_try_from::<i16, i64, _>(&[-1; 3], 1).unwrap();
	new_as_by_try_from::<i16, i8, _>(&[i8::MIN; 3], 1).unwrap();
	new_as_by_try_from::<i16, i16, _>(&[i16::MIN; 3], 1).unwrap();
	new_as_by_try_from::<i16, i32, _>(&[i32::MIN; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, i64, _>(&[i64::MIN; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i32_minus_test() {
	new_as_by_try_from::<i32, i8, _>(&[-1; 3], 1).unwrap();
	new_as_by_try_from::<i32, i16, _>(&[-1; 3], 1).unwrap();
	new_as_by_try_from::<i32, i32, _>(&[-1; 3], 1).unwrap();
	new_as_by_try_from::<i32, i64, _>(&[-1; 3], 1).unwrap();
	new_as_by_try_from::<i32, i8, _>(&[i8::MIN; 3], 1).unwrap();
	new_as_by_try_from::<i32, i16, _>(&[i16::MIN; 3], 1).unwrap();
	new_as_by_try_from::<i32, i32, _>(&[i32::MIN; 3], 1).unwrap();
	new_as_by_try_from::<i32, i64, _>(&[i64::MIN; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i64_minus_test() {
	new_as_by_try_from::<i64, i8, _>(&[-1; 3], 1).unwrap();
	new_as_by_try_from::<i64, i16, _>(&[-1; 3], 1).unwrap();
	new_as_by_try_from::<i64, i32, _>(&[-1; 3], 1).unwrap();
	new_as_by_try_from::<i64, i64, _>(&[-1; 3], 1).unwrap();
	new_as_by_try_from::<i64, i8, _>(&[i8::MIN; 3], 1).unwrap();
	new_as_by_try_from::<i64, i16, _>(&[i16::MIN; 3], 1).unwrap();
	new_as_by_try_from::<i64, i32, _>(&[i32::MIN; 3], 1).unwrap();
	new_as_by_try_from::<i64, i64, _>(&[i64::MIN; 3], 1).unwrap();
}
