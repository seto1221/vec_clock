use vec_clock::*;

#[test]
fn new_convert_vec_test() {
	let mut vec = vec![0; 3];
	new::<u8>(convert(&vec), 0).unwrap();
	new::<u8>(convert(&mut vec), 0).unwrap();
	new::<u8>(convert(&vec[..]), 0).unwrap();
	new::<u8>(convert(&mut vec[..]), 0).unwrap();
	new::<u8>(convert(vec.as_slice()), 0).unwrap();
	new::<u8>(convert(vec.as_mut_slice()), 0).unwrap();
	new::<u8>(convert(vec), 0).unwrap();
}

#[test]
fn new_try_convert_vec_test() {
	let mut vec = vec![0; 3];
	new::<u8>(try_convert(&vec).unwrap(), 0).unwrap();
	new::<u8>(try_convert(&mut vec).unwrap(), 0).unwrap();
	new::<u8>(try_convert(&vec[..]).unwrap(), 0).unwrap();
	new::<u8>(try_convert(&mut vec[..]).unwrap(), 0).unwrap();
	new::<u8>(try_convert(vec.as_slice()).unwrap(), 0).unwrap();
	new::<u8>(try_convert(vec.as_mut_slice()).unwrap(), 0).unwrap();
	new::<u8>(try_convert(vec).unwrap(), 0).unwrap();
}

#[test]
fn new_convert_array_test() {
	let mut arr = [0; 3];
	new::<u8>(convert(&arr), 0).unwrap();
	new::<u8>(convert(&mut arr), 0).unwrap();
	new::<u8>(convert(&arr[..]), 0).unwrap();
	new::<u8>(convert(&mut arr[..]), 0).unwrap();
	new::<u8>(convert(arr), 0).unwrap();
}

#[test]
fn new_try_convert_array_test() {
	let mut arr = [0; 3];
	new::<u8>(try_convert(&arr).unwrap(), 0).unwrap();
	new::<u8>(try_convert(&mut arr).unwrap(), 0).unwrap();
	new::<u8>(try_convert(&arr[..]).unwrap(), 0).unwrap();
	new::<u8>(try_convert(&mut arr[..]).unwrap(), 0).unwrap();
	new::<u8>(try_convert(arr).unwrap(), 0).unwrap();
}

#[test]
fn convert_empty_test() {
	convert::<u8, u8, _>(&[]);
	convert::<u8, bool, _>(&[]);

	convert::<u16, u8, _>(&[]);
	convert::<u16, u16, _>(&[]);
	convert::<u16, bool, _>(&[]);

	convert::<u32, u8, _>(&[]);
	convert::<u32, u16, _>(&[]);
	convert::<u32, u32, _>(&[]);
	convert::<u32, char, _>(&[]);
	convert::<u32, bool, _>(&[]);

	convert::<u64, u8, _>(&[]);
	convert::<u64, u16, _>(&[]);
	convert::<u64, u32, _>(&[]);
	convert::<u64, u64, _>(&[]);
	convert::<u64, char, _>(&[]);
	convert::<u64, bool, _>(&[]);

	convert::<i8, i8, _>(&[]);
	convert::<i8, bool, _>(&[]);

	convert::<i16, u8, _>(&[]);
	convert::<i16, i8, _>(&[]);
	convert::<i16, i16, _>(&[]);
	convert::<i16, bool, _>(&[]);

	convert::<i32, u8, _>(&[]);
	convert::<i32, u16, _>(&[]);
	convert::<i32, i8, _>(&[]);
	convert::<i32, i16, _>(&[]);
	convert::<i32, i32, _>(&[]);
	convert::<i32, bool, _>(&[]);

	convert::<i64, u8, _>(&[]);
	convert::<i64, u16, _>(&[]);
	convert::<i64, u32, _>(&[]);
	convert::<i64, i8, _>(&[]);
	convert::<i64, i16, _>(&[]);
	convert::<i64, i32, _>(&[]);
	convert::<i64, i64, _>(&[]);
	convert::<i64, bool, _>(&[]);
}

#[test]
fn convert_zero_test() {
	convert::<u8, u8, _>(&[0; 3]);
	convert::<u8, bool, _>(&[false; 3]);

	convert::<u16, u8, _>(&[0; 3]);
	convert::<u16, u16, _>(&[0; 3]);
	convert::<u16, bool, _>(&[false; 3]);

	convert::<u32, u8, _>(&[0; 3]);
	convert::<u32, u16, _>(&[0; 3]);
	convert::<u32, u32, _>(&[0; 3]);
	convert::<u32, char, _>(&[char::from(0); 3]);
	convert::<u32, bool, _>(&[false; 3]);

	convert::<u64, u8, _>(&[0; 3]);
	convert::<u64, u16, _>(&[0; 3]);
	convert::<u64, u32, _>(&[0; 3]);
	convert::<u64, u64, _>(&[0; 3]);
	convert::<u64, char, _>(&[char::from(0); 3]);
	convert::<u64, bool, _>(&[false; 3]);

	convert::<i8, i8, _>(&[0; 3]);
	convert::<i8, bool, _>(&[false; 3]);

	convert::<i16, u8, _>(&[0; 3]);
	convert::<i16, i8, _>(&[0; 3]);
	convert::<i16, i16, _>(&[0; 3]);
	convert::<i16, bool, _>(&[false; 3]);

	convert::<i32, u8, _>(&[0; 3]);
	convert::<i32, u16, _>(&[0; 3]);
	convert::<i32, i8, _>(&[0; 3]);
	convert::<i32, i16, _>(&[0; 3]);
	convert::<i32, i32, _>(&[0; 3]);
	convert::<i32, bool, _>(&[false; 3]);

	convert::<i64, u8, _>(&[0; 3]);
	convert::<i64, u16, _>(&[0; 3]);
	convert::<i64, u32, _>(&[0; 3]);
	convert::<i64, i8, _>(&[0; 3]);
	convert::<i64, i16, _>(&[0; 3]);
	convert::<i64, i32, _>(&[0; 3]);
	convert::<i64, i64, _>(&[0; 3]);
	convert::<i64, bool, _>(&[false; 3]);
}

#[test]
fn convert_one_test() {
	convert::<u8, u8, _>(&[1; 3]);
	convert::<u8, bool, _>(&[true; 3]);

	convert::<u16, u8, _>(&[1; 3]);
	convert::<u16, u16, _>(&[1; 3]);
	convert::<u16, bool, _>(&[true; 3]);

	convert::<u32, u8, _>(&[1; 3]);
	convert::<u32, u16, _>(&[1; 3]);
	convert::<u32, u32, _>(&[1; 3]);
	convert::<u32, char, _>(&[char::from(1); 3]);
	convert::<u32, bool, _>(&[true; 3]);

	convert::<u64, u8, _>(&[1; 3]);
	convert::<u64, u16, _>(&[1; 3]);
	convert::<u64, u32, _>(&[1; 3]);
	convert::<u64, u64, _>(&[1; 3]);
	convert::<u64, char, _>(&[char::from(1); 3]);
	convert::<u64, bool, _>(&[true; 3]);

	convert::<i8, i8, _>(&[1; 3]);
	convert::<i8, bool, _>(&[true; 3]);

	convert::<i16, u8, _>(&[1; 3]);
	convert::<i16, i8, _>(&[1; 3]);
	convert::<i16, i16, _>(&[1; 3]);
	convert::<i16, bool, _>(&[true; 3]);

	convert::<i32, u8, _>(&[1; 3]);
	convert::<i32, u16, _>(&[1; 3]);
	convert::<i32, i8, _>(&[1; 3]);
	convert::<i32, i16, _>(&[1; 3]);
	convert::<i32, i32, _>(&[1; 3]);
	convert::<i32, bool, _>(&[true; 3]);

	convert::<i64, u8, _>(&[1; 3]);
	convert::<i64, u16, _>(&[1; 3]);
	convert::<i64, u32, _>(&[1; 3]);
	convert::<i64, i8, _>(&[1; 3]);
	convert::<i64, i16, _>(&[1; 3]);
	convert::<i64, i32, _>(&[1; 3]);
	convert::<i64, i64, _>(&[1; 3]);
	convert::<i64, bool, _>(&[true; 3]);
}

#[test]
fn convert_u8_max_test() {
	convert::<u8, u8, _>(&[u8::MAX as u8; 3]);

	convert::<u16, u8, _>(&[u8::MAX as u8; 3]);
	convert::<u16, u16, _>(&[u8::MAX as u16; 3]);

	convert::<u32, u8, _>(&[u8::MAX as u8; 3]);
	convert::<u32, u16, _>(&[u8::MAX as u16; 3]);
	convert::<u32, u32, _>(&[u8::MAX as u32; 3]);

	convert::<u64, u8, _>(&[u8::MAX as u8; 3]);
	convert::<u64, u16, _>(&[u8::MAX as u16; 3]);
	convert::<u64, u32, _>(&[u8::MAX as u32; 3]);
	convert::<u64, u64, _>(&[u8::MAX as u64; 3]);

	convert::<i8, i8, _>(&[u8::MAX as i8; 3]);

	convert::<i16, u8, _>(&[u8::MAX as u8; 3]);
	convert::<i16, i8, _>(&[u8::MAX as i8; 3]);
	convert::<i16, i16, _>(&[u8::MAX as i16; 3]);

	convert::<i32, u8, _>(&[u8::MAX as u8; 3]);
	convert::<i32, u16, _>(&[u8::MAX as u16; 3]);
	convert::<i32, i8, _>(&[u8::MAX as i8; 3]);
	convert::<i32, i16, _>(&[u8::MAX as i16; 3]);
	convert::<i32, i32, _>(&[u8::MAX as i32; 3]);

	convert::<i64, u8, _>(&[u8::MAX as u8; 3]);
	convert::<i64, u16, _>(&[u8::MAX as u16; 3]);
	convert::<i64, u32, _>(&[u8::MAX as u32; 3]);
	convert::<i64, i8, _>(&[u8::MAX as i8; 3]);
	convert::<i64, i16, _>(&[u8::MAX as i16; 3]);
	convert::<i64, i32, _>(&[u8::MAX as i32; 3]);
	convert::<i64, i64, _>(&[u8::MAX as i64; 3]);
}

#[test]
fn convert_u16_max_test() {
	convert::<u8, u8, _>(&[u16::MAX as u8; 3]);

	convert::<u16, u8, _>(&[u16::MAX as u8; 3]);
	convert::<u16, u16, _>(&[u16::MAX as u16; 3]);

	convert::<u32, u8, _>(&[u16::MAX as u8; 3]);
	convert::<u32, u16, _>(&[u16::MAX as u16; 3]);
	convert::<u32, u32, _>(&[u16::MAX as u32; 3]);

	convert::<u64, u8, _>(&[u16::MAX as u8; 3]);
	convert::<u64, u16, _>(&[u16::MAX as u16; 3]);
	convert::<u64, u32, _>(&[u16::MAX as u32; 3]);
	convert::<u64, u64, _>(&[u16::MAX as u64; 3]);

	convert::<i8, i8, _>(&[u16::MAX as i8; 3]);

	convert::<i16, u8, _>(&[u16::MAX as u8; 3]);
	convert::<i16, i8, _>(&[u16::MAX as i8; 3]);
	convert::<i16, i16, _>(&[u16::MAX as i16; 3]);

	convert::<i32, u8, _>(&[u16::MAX as u8; 3]);
	convert::<i32, u16, _>(&[u16::MAX as u16; 3]);
	convert::<i32, i8, _>(&[u16::MAX as i8; 3]);
	convert::<i32, i16, _>(&[u16::MAX as i16; 3]);
	convert::<i32, i32, _>(&[u16::MAX as i32; 3]);

	convert::<i64, u8, _>(&[u16::MAX as u8; 3]);
	convert::<i64, u16, _>(&[u16::MAX as u16; 3]);
	convert::<i64, u32, _>(&[u16::MAX as u32; 3]);
	convert::<i64, i8, _>(&[u16::MAX as i8; 3]);
	convert::<i64, i16, _>(&[u16::MAX as i16; 3]);
	convert::<i64, i32, _>(&[u16::MAX as i32; 3]);
	convert::<i64, i64, _>(&[u16::MAX as i64; 3]);
}

#[test]
fn convert_u32_max_test() {
	convert::<u8, u8, _>(&[u32::MAX as u8; 3]);

	convert::<u16, u8, _>(&[u32::MAX as u8; 3]);
	convert::<u16, u16, _>(&[u32::MAX as u16; 3]);

	convert::<u32, u8, _>(&[u32::MAX as u8; 3]);
	convert::<u32, u16, _>(&[u32::MAX as u16; 3]);
	convert::<u32, u32, _>(&[u32::MAX as u32; 3]);

	convert::<u64, u8, _>(&[u32::MAX as u8; 3]);
	convert::<u64, u16, _>(&[u32::MAX as u16; 3]);
	convert::<u64, u32, _>(&[u32::MAX as u32; 3]);
	convert::<u64, u64, _>(&[u32::MAX as u64; 3]);

	convert::<i8, i8, _>(&[u32::MAX as i8; 3]);

	convert::<i16, u8, _>(&[u32::MAX as u8; 3]);
	convert::<i16, i8, _>(&[u32::MAX as i8; 3]);
	convert::<i16, i16, _>(&[u32::MAX as i16; 3]);

	convert::<i32, u8, _>(&[u32::MAX as u8; 3]);
	convert::<i32, u16, _>(&[u32::MAX as u16; 3]);
	convert::<i32, i8, _>(&[u32::MAX as i8; 3]);
	convert::<i32, i16, _>(&[u32::MAX as i16; 3]);
	convert::<i32, i32, _>(&[u32::MAX as i32; 3]);

	convert::<i64, u8, _>(&[u32::MAX as u8; 3]);
	convert::<i64, u16, _>(&[u32::MAX as u16; 3]);
	convert::<i64, u32, _>(&[u32::MAX as u32; 3]);
	convert::<i64, i8, _>(&[u32::MAX as i8; 3]);
	convert::<i64, i16, _>(&[u32::MAX as i16; 3]);
	convert::<i64, i32, _>(&[u32::MAX as i32; 3]);
	convert::<i64, i64, _>(&[u32::MAX as i64; 3]);
}

#[test]
fn convert_u64_max_test() {
	convert::<u8, u8, _>(&[u64::MAX as u8; 3]);

	convert::<u16, u8, _>(&[u64::MAX as u8; 3]);
	convert::<u16, u16, _>(&[u64::MAX as u16; 3]);

	convert::<u32, u8, _>(&[u64::MAX as u8; 3]);
	convert::<u32, u16, _>(&[u64::MAX as u16; 3]);
	convert::<u32, u32, _>(&[u64::MAX as u32; 3]);

	convert::<u64, u8, _>(&[u64::MAX as u8; 3]);
	convert::<u64, u16, _>(&[u64::MAX as u16; 3]);
	convert::<u64, u32, _>(&[u64::MAX as u32; 3]);
	convert::<u64, u64, _>(&[u64::MAX as u64; 3]);

	convert::<i8, i8, _>(&[u64::MAX as i8; 3]);

	convert::<i16, u8, _>(&[u64::MAX as u8; 3]);
	convert::<i16, i8, _>(&[u64::MAX as i8; 3]);
	convert::<i16, i16, _>(&[u64::MAX as i16; 3]);

	convert::<i32, u8, _>(&[u64::MAX as u8; 3]);
	convert::<i32, u16, _>(&[u64::MAX as u16; 3]);
	convert::<i32, i8, _>(&[u64::MAX as i8; 3]);
	convert::<i32, i16, _>(&[u64::MAX as i16; 3]);
	convert::<i32, i32, _>(&[u64::MAX as i32; 3]);

	convert::<i64, u8, _>(&[u64::MAX as u8; 3]);
	convert::<i64, u16, _>(&[u64::MAX as u16; 3]);
	convert::<i64, u32, _>(&[u64::MAX as u32; 3]);
	convert::<i64, i8, _>(&[u64::MAX as i8; 3]);
	convert::<i64, i16, _>(&[u64::MAX as i16; 3]);
	convert::<i64, i32, _>(&[u64::MAX as i32; 3]);
	convert::<i64, i64, _>(&[u64::MAX as i64; 3]);
}

#[test]
fn convert_i8_max_test() {
	convert::<u8, u8, _>(&[i8::MAX as u8; 3]);

	convert::<u16, u8, _>(&[i8::MAX as u8; 3]);
	convert::<u16, u16, _>(&[i8::MAX as u16; 3]);

	convert::<u32, u8, _>(&[i8::MAX as u8; 3]);
	convert::<u32, u16, _>(&[i8::MAX as u16; 3]);
	convert::<u32, u32, _>(&[i8::MAX as u32; 3]);

	convert::<u64, u8, _>(&[i8::MAX as u8; 3]);
	convert::<u64, u16, _>(&[i8::MAX as u16; 3]);
	convert::<u64, u32, _>(&[i8::MAX as u32; 3]);
	convert::<u64, u64, _>(&[i8::MAX as u64; 3]);

	convert::<i8, i8, _>(&[i8::MAX as i8; 3]);

	convert::<i16, u8, _>(&[i8::MAX as u8; 3]);
	convert::<i16, i8, _>(&[i8::MAX as i8; 3]);
	convert::<i16, i16, _>(&[i8::MAX as i16; 3]);

	convert::<i32, u8, _>(&[i8::MAX as u8; 3]);
	convert::<i32, u16, _>(&[i8::MAX as u16; 3]);
	convert::<i32, i8, _>(&[i8::MAX as i8; 3]);
	convert::<i32, i16, _>(&[i8::MAX as i16; 3]);
	convert::<i32, i32, _>(&[i8::MAX as i32; 3]);

	convert::<i64, u8, _>(&[i8::MAX as u8; 3]);
	convert::<i64, u16, _>(&[i8::MAX as u16; 3]);
	convert::<i64, u32, _>(&[i8::MAX as u32; 3]);
	convert::<i64, i8, _>(&[i8::MAX as i8; 3]);
	convert::<i64, i16, _>(&[i8::MAX as i16; 3]);
	convert::<i64, i32, _>(&[i8::MAX as i32; 3]);
	convert::<i64, i64, _>(&[i8::MAX as i64; 3]);
}

#[test]
fn convert_i16_max_test() {
	convert::<u8, u8, _>(&[i16::MAX as u8; 3]);

	convert::<u16, u8, _>(&[i16::MAX as u8; 3]);
	convert::<u16, u16, _>(&[i16::MAX as u16; 3]);

	convert::<u32, u8, _>(&[i16::MAX as u8; 3]);
	convert::<u32, u16, _>(&[i16::MAX as u16; 3]);
	convert::<u32, u32, _>(&[i16::MAX as u32; 3]);

	convert::<u64, u8, _>(&[i16::MAX as u8; 3]);
	convert::<u64, u16, _>(&[i16::MAX as u16; 3]);
	convert::<u64, u32, _>(&[i16::MAX as u32; 3]);
	convert::<u64, u64, _>(&[i16::MAX as u64; 3]);

	convert::<i8, i8, _>(&[i16::MAX as i8; 3]);

	convert::<i16, u8, _>(&[i16::MAX as u8; 3]);
	convert::<i16, i8, _>(&[i16::MAX as i8; 3]);
	convert::<i16, i16, _>(&[i16::MAX as i16; 3]);

	convert::<i32, u8, _>(&[i16::MAX as u8; 3]);
	convert::<i32, u16, _>(&[i16::MAX as u16; 3]);
	convert::<i32, i8, _>(&[i16::MAX as i8; 3]);
	convert::<i32, i16, _>(&[i16::MAX as i16; 3]);
	convert::<i32, i32, _>(&[i16::MAX as i32; 3]);

	convert::<i64, u8, _>(&[i16::MAX as u8; 3]);
	convert::<i64, u16, _>(&[i16::MAX as u16; 3]);
	convert::<i64, u32, _>(&[i16::MAX as u32; 3]);
	convert::<i64, i8, _>(&[i16::MAX as i8; 3]);
	convert::<i64, i16, _>(&[i16::MAX as i16; 3]);
	convert::<i64, i32, _>(&[i16::MAX as i32; 3]);
	convert::<i64, i64, _>(&[i16::MAX as i64; 3]);
}

#[test]
fn convert_i32_max_test() {
	convert::<u8, u8, _>(&[i32::MAX as u8; 3]);

	convert::<u16, u8, _>(&[i32::MAX as u8; 3]);
	convert::<u16, u16, _>(&[i32::MAX as u16; 3]);

	convert::<u32, u8, _>(&[i32::MAX as u8; 3]);
	convert::<u32, u16, _>(&[i32::MAX as u16; 3]);
	convert::<u32, u32, _>(&[i32::MAX as u32; 3]);

	convert::<u64, u8, _>(&[i32::MAX as u8; 3]);
	convert::<u64, u16, _>(&[i32::MAX as u16; 3]);
	convert::<u64, u32, _>(&[i32::MAX as u32; 3]);
	convert::<u64, u64, _>(&[i32::MAX as u64; 3]);

	convert::<i8, i8, _>(&[i32::MAX as i8; 3]);

	convert::<i16, u8, _>(&[i32::MAX as u8; 3]);
	convert::<i16, i8, _>(&[i32::MAX as i8; 3]);
	convert::<i16, i16, _>(&[i32::MAX as i16; 3]);

	convert::<i32, u8, _>(&[i32::MAX as u8; 3]);
	convert::<i32, u16, _>(&[i32::MAX as u16; 3]);
	convert::<i32, i8, _>(&[i32::MAX as i8; 3]);
	convert::<i32, i16, _>(&[i32::MAX as i16; 3]);
	convert::<i32, i32, _>(&[i32::MAX as i32; 3]);

	convert::<i64, u8, _>(&[i32::MAX as u8; 3]);
	convert::<i64, u16, _>(&[i32::MAX as u16; 3]);
	convert::<i64, u32, _>(&[i32::MAX as u32; 3]);
	convert::<i64, i8, _>(&[i32::MAX as i8; 3]);
	convert::<i64, i16, _>(&[i32::MAX as i16; 3]);
	convert::<i64, i32, _>(&[i32::MAX as i32; 3]);
	convert::<i64, i64, _>(&[i32::MAX as i64; 3]);
}

#[test]
fn convert_i64_max_test() {
	convert::<u8, u8, _>(&[i64::MAX as u8; 3]);

	convert::<u16, u8, _>(&[i64::MAX as u8; 3]);
	convert::<u16, u16, _>(&[i64::MAX as u16; 3]);

	convert::<u32, u8, _>(&[i64::MAX as u8; 3]);
	convert::<u32, u16, _>(&[i64::MAX as u16; 3]);
	convert::<u32, u32, _>(&[i64::MAX as u32; 3]);

	convert::<u64, u8, _>(&[i64::MAX as u8; 3]);
	convert::<u64, u16, _>(&[i64::MAX as u16; 3]);
	convert::<u64, u32, _>(&[i64::MAX as u32; 3]);
	convert::<u64, u64, _>(&[i64::MAX as u64; 3]);

	convert::<i8, i8, _>(&[i64::MAX as i8; 3]);

	convert::<i16, u8, _>(&[i64::MAX as u8; 3]);
	convert::<i16, i8, _>(&[i64::MAX as i8; 3]);
	convert::<i16, i16, _>(&[i64::MAX as i16; 3]);

	convert::<i32, u8, _>(&[i64::MAX as u8; 3]);
	convert::<i32, u16, _>(&[i64::MAX as u16; 3]);
	convert::<i32, i8, _>(&[i64::MAX as i8; 3]);
	convert::<i32, i16, _>(&[i64::MAX as i16; 3]);
	convert::<i32, i32, _>(&[i64::MAX as i32; 3]);

	convert::<i64, u8, _>(&[i64::MAX as u8; 3]);
	convert::<i64, u16, _>(&[i64::MAX as u16; 3]);
	convert::<i64, u32, _>(&[i64::MAX as u32; 3]);
	convert::<i64, i8, _>(&[i64::MAX as i8; 3]);
	convert::<i64, i16, _>(&[i64::MAX as i16; 3]);
	convert::<i64, i32, _>(&[i64::MAX as i32; 3]);
	convert::<i64, i64, _>(&[i64::MAX as i64; 3]);
}

#[test]
fn convert_char_max_test() {
	convert::<u8, u8, _>(&[char::MAX as u8; 3]);

	convert::<u16, u8, _>(&[char::MAX as u8; 3]);
	convert::<u16, u16, _>(&[char::MAX as u16; 3]);

	convert::<u32, u8, _>(&[char::MAX as u8; 3]);
	convert::<u32, u16, _>(&[char::MAX as u16; 3]);
	convert::<u32, u32, _>(&[char::MAX as u32; 3]);

	convert::<u64, u8, _>(&[char::MAX as u8; 3]);
	convert::<u64, u16, _>(&[char::MAX as u16; 3]);
	convert::<u64, u32, _>(&[char::MAX as u32; 3]);
	convert::<u64, u64, _>(&[char::MAX as u64; 3]);

	convert::<i8, i8, _>(&[char::MAX as i8; 3]);

	convert::<i16, u8, _>(&[char::MAX as u8; 3]);
	convert::<i16, i8, _>(&[char::MAX as i8; 3]);
	convert::<i16, i16, _>(&[char::MAX as i16; 3]);

	convert::<i32, u8, _>(&[char::MAX as u8; 3]);
	convert::<i32, u16, _>(&[char::MAX as u16; 3]);
	convert::<i32, i8, _>(&[char::MAX as i8; 3]);
	convert::<i32, i16, _>(&[char::MAX as i16; 3]);
	convert::<i32, i32, _>(&[char::MAX as i32; 3]);

	convert::<i64, u8, _>(&[char::MAX as u8; 3]);
	convert::<i64, u16, _>(&[char::MAX as u16; 3]);
	convert::<i64, u32, _>(&[char::MAX as u32; 3]);
	convert::<i64, i8, _>(&[char::MAX as i8; 3]);
	convert::<i64, i16, _>(&[char::MAX as i16; 3]);
	convert::<i64, i32, _>(&[char::MAX as i32; 3]);
	convert::<i64, i64, _>(&[char::MAX as i64; 3]);
}

#[test]
fn convert_minus_test() {
	convert::<i8, i8, _>(&[-1; 3]);
	convert::<i8, i8, _>(&[i8::MIN; 3]);

	convert::<i16, i8, _>(&[-1; 3]);
	convert::<i16, i16, _>(&[-1; 3]);
	convert::<i16, i8, _>(&[i8::MIN; 3]);
	convert::<i16, i16, _>(&[i16::MIN; 3]);

	convert::<i32, i8, _>(&[-1; 3]);
	convert::<i32, i16, _>(&[-1; 3]);
	convert::<i32, i32, _>(&[-1; 3]);
	convert::<i32, i8, _>(&[i8::MIN; 3]);
	convert::<i32, i16, _>(&[i16::MIN; 3]);
	convert::<i32, i32, _>(&[i32::MIN; 3]);

	convert::<i64, i8, _>(&[-1; 3]);
	convert::<i64, i16, _>(&[-1; 3]);
	convert::<i64, i32, _>(&[-1; 3]);
	convert::<i64, i64, _>(&[-1; 3]);
	convert::<i64, i8, _>(&[i8::MIN; 3]);
	convert::<i64, i16, _>(&[i16::MIN; 3]);
	convert::<i64, i32, _>(&[i32::MIN; 3]);
	convert::<i64, i64, _>(&[i64::MIN; 3]);
}

#[test]
fn try_convert_u8_empty_test() {
	try_convert::<u8, u8, _>(&[]).unwrap();
	try_convert::<u8, u16, _>(&[]).unwrap();
	try_convert::<u8, u32, _>(&[]).unwrap();
	try_convert::<u8, u64, _>(&[]).unwrap();
	try_convert::<u8, i8, _>(&[]).unwrap();
	try_convert::<u8, i16, _>(&[]).unwrap();
	try_convert::<u8, i32, _>(&[]).unwrap();
	try_convert::<u8, i64, _>(&[]).unwrap();
	try_convert::<u8, char, _>(&[]).unwrap();
	try_convert::<u8, bool, _>(&[]).unwrap();
}

#[test]
fn try_convert_u16_empty_test() {
	try_convert::<u16, u8, _>(&[]).unwrap();
	try_convert::<u16, u16, _>(&[]).unwrap();
	try_convert::<u16, u32, _>(&[]).unwrap();
	try_convert::<u16, u64, _>(&[]).unwrap();
	try_convert::<u16, i8, _>(&[]).unwrap();
	try_convert::<u16, i16, _>(&[]).unwrap();
	try_convert::<u16, i32, _>(&[]).unwrap();
	try_convert::<u16, i64, _>(&[]).unwrap();
	try_convert::<u16, char, _>(&[]).unwrap();
	try_convert::<u16, bool, _>(&[]).unwrap();
}

#[test]
fn try_convert_u32_empty_test() {
	try_convert::<u32, u8, _>(&[]).unwrap();
	try_convert::<u32, u16, _>(&[]).unwrap();
	try_convert::<u32, u32, _>(&[]).unwrap();
	try_convert::<u32, u64, _>(&[]).unwrap();
	try_convert::<u32, i8, _>(&[]).unwrap();
	try_convert::<u32, i16, _>(&[]).unwrap();
	try_convert::<u32, i32, _>(&[]).unwrap();
	try_convert::<u32, i64, _>(&[]).unwrap();
	try_convert::<u32, char, _>(&[]).unwrap();
	try_convert::<u32, bool, _>(&[]).unwrap();
}

#[test]
fn try_convert_u64_empty_test() {
	try_convert::<u64, u8, _>(&[]).unwrap();
	try_convert::<u64, u16, _>(&[]).unwrap();
	try_convert::<u64, u32, _>(&[]).unwrap();
	try_convert::<u64, u64, _>(&[]).unwrap();
	try_convert::<u64, i8, _>(&[]).unwrap();
	try_convert::<u64, i16, _>(&[]).unwrap();
	try_convert::<u64, i32, _>(&[]).unwrap();
	try_convert::<u64, i64, _>(&[]).unwrap();
	try_convert::<u64, char, _>(&[]).unwrap();
	try_convert::<u64, bool, _>(&[]).unwrap();
}

#[test]
fn try_convert_i8_empty_test() {
	try_convert::<i8, u8, _>(&[]).unwrap();
	try_convert::<i8, u16, _>(&[]).unwrap();
	try_convert::<i8, u32, _>(&[]).unwrap();
	try_convert::<i8, u64, _>(&[]).unwrap();
	try_convert::<i8, i8, _>(&[]).unwrap();
	try_convert::<i8, i16, _>(&[]).unwrap();
	try_convert::<i8, i32, _>(&[]).unwrap();
	try_convert::<i8, i64, _>(&[]).unwrap();
	try_convert::<i8, bool, _>(&[]).unwrap();
}

#[test]
fn try_convert_i16_empty_test() {
	try_convert::<i16, u8, _>(&[]).unwrap();
	try_convert::<i16, u16, _>(&[]).unwrap();
	try_convert::<i16, u32, _>(&[]).unwrap();
	try_convert::<i16, u64, _>(&[]).unwrap();
	try_convert::<i16, i8, _>(&[]).unwrap();
	try_convert::<i16, i16, _>(&[]).unwrap();
	try_convert::<i16, i32, _>(&[]).unwrap();
	try_convert::<i16, i64, _>(&[]).unwrap();
	try_convert::<i16, bool, _>(&[]).unwrap();
}

#[test]
fn try_convert_i32_empty_test() {
	try_convert::<i32, u8, _>(&[]).unwrap();
	try_convert::<i32, u16, _>(&[]).unwrap();
	try_convert::<i32, u32, _>(&[]).unwrap();
	try_convert::<i32, u64, _>(&[]).unwrap();
	try_convert::<i32, i8, _>(&[]).unwrap();
	try_convert::<i32, i16, _>(&[]).unwrap();
	try_convert::<i32, i32, _>(&[]).unwrap();
	try_convert::<i32, i64, _>(&[]).unwrap();
	try_convert::<i32, bool, _>(&[]).unwrap();
}

#[test]
fn try_convert_i64_empty_test() {
	try_convert::<i64, u8, _>(&[]).unwrap();
	try_convert::<i64, u16, _>(&[]).unwrap();
	try_convert::<i64, u32, _>(&[]).unwrap();
	try_convert::<i64, u64, _>(&[]).unwrap();
	try_convert::<i64, i8, _>(&[]).unwrap();
	try_convert::<i64, i16, _>(&[]).unwrap();
	try_convert::<i64, i32, _>(&[]).unwrap();
	try_convert::<i64, i64, _>(&[]).unwrap();
	try_convert::<i64, bool, _>(&[]).unwrap();
}

#[test]
fn try_convert_u8_zero_test() {
	try_convert::<u8, u8, _>(&[0; 3]).unwrap();
	try_convert::<u8, u16, _>(&[0; 3]).unwrap();
	try_convert::<u8, u32, _>(&[0; 3]).unwrap();
	try_convert::<u8, u64, _>(&[0; 3]).unwrap();
	try_convert::<u8, i8, _>(&[0; 3]).unwrap();
	try_convert::<u8, i16, _>(&[0; 3]).unwrap();
	try_convert::<u8, i32, _>(&[0; 3]).unwrap();
	try_convert::<u8, i64, _>(&[0; 3]).unwrap();
	try_convert::<u8, char, _>(&[char::from(0); 3]).unwrap();
	try_convert::<u8, bool, _>(&[false; 3]).unwrap();
}

#[test]
fn try_convert_u16_zero_test() {
	try_convert::<u16, u8, _>(&[0; 3]).unwrap();
	try_convert::<u16, u16, _>(&[0; 3]).unwrap();
	try_convert::<u16, u32, _>(&[0; 3]).unwrap();
	try_convert::<u16, u64, _>(&[0; 3]).unwrap();
	try_convert::<u16, i8, _>(&[0; 3]).unwrap();
	try_convert::<u16, i16, _>(&[0; 3]).unwrap();
	try_convert::<u16, i32, _>(&[0; 3]).unwrap();
	try_convert::<u16, i64, _>(&[0; 3]).unwrap();
	try_convert::<u16, char, _>(&[char::from(0); 3]).unwrap();
	try_convert::<u16, bool, _>(&[false; 3]).unwrap();
}

#[test]
fn try_convert_u32_zero_test() {
	try_convert::<u32, u8, _>(&[0; 3]).unwrap();
	try_convert::<u32, u16, _>(&[0; 3]).unwrap();
	try_convert::<u32, u32, _>(&[0; 3]).unwrap();
	try_convert::<u32, u64, _>(&[0; 3]).unwrap();
	try_convert::<u32, i8, _>(&[0; 3]).unwrap();
	try_convert::<u32, i16, _>(&[0; 3]).unwrap();
	try_convert::<u32, i32, _>(&[0; 3]).unwrap();
	try_convert::<u32, i64, _>(&[0; 3]).unwrap();
	try_convert::<u32, char, _>(&[char::from(0); 3]).unwrap();
	try_convert::<u32, bool, _>(&[false; 3]).unwrap();
}

#[test]
fn try_convert_u64_zero_test() {
	try_convert::<u64, u8, _>(&[0; 3]).unwrap();
	try_convert::<u64, u16, _>(&[0; 3]).unwrap();
	try_convert::<u64, u32, _>(&[0; 3]).unwrap();
	try_convert::<u64, u64, _>(&[0; 3]).unwrap();
	try_convert::<u64, i8, _>(&[0; 3]).unwrap();
	try_convert::<u64, i16, _>(&[0; 3]).unwrap();
	try_convert::<u64, i32, _>(&[0; 3]).unwrap();
	try_convert::<u64, i64, _>(&[0; 3]).unwrap();
	try_convert::<u64, char, _>(&[char::from(0); 3]).unwrap();
	try_convert::<u64, bool, _>(&[false; 3]).unwrap();
}

#[test]
fn try_convert_i8_zero_test() {
	try_convert::<i8, u8, _>(&[0; 3]).unwrap();
	try_convert::<i8, u16, _>(&[0; 3]).unwrap();
	try_convert::<i8, u32, _>(&[0; 3]).unwrap();
	try_convert::<i8, u64, _>(&[0; 3]).unwrap();
	try_convert::<i8, i8, _>(&[0; 3]).unwrap();
	try_convert::<i8, i16, _>(&[0; 3]).unwrap();
	try_convert::<i8, i32, _>(&[0; 3]).unwrap();
	try_convert::<i8, i64, _>(&[0; 3]).unwrap();
	try_convert::<i8, bool, _>(&[false; 3]).unwrap();
}

#[test]
fn try_convert_i16_zero_test() {
	try_convert::<i16, u8, _>(&[0; 3]).unwrap();
	try_convert::<i16, u16, _>(&[0; 3]).unwrap();
	try_convert::<i16, u32, _>(&[0; 3]).unwrap();
	try_convert::<i16, u64, _>(&[0; 3]).unwrap();
	try_convert::<i16, i8, _>(&[0; 3]).unwrap();
	try_convert::<i16, i16, _>(&[0; 3]).unwrap();
	try_convert::<i16, i32, _>(&[0; 3]).unwrap();
	try_convert::<i16, i64, _>(&[0; 3]).unwrap();
	try_convert::<i16, bool, _>(&[false; 3]).unwrap();
}

#[test]
fn try_convert_i32_zero_test() {
	try_convert::<i32, u8, _>(&[0; 3]).unwrap();
	try_convert::<i32, u16, _>(&[0; 3]).unwrap();
	try_convert::<i32, u32, _>(&[0; 3]).unwrap();
	try_convert::<i32, u64, _>(&[0; 3]).unwrap();
	try_convert::<i32, i8, _>(&[0; 3]).unwrap();
	try_convert::<i32, i16, _>(&[0; 3]).unwrap();
	try_convert::<i32, i32, _>(&[0; 3]).unwrap();
	try_convert::<i32, i64, _>(&[0; 3]).unwrap();
	try_convert::<i32, bool, _>(&[false; 3]).unwrap();
}

#[test]
fn try_convert_i64_zero_test() {
	try_convert::<i64, u8, _>(&[0; 3]).unwrap();
	try_convert::<i64, u16, _>(&[0; 3]).unwrap();
	try_convert::<i64, u32, _>(&[0; 3]).unwrap();
	try_convert::<i64, u64, _>(&[0; 3]).unwrap();
	try_convert::<i64, i8, _>(&[0; 3]).unwrap();
	try_convert::<i64, i16, _>(&[0; 3]).unwrap();
	try_convert::<i64, i32, _>(&[0; 3]).unwrap();
	try_convert::<i64, i64, _>(&[0; 3]).unwrap();
	try_convert::<i64, bool, _>(&[false; 3]).unwrap();
}

#[test]
fn try_convert_u8_one_test() {
	try_convert::<u8, u8, _>(&[1; 3]).unwrap();
	try_convert::<u8, u16, _>(&[1; 3]).unwrap();
	try_convert::<u8, u32, _>(&[1; 3]).unwrap();
	try_convert::<u8, u64, _>(&[1; 3]).unwrap();
	try_convert::<u8, i8, _>(&[1; 3]).unwrap();
	try_convert::<u8, i16, _>(&[1; 3]).unwrap();
	try_convert::<u8, i32, _>(&[1; 3]).unwrap();
	try_convert::<u8, i64, _>(&[1; 3]).unwrap();
	try_convert::<u8, char, _>(&[char::from(1); 3]).unwrap();
	try_convert::<u8, bool, _>(&[true; 3]).unwrap();
}

#[test]
fn try_convert_u16_one_test() {
	try_convert::<u16, u8, _>(&[1; 3]).unwrap();
	try_convert::<u16, u16, _>(&[1; 3]).unwrap();
	try_convert::<u16, u32, _>(&[1; 3]).unwrap();
	try_convert::<u16, u64, _>(&[1; 3]).unwrap();
	try_convert::<u16, i8, _>(&[1; 3]).unwrap();
	try_convert::<u16, i16, _>(&[1; 3]).unwrap();
	try_convert::<u16, i32, _>(&[1; 3]).unwrap();
	try_convert::<u16, i64, _>(&[1; 3]).unwrap();
	try_convert::<u16, char, _>(&[char::from(1); 3]).unwrap();
	try_convert::<u16, bool, _>(&[true; 3]).unwrap();
}

#[test]
fn try_convert_u32_one_test() {
	try_convert::<u32, u8, _>(&[1; 3]).unwrap();
	try_convert::<u32, u16, _>(&[1; 3]).unwrap();
	try_convert::<u32, u32, _>(&[1; 3]).unwrap();
	try_convert::<u32, u64, _>(&[1; 3]).unwrap();
	try_convert::<u32, i8, _>(&[1; 3]).unwrap();
	try_convert::<u32, i16, _>(&[1; 3]).unwrap();
	try_convert::<u32, i32, _>(&[1; 3]).unwrap();
	try_convert::<u32, i64, _>(&[1; 3]).unwrap();
	try_convert::<u32, char, _>(&[char::from(1); 3]).unwrap();
	try_convert::<u32, bool, _>(&[true; 3]).unwrap();
}

#[test]
fn try_convert_u64_one_test() {
	try_convert::<u64, u8, _>(&[1; 3]).unwrap();
	try_convert::<u64, u16, _>(&[1; 3]).unwrap();
	try_convert::<u64, u32, _>(&[1; 3]).unwrap();
	try_convert::<u64, u64, _>(&[1; 3]).unwrap();
	try_convert::<u64, i8, _>(&[1; 3]).unwrap();
	try_convert::<u64, i16, _>(&[1; 3]).unwrap();
	try_convert::<u64, i32, _>(&[1; 3]).unwrap();
	try_convert::<u64, i64, _>(&[1; 3]).unwrap();
	try_convert::<u64, char, _>(&[char::from(1); 3]).unwrap();
	try_convert::<u64, bool, _>(&[true; 3]).unwrap();
}

#[test]
fn try_convert_i8_one_test() {
	try_convert::<i8, u8, _>(&[1; 3]).unwrap();
	try_convert::<i8, u16, _>(&[1; 3]).unwrap();
	try_convert::<i8, u32, _>(&[1; 3]).unwrap();
	try_convert::<i8, u64, _>(&[1; 3]).unwrap();
	try_convert::<i8, i8, _>(&[1; 3]).unwrap();
	try_convert::<i8, i16, _>(&[1; 3]).unwrap();
	try_convert::<i8, i32, _>(&[1; 3]).unwrap();
	try_convert::<i8, i64, _>(&[1; 3]).unwrap();
	try_convert::<i8, bool, _>(&[true; 3]).unwrap();
}

#[test]
fn try_convert_i16_one_test() {
	try_convert::<i16, u8, _>(&[1; 3]).unwrap();
	try_convert::<i16, u16, _>(&[1; 3]).unwrap();
	try_convert::<i16, u32, _>(&[1; 3]).unwrap();
	try_convert::<i16, u64, _>(&[1; 3]).unwrap();
	try_convert::<i16, i8, _>(&[1; 3]).unwrap();
	try_convert::<i16, i16, _>(&[1; 3]).unwrap();
	try_convert::<i16, i32, _>(&[1; 3]).unwrap();
	try_convert::<i16, i64, _>(&[1; 3]).unwrap();
	try_convert::<i16, bool, _>(&[true; 3]).unwrap();
}

#[test]
fn try_convert_i32_one_test() {
	try_convert::<i32, u8, _>(&[1; 3]).unwrap();
	try_convert::<i32, u16, _>(&[1; 3]).unwrap();
	try_convert::<i32, u32, _>(&[1; 3]).unwrap();
	try_convert::<i32, u64, _>(&[1; 3]).unwrap();
	try_convert::<i32, i8, _>(&[1; 3]).unwrap();
	try_convert::<i32, i16, _>(&[1; 3]).unwrap();
	try_convert::<i32, i32, _>(&[1; 3]).unwrap();
	try_convert::<i32, i64, _>(&[1; 3]).unwrap();
	try_convert::<i32, bool, _>(&[true; 3]).unwrap();
}

#[test]
fn try_convert_i64_one_test() {
	try_convert::<i64, u8, _>(&[1; 3]).unwrap();
	try_convert::<i64, u16, _>(&[1; 3]).unwrap();
	try_convert::<i64, u32, _>(&[1; 3]).unwrap();
	try_convert::<i64, u64, _>(&[1; 3]).unwrap();
	try_convert::<i64, i8, _>(&[1; 3]).unwrap();
	try_convert::<i64, i16, _>(&[1; 3]).unwrap();
	try_convert::<i64, i32, _>(&[1; 3]).unwrap();
	try_convert::<i64, i64, _>(&[1; 3]).unwrap();
	try_convert::<i64, bool, _>(&[true; 3]).unwrap();
}

#[test]
fn try_convert_u8_u8_max_test() {
	try_convert::<u8, u8, _>(&[u8::MAX as u8; 3]).unwrap();
	try_convert::<u8, u16, _>(&[u8::MAX as u16; 3]).unwrap();
	try_convert::<u8, u32, _>(&[u8::MAX as u32; 3]).unwrap();
	try_convert::<u8, u64, _>(&[u8::MAX as u64; 3]).unwrap();
	try_convert::<u8, i8, _>(&[u8::MAX as i8; 3]).unwrap_err();
	try_convert::<u8, i16, _>(&[u8::MAX as i16; 3]).unwrap();
	try_convert::<u8, i32, _>(&[u8::MAX as i32; 3]).unwrap();
	try_convert::<u8, i64, _>(&[u8::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_u8_u16_max_test() {
	try_convert::<u8, u8, _>(&[u16::MAX as u8; 3]).unwrap();
	try_convert::<u8, u16, _>(&[u16::MAX as u16; 3]).unwrap_err();
	try_convert::<u8, u32, _>(&[u16::MAX as u32; 3]).unwrap_err();
	try_convert::<u8, u64, _>(&[u16::MAX as u64; 3]).unwrap_err();
	try_convert::<u8, i8, _>(&[u16::MAX as i8; 3]).unwrap_err();
	try_convert::<u8, i16, _>(&[u16::MAX as i16; 3]).unwrap_err();
	try_convert::<u8, i32, _>(&[u16::MAX as i32; 3]).unwrap_err();
	try_convert::<u8, i64, _>(&[u16::MAX as i64; 3]).unwrap_err();
}

#[test]
fn try_convert_u8_u32_max_test() {
	try_convert::<u8, u8, _>(&[u32::MAX as u8; 3]).unwrap();
	try_convert::<u8, u16, _>(&[u32::MAX as u16; 3]).unwrap_err();
	try_convert::<u8, u32, _>(&[u32::MAX as u32; 3]).unwrap_err();
	try_convert::<u8, u64, _>(&[u32::MAX as u64; 3]).unwrap_err();
	try_convert::<u8, i8, _>(&[u32::MAX as i8; 3]).unwrap_err();
	try_convert::<u8, i16, _>(&[u32::MAX as i16; 3]).unwrap_err();
	try_convert::<u8, i32, _>(&[u32::MAX as i32; 3]).unwrap_err();
	try_convert::<u8, i64, _>(&[u32::MAX as i64; 3]).unwrap_err();
}

#[test]
fn try_convert_u8_u64_max_test() {
	try_convert::<u8, u8, _>(&[u64::MAX as u8; 3]).unwrap();
	try_convert::<u8, u16, _>(&[u64::MAX as u16; 3]).unwrap_err();
	try_convert::<u8, u32, _>(&[u64::MAX as u32; 3]).unwrap_err();
	try_convert::<u8, u64, _>(&[u64::MAX as u64; 3]).unwrap_err();
	try_convert::<u8, i8, _>(&[u64::MAX as i8; 3]).unwrap_err();
	try_convert::<u8, i16, _>(&[u64::MAX as i16; 3]).unwrap_err();
	try_convert::<u8, i32, _>(&[u64::MAX as i32; 3]).unwrap_err();
	try_convert::<u8, i64, _>(&[u64::MAX as i64; 3]).unwrap_err();
}

#[test]
fn try_convert_u8_i8_max_test() {
	try_convert::<u8, u8, _>(&[i8::MAX as u8; 3]).unwrap();
	try_convert::<u8, u16, _>(&[i8::MAX as u16; 3]).unwrap();
	try_convert::<u8, u32, _>(&[i8::MAX as u32; 3]).unwrap();
	try_convert::<u8, u64, _>(&[i8::MAX as u64; 3]).unwrap();
	try_convert::<u8, i8, _>(&[i8::MAX as i8; 3]).unwrap();
	try_convert::<u8, i16, _>(&[i8::MAX as i16; 3]).unwrap();
	try_convert::<u8, i32, _>(&[i8::MAX as i32; 3]).unwrap();
	try_convert::<u8, i64, _>(&[i8::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_u8_i16_max_test() {
	try_convert::<u8, u8, _>(&[i16::MAX as u8; 3]).unwrap();
	try_convert::<u8, u16, _>(&[i16::MAX as u16; 3]).unwrap_err();
	try_convert::<u8, u32, _>(&[i16::MAX as u32; 3]).unwrap_err();
	try_convert::<u8, u64, _>(&[i16::MAX as u64; 3]).unwrap_err();
	try_convert::<u8, i8, _>(&[i16::MAX as i8; 3]).unwrap_err();
	try_convert::<u8, i16, _>(&[i16::MAX as i16; 3]).unwrap_err();
	try_convert::<u8, i32, _>(&[i16::MAX as i32; 3]).unwrap_err();
	try_convert::<u8, i64, _>(&[i16::MAX as i64; 3]).unwrap_err();
}

#[test]
fn try_convert_u8_i32_max_test() {
	try_convert::<u8, u8, _>(&[i32::MAX as u8; 3]).unwrap();
	try_convert::<u8, u16, _>(&[i32::MAX as u16; 3]).unwrap_err();
	try_convert::<u8, u32, _>(&[i32::MAX as u32; 3]).unwrap_err();
	try_convert::<u8, u64, _>(&[i32::MAX as u64; 3]).unwrap_err();
	try_convert::<u8, i8, _>(&[i32::MAX as i8; 3]).unwrap_err();
	try_convert::<u8, i16, _>(&[i32::MAX as i16; 3]).unwrap_err();
	try_convert::<u8, i32, _>(&[i32::MAX as i32; 3]).unwrap_err();
	try_convert::<u8, i64, _>(&[i32::MAX as i64; 3]).unwrap_err();
}

#[test]
fn try_convert_u8_i64_max_test() {
	try_convert::<u8, u8, _>(&[i64::MAX as u8; 3]).unwrap();
	try_convert::<u8, u16, _>(&[i64::MAX as u16; 3]).unwrap_err();
	try_convert::<u8, u32, _>(&[i64::MAX as u32; 3]).unwrap_err();
	try_convert::<u8, u64, _>(&[i64::MAX as u64; 3]).unwrap_err();
	try_convert::<u8, i8, _>(&[i64::MAX as i8; 3]).unwrap_err();
	try_convert::<u8, i16, _>(&[i64::MAX as i16; 3]).unwrap_err();
	try_convert::<u8, i32, _>(&[i64::MAX as i32; 3]).unwrap_err();
	try_convert::<u8, i64, _>(&[i64::MAX as i64; 3]).unwrap_err();
}

#[test]
fn try_convert_u8_char_max_test() {
	try_convert::<u8, u8, _>(&[char::MAX as u8; 3]).unwrap();
	try_convert::<u8, u16, _>(&[char::MAX as u16; 3]).unwrap_err();
	try_convert::<u8, u32, _>(&[char::MAX as u32; 3]).unwrap_err();
	try_convert::<u8, u64, _>(&[char::MAX as u64; 3]).unwrap_err();
	try_convert::<u8, i8, _>(&[char::MAX as i8; 3]).unwrap_err();
	try_convert::<u8, i16, _>(&[char::MAX as i16; 3]).unwrap_err();
	try_convert::<u8, i32, _>(&[char::MAX as i32; 3]).unwrap_err();
	try_convert::<u8, i64, _>(&[char::MAX as i64; 3]).unwrap_err();
}

#[test]
fn try_convert_u16_u8_max_test() {
	try_convert::<u16, u8, _>(&[u8::MAX as u8; 3]).unwrap();
	try_convert::<u16, u16, _>(&[u8::MAX as u16; 3]).unwrap();
	try_convert::<u16, u32, _>(&[u8::MAX as u32; 3]).unwrap();
	try_convert::<u16, u64, _>(&[u8::MAX as u64; 3]).unwrap();
	try_convert::<u16, i8, _>(&[u8::MAX as i8; 3]).unwrap_err();
	try_convert::<u16, i16, _>(&[u8::MAX as i16; 3]).unwrap();
	try_convert::<u16, i32, _>(&[u8::MAX as i32; 3]).unwrap();
	try_convert::<u16, i64, _>(&[u8::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_u16_u16_max_test() {
	try_convert::<u16, u8, _>(&[u16::MAX as u8; 3]).unwrap();
	try_convert::<u16, u16, _>(&[u16::MAX as u16; 3]).unwrap();
	try_convert::<u16, u32, _>(&[u16::MAX as u32; 3]).unwrap();
	try_convert::<u16, u64, _>(&[u16::MAX as u64; 3]).unwrap();
	try_convert::<u16, i8, _>(&[u16::MAX as i8; 3]).unwrap_err();
	try_convert::<u16, i16, _>(&[u16::MAX as i16; 3]).unwrap_err();
	try_convert::<u16, i32, _>(&[u16::MAX as i32; 3]).unwrap();
	try_convert::<u16, i64, _>(&[u16::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_u16_u32_max_test() {
	try_convert::<u16, u8, _>(&[u32::MAX as u8; 3]).unwrap();
	try_convert::<u16, u16, _>(&[u32::MAX as u16; 3]).unwrap();
	try_convert::<u16, u32, _>(&[u32::MAX as u32; 3]).unwrap_err();
	try_convert::<u16, u64, _>(&[u32::MAX as u64; 3]).unwrap_err();
	try_convert::<u16, i8, _>(&[u32::MAX as i8; 3]).unwrap_err();
	try_convert::<u16, i16, _>(&[u32::MAX as i16; 3]).unwrap_err();
	try_convert::<u16, i32, _>(&[u32::MAX as i32; 3]).unwrap_err();
	try_convert::<u16, i64, _>(&[u32::MAX as i64; 3]).unwrap_err();
}

#[test]
fn try_convert_u16_u64_max_test() {
	try_convert::<u16, u8, _>(&[u64::MAX as u8; 3]).unwrap();
	try_convert::<u16, u16, _>(&[u64::MAX as u16; 3]).unwrap();
	try_convert::<u16, u32, _>(&[u64::MAX as u32; 3]).unwrap_err();
	try_convert::<u16, u64, _>(&[u64::MAX as u64; 3]).unwrap_err();
	try_convert::<u16, i8, _>(&[u64::MAX as i8; 3]).unwrap_err();
	try_convert::<u16, i16, _>(&[u64::MAX as i16; 3]).unwrap_err();
	try_convert::<u16, i32, _>(&[u64::MAX as i32; 3]).unwrap_err();
	try_convert::<u16, i64, _>(&[u64::MAX as i64; 3]).unwrap_err();
}

#[test]
fn try_convert_u16_i8_max_test() {
	try_convert::<u16, u8, _>(&[i8::MAX as u8; 3]).unwrap();
	try_convert::<u16, u16, _>(&[i8::MAX as u16; 3]).unwrap();
	try_convert::<u16, u32, _>(&[i8::MAX as u32; 3]).unwrap();
	try_convert::<u16, u64, _>(&[i8::MAX as u64; 3]).unwrap();
	try_convert::<u16, i8, _>(&[i8::MAX as i8; 3]).unwrap();
	try_convert::<u16, i16, _>(&[i8::MAX as i16; 3]).unwrap();
	try_convert::<u16, i32, _>(&[i8::MAX as i32; 3]).unwrap();
	try_convert::<u16, i64, _>(&[i8::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_u16_i16_max_test() {
	try_convert::<u16, u8, _>(&[i16::MAX as u8; 3]).unwrap();
	try_convert::<u16, u16, _>(&[i16::MAX as u16; 3]).unwrap();
	try_convert::<u16, u32, _>(&[i16::MAX as u32; 3]).unwrap();
	try_convert::<u16, u64, _>(&[i16::MAX as u64; 3]).unwrap();
	try_convert::<u16, i8, _>(&[i16::MAX as i8; 3]).unwrap_err();
	try_convert::<u16, i16, _>(&[i16::MAX as i16; 3]).unwrap();
	try_convert::<u16, i32, _>(&[i16::MAX as i32; 3]).unwrap();
	try_convert::<u16, i64, _>(&[i16::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_u16_i32_max_test() {
	try_convert::<u16, u8, _>(&[i32::MAX as u8; 3]).unwrap();
	try_convert::<u16, u16, _>(&[i32::MAX as u16; 3]).unwrap();
	try_convert::<u16, u32, _>(&[i32::MAX as u32; 3]).unwrap_err();
	try_convert::<u16, u64, _>(&[i32::MAX as u64; 3]).unwrap_err();
	try_convert::<u16, i8, _>(&[i32::MAX as i8; 3]).unwrap_err();
	try_convert::<u16, i16, _>(&[i32::MAX as i16; 3]).unwrap_err();
	try_convert::<u16, i32, _>(&[i32::MAX as i32; 3]).unwrap_err();
	try_convert::<u16, i64, _>(&[i32::MAX as i64; 3]).unwrap_err();
}

#[test]
fn try_convert_u16_i64_max_test() {
	try_convert::<u16, u8, _>(&[i64::MAX as u8; 3]).unwrap();
	try_convert::<u16, u16, _>(&[i64::MAX as u16; 3]).unwrap();
	try_convert::<u16, u32, _>(&[i64::MAX as u32; 3]).unwrap_err();
	try_convert::<u16, u64, _>(&[i64::MAX as u64; 3]).unwrap_err();
	try_convert::<u16, i8, _>(&[i64::MAX as i8; 3]).unwrap_err();
	try_convert::<u16, i16, _>(&[i64::MAX as i16; 3]).unwrap_err();
	try_convert::<u16, i32, _>(&[i64::MAX as i32; 3]).unwrap_err();
	try_convert::<u16, i64, _>(&[i64::MAX as i64; 3]).unwrap_err();
}

#[test]
fn try_convert_u16_char_max_test() {
	try_convert::<u16, u8, _>(&[char::MAX as u8; 3]).unwrap();
	try_convert::<u16, u16, _>(&[char::MAX as u16; 3]).unwrap();
	try_convert::<u16, u32, _>(&[char::MAX as u32; 3]).unwrap_err();
	try_convert::<u16, u64, _>(&[char::MAX as u64; 3]).unwrap_err();
	try_convert::<u16, i8, _>(&[char::MAX as i8; 3]).unwrap_err();
	try_convert::<u16, i16, _>(&[char::MAX as i16; 3]).unwrap_err();
	try_convert::<u16, i32, _>(&[char::MAX as i32; 3]).unwrap_err();
	try_convert::<u16, i64, _>(&[char::MAX as i64; 3]).unwrap_err();
}

#[test]
fn try_convert_u32_u8_max_test() {
	try_convert::<u32, u8, _>(&[u8::MAX as u8; 3]).unwrap();
	try_convert::<u32, u16, _>(&[u8::MAX as u16; 3]).unwrap();
	try_convert::<u32, u32, _>(&[u8::MAX as u32; 3]).unwrap();
	try_convert::<u32, u64, _>(&[u8::MAX as u64; 3]).unwrap();
	try_convert::<u32, i8, _>(&[u8::MAX as i8; 3]).unwrap_err();
	try_convert::<u32, i16, _>(&[u8::MAX as i16; 3]).unwrap();
	try_convert::<u32, i32, _>(&[u8::MAX as i32; 3]).unwrap();
	try_convert::<u32, i64, _>(&[u8::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_u32_u16_max_test() {
	try_convert::<u32, u8, _>(&[u16::MAX as u8; 3]).unwrap();
	try_convert::<u32, u16, _>(&[u16::MAX as u16; 3]).unwrap();
	try_convert::<u32, u32, _>(&[u16::MAX as u32; 3]).unwrap();
	try_convert::<u32, u64, _>(&[u16::MAX as u64; 3]).unwrap();
	try_convert::<u32, i8, _>(&[u16::MAX as i8; 3]).unwrap_err();
	try_convert::<u32, i16, _>(&[u16::MAX as i16; 3]).unwrap_err();
	try_convert::<u32, i32, _>(&[u16::MAX as i32; 3]).unwrap();
	try_convert::<u32, i64, _>(&[u16::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_u32_u32_max_test() {
	try_convert::<u32, u8, _>(&[u32::MAX as u8; 3]).unwrap();
	try_convert::<u32, u16, _>(&[u32::MAX as u16; 3]).unwrap();
	try_convert::<u32, u32, _>(&[u32::MAX as u32; 3]).unwrap();
	try_convert::<u32, u64, _>(&[u32::MAX as u64; 3]).unwrap();
	try_convert::<u32, i8, _>(&[u32::MAX as i8; 3]).unwrap_err();
	try_convert::<u32, i16, _>(&[u32::MAX as i16; 3]).unwrap_err();
	try_convert::<u32, i32, _>(&[u32::MAX as i32; 3]).unwrap_err();
	try_convert::<u32, i64, _>(&[u32::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_u32_u64_max_test() {
	try_convert::<u32, u8, _>(&[u64::MAX as u8; 3]).unwrap();
	try_convert::<u32, u16, _>(&[u64::MAX as u16; 3]).unwrap();
	try_convert::<u32, u32, _>(&[u64::MAX as u32; 3]).unwrap();
	try_convert::<u32, u64, _>(&[u64::MAX as u64; 3]).unwrap_err();
	try_convert::<u32, i8, _>(&[u64::MAX as i8; 3]).unwrap_err();
	try_convert::<u32, i16, _>(&[u64::MAX as i16; 3]).unwrap_err();
	try_convert::<u32, i32, _>(&[u64::MAX as i32; 3]).unwrap_err();
	try_convert::<u32, i64, _>(&[u64::MAX as i64; 3]).unwrap_err();
}

#[test]
fn try_convert_u32_i8_max_test() {
	try_convert::<u32, u8, _>(&[i8::MAX as u8; 3]).unwrap();
	try_convert::<u32, u16, _>(&[i8::MAX as u16; 3]).unwrap();
	try_convert::<u32, u32, _>(&[i8::MAX as u32; 3]).unwrap();
	try_convert::<u32, u64, _>(&[i8::MAX as u64; 3]).unwrap();
	try_convert::<u32, i8, _>(&[i8::MAX as i8; 3]).unwrap();
	try_convert::<u32, i16, _>(&[i8::MAX as i16; 3]).unwrap();
	try_convert::<u32, i32, _>(&[i8::MAX as i32; 3]).unwrap();
	try_convert::<u32, i64, _>(&[i8::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_u32_i16_max_test() {
	try_convert::<u32, u8, _>(&[i16::MAX as u8; 3]).unwrap();
	try_convert::<u32, u16, _>(&[i16::MAX as u16; 3]).unwrap();
	try_convert::<u32, u32, _>(&[i16::MAX as u32; 3]).unwrap();
	try_convert::<u32, u64, _>(&[i16::MAX as u64; 3]).unwrap();
	try_convert::<u32, i8, _>(&[i16::MAX as i8; 3]).unwrap_err();
	try_convert::<u32, i16, _>(&[i16::MAX as i16; 3]).unwrap();
	try_convert::<u32, i32, _>(&[i16::MAX as i32; 3]).unwrap();
	try_convert::<u32, i64, _>(&[i16::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_u32_i32_max_test() {
	try_convert::<u32, u8, _>(&[i32::MAX as u8; 3]).unwrap();
	try_convert::<u32, u16, _>(&[i32::MAX as u16; 3]).unwrap();
	try_convert::<u32, u32, _>(&[i32::MAX as u32; 3]).unwrap();
	try_convert::<u32, u64, _>(&[i32::MAX as u64; 3]).unwrap();
	try_convert::<u32, i8, _>(&[i32::MAX as i8; 3]).unwrap_err();
	try_convert::<u32, i16, _>(&[i32::MAX as i16; 3]).unwrap_err();
	try_convert::<u32, i32, _>(&[i32::MAX as i32; 3]).unwrap();
	try_convert::<u32, i64, _>(&[i32::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_u32_i64_max_test() {
	try_convert::<u32, u8, _>(&[i64::MAX as u8; 3]).unwrap();
	try_convert::<u32, u16, _>(&[i64::MAX as u16; 3]).unwrap();
	try_convert::<u32, u32, _>(&[i64::MAX as u32; 3]).unwrap();
	try_convert::<u32, u64, _>(&[i64::MAX as u64; 3]).unwrap_err();
	try_convert::<u32, i8, _>(&[i64::MAX as i8; 3]).unwrap_err();
	try_convert::<u32, i16, _>(&[i64::MAX as i16; 3]).unwrap_err();
	try_convert::<u32, i32, _>(&[i64::MAX as i32; 3]).unwrap_err();
	try_convert::<u32, i64, _>(&[i64::MAX as i64; 3]).unwrap_err();
}

#[test]
fn try_convert_u32_char_max_test() {
	try_convert::<u32, u8, _>(&[char::MAX as u8; 3]).unwrap();
	try_convert::<u32, u16, _>(&[char::MAX as u16; 3]).unwrap();
	try_convert::<u32, u32, _>(&[char::MAX as u32; 3]).unwrap();
	try_convert::<u32, u64, _>(&[char::MAX as u64; 3]).unwrap();
	try_convert::<u32, i8, _>(&[char::MAX as i8; 3]).unwrap_err();
	try_convert::<u32, i16, _>(&[char::MAX as i16; 3]).unwrap_err();
	try_convert::<u32, i32, _>(&[char::MAX as i32; 3]).unwrap();
	try_convert::<u32, i64, _>(&[char::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_u64_u8_max_test() {
	try_convert::<u64, u8, _>(&[u8::MAX as u8; 3]).unwrap();
	try_convert::<u64, u16, _>(&[u8::MAX as u16; 3]).unwrap();
	try_convert::<u64, u32, _>(&[u8::MAX as u32; 3]).unwrap();
	try_convert::<u64, u64, _>(&[u8::MAX as u64; 3]).unwrap();
	try_convert::<u64, i8, _>(&[u8::MAX as i8; 3]).unwrap_err();
	try_convert::<u64, i16, _>(&[u8::MAX as i16; 3]).unwrap();
	try_convert::<u64, i32, _>(&[u8::MAX as i32; 3]).unwrap();
	try_convert::<u64, i64, _>(&[u8::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_u64_u16_max_test() {
	try_convert::<u64, u8, _>(&[u16::MAX as u8; 3]).unwrap();
	try_convert::<u64, u16, _>(&[u16::MAX as u16; 3]).unwrap();
	try_convert::<u64, u32, _>(&[u16::MAX as u32; 3]).unwrap();
	try_convert::<u64, u64, _>(&[u16::MAX as u64; 3]).unwrap();
	try_convert::<u64, i8, _>(&[u16::MAX as i8; 3]).unwrap_err();
	try_convert::<u64, i16, _>(&[u16::MAX as i16; 3]).unwrap_err();
	try_convert::<u64, i32, _>(&[u16::MAX as i32; 3]).unwrap();
	try_convert::<u64, i64, _>(&[u16::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_u64_u32_max_test() {
	try_convert::<u64, u8, _>(&[u32::MAX as u8; 3]).unwrap();
	try_convert::<u64, u16, _>(&[u32::MAX as u16; 3]).unwrap();
	try_convert::<u64, u32, _>(&[u32::MAX as u32; 3]).unwrap();
	try_convert::<u64, u64, _>(&[u32::MAX as u64; 3]).unwrap();
	try_convert::<u64, i8, _>(&[u32::MAX as i8; 3]).unwrap_err();
	try_convert::<u64, i16, _>(&[u32::MAX as i16; 3]).unwrap_err();
	try_convert::<u64, i32, _>(&[u32::MAX as i32; 3]).unwrap_err();
	try_convert::<u64, i64, _>(&[u32::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_u64_u64_max_test() {
	try_convert::<u64, u8, _>(&[u64::MAX as u8; 3]).unwrap();
	try_convert::<u64, u16, _>(&[u64::MAX as u16; 3]).unwrap();
	try_convert::<u64, u32, _>(&[u64::MAX as u32; 3]).unwrap();
	try_convert::<u64, u64, _>(&[u64::MAX as u64; 3]).unwrap();
	try_convert::<u64, i8, _>(&[u64::MAX as i8; 3]).unwrap_err();
	try_convert::<u64, i16, _>(&[u64::MAX as i16; 3]).unwrap_err();
	try_convert::<u64, i32, _>(&[u64::MAX as i32; 3]).unwrap_err();
	try_convert::<u64, i64, _>(&[u64::MAX as i64; 3]).unwrap_err();
}

#[test]
fn try_convert_u64_i8_max_test() {
	try_convert::<u64, u8, _>(&[i8::MAX as u8; 3]).unwrap();
	try_convert::<u64, u16, _>(&[i8::MAX as u16; 3]).unwrap();
	try_convert::<u64, u32, _>(&[i8::MAX as u32; 3]).unwrap();
	try_convert::<u64, u64, _>(&[i8::MAX as u64; 3]).unwrap();
	try_convert::<u64, i8, _>(&[i8::MAX as i8; 3]).unwrap();
	try_convert::<u64, i16, _>(&[i8::MAX as i16; 3]).unwrap();
	try_convert::<u64, i32, _>(&[i8::MAX as i32; 3]).unwrap();
	try_convert::<u64, i64, _>(&[i8::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_u64_i16_max_test() {
	try_convert::<u64, u8, _>(&[i16::MAX as u8; 3]).unwrap();
	try_convert::<u64, u16, _>(&[i16::MAX as u16; 3]).unwrap();
	try_convert::<u64, u32, _>(&[i16::MAX as u32; 3]).unwrap();
	try_convert::<u64, u64, _>(&[i16::MAX as u64; 3]).unwrap();
	try_convert::<u64, i8, _>(&[i16::MAX as i8; 3]).unwrap_err();
	try_convert::<u64, i16, _>(&[i16::MAX as i16; 3]).unwrap();
	try_convert::<u64, i32, _>(&[i16::MAX as i32; 3]).unwrap();
	try_convert::<u64, i64, _>(&[i16::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_u64_i32_max_test() {
	try_convert::<u64, u8, _>(&[i32::MAX as u8; 3]).unwrap();
	try_convert::<u64, u16, _>(&[i32::MAX as u16; 3]).unwrap();
	try_convert::<u64, u32, _>(&[i32::MAX as u32; 3]).unwrap();
	try_convert::<u64, u64, _>(&[i32::MAX as u64; 3]).unwrap();
	try_convert::<u64, i8, _>(&[i32::MAX as i8; 3]).unwrap_err();
	try_convert::<u64, i16, _>(&[i32::MAX as i16; 3]).unwrap_err();
	try_convert::<u64, i32, _>(&[i32::MAX as i32; 3]).unwrap();
	try_convert::<u64, i64, _>(&[i32::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_u64_i64_max_test() {
	try_convert::<u64, u8, _>(&[i64::MAX as u8; 3]).unwrap();
	try_convert::<u64, u16, _>(&[i64::MAX as u16; 3]).unwrap();
	try_convert::<u64, u32, _>(&[i64::MAX as u32; 3]).unwrap();
	try_convert::<u64, u64, _>(&[i64::MAX as u64; 3]).unwrap();
	try_convert::<u64, i8, _>(&[i64::MAX as i8; 3]).unwrap_err();
	try_convert::<u64, i16, _>(&[i64::MAX as i16; 3]).unwrap_err();
	try_convert::<u64, i32, _>(&[i64::MAX as i32; 3]).unwrap_err();
	try_convert::<u64, i64, _>(&[i64::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_u64_char_max_test() {
	try_convert::<u64, u8, _>(&[char::MAX as u8; 3]).unwrap();
	try_convert::<u64, u16, _>(&[char::MAX as u16; 3]).unwrap();
	try_convert::<u64, u32, _>(&[char::MAX as u32; 3]).unwrap();
	try_convert::<u64, u64, _>(&[char::MAX as u64; 3]).unwrap();
	try_convert::<u64, i8, _>(&[char::MAX as i8; 3]).unwrap_err();
	try_convert::<u64, i16, _>(&[char::MAX as i16; 3]).unwrap_err();
	try_convert::<u64, i32, _>(&[char::MAX as i32; 3]).unwrap();
	try_convert::<u64, i64, _>(&[char::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_i8_u8_max_test() {
	try_convert::<i8, u8, _>(&[u8::MAX as u8; 3]).unwrap_err();
	try_convert::<i8, u16, _>(&[u8::MAX as u16; 3]).unwrap_err();
	try_convert::<i8, u32, _>(&[u8::MAX as u32; 3]).unwrap_err();
	try_convert::<i8, u64, _>(&[u8::MAX as u64; 3]).unwrap_err();
	try_convert::<i8, i8, _>(&[u8::MAX as i8; 3]).unwrap();
	try_convert::<i8, i16, _>(&[u8::MAX as i16; 3]).unwrap_err();
	try_convert::<i8, i32, _>(&[u8::MAX as i32; 3]).unwrap_err();
	try_convert::<i8, i64, _>(&[u8::MAX as i64; 3]).unwrap_err();
}

#[test]
fn try_convert_i8_u16_max_test() {
	try_convert::<i8, u8, _>(&[u16::MAX as u8; 3]).unwrap_err();
	try_convert::<i8, u16, _>(&[u16::MAX as u16; 3]).unwrap_err();
	try_convert::<i8, u32, _>(&[u16::MAX as u32; 3]).unwrap_err();
	try_convert::<i8, u64, _>(&[u16::MAX as u64; 3]).unwrap_err();
	try_convert::<i8, i8, _>(&[u16::MAX as i8; 3]).unwrap();
	try_convert::<i8, i16, _>(&[u16::MAX as i16; 3]).unwrap();
	try_convert::<i8, i32, _>(&[u16::MAX as i32; 3]).unwrap_err();
	try_convert::<i8, i64, _>(&[u16::MAX as i64; 3]).unwrap_err();
}

#[test]
fn try_convert_i8_u32_max_test() {
	try_convert::<i8, u8, _>(&[u32::MAX as u8; 3]).unwrap_err();
	try_convert::<i8, u16, _>(&[u32::MAX as u16; 3]).unwrap_err();
	try_convert::<i8, u32, _>(&[u32::MAX as u32; 3]).unwrap_err();
	try_convert::<i8, u64, _>(&[u32::MAX as u64; 3]).unwrap_err();
	try_convert::<i8, i8, _>(&[u32::MAX as i8; 3]).unwrap();
	try_convert::<i8, i16, _>(&[u32::MAX as i16; 3]).unwrap();
	try_convert::<i8, i32, _>(&[u32::MAX as i32; 3]).unwrap();
	try_convert::<i8, i64, _>(&[u32::MAX as i64; 3]).unwrap_err();
}

#[test]
fn try_convert_i8_u64_max_test() {
	try_convert::<i8, u8, _>(&[u64::MAX as u8; 3]).unwrap_err();
	try_convert::<i8, u16, _>(&[u64::MAX as u16; 3]).unwrap_err();
	try_convert::<i8, u32, _>(&[u64::MAX as u32; 3]).unwrap_err();
	try_convert::<i8, u64, _>(&[u64::MAX as u64; 3]).unwrap_err();
	try_convert::<i8, i8, _>(&[u64::MAX as i8; 3]).unwrap();
	try_convert::<i8, i16, _>(&[u64::MAX as i16; 3]).unwrap();
	try_convert::<i8, i32, _>(&[u64::MAX as i32; 3]).unwrap();
	try_convert::<i8, i64, _>(&[u64::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_i8_i8_max_test() {
	try_convert::<i8, u8, _>(&[i8::MAX as u8; 3]).unwrap();
	try_convert::<i8, u16, _>(&[i8::MAX as u16; 3]).unwrap();
	try_convert::<i8, u32, _>(&[i8::MAX as u32; 3]).unwrap();
	try_convert::<i8, u64, _>(&[i8::MAX as u64; 3]).unwrap();
	try_convert::<i8, i8, _>(&[i8::MAX as i8; 3]).unwrap();
	try_convert::<i8, i16, _>(&[i8::MAX as i16; 3]).unwrap();
	try_convert::<i8, i32, _>(&[i8::MAX as i32; 3]).unwrap();
	try_convert::<i8, i64, _>(&[i8::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_i8_i16_max_test() {
	try_convert::<i8, u8, _>(&[i16::MAX as u8; 3]).unwrap_err();
	try_convert::<i8, u16, _>(&[i16::MAX as u16; 3]).unwrap_err();
	try_convert::<i8, u32, _>(&[i16::MAX as u32; 3]).unwrap_err();
	try_convert::<i8, u64, _>(&[i16::MAX as u64; 3]).unwrap_err();
	try_convert::<i8, i8, _>(&[i16::MAX as i8; 3]).unwrap();
	try_convert::<i8, i16, _>(&[i16::MAX as i16; 3]).unwrap_err();
	try_convert::<i8, i32, _>(&[i16::MAX as i32; 3]).unwrap_err();
	try_convert::<i8, i64, _>(&[i16::MAX as i64; 3]).unwrap_err();
}

#[test]
fn try_convert_i8_i32_max_test() {
	try_convert::<i8, u8, _>(&[i32::MAX as u8; 3]).unwrap_err();
	try_convert::<i8, u16, _>(&[i32::MAX as u16; 3]).unwrap_err();
	try_convert::<i8, u32, _>(&[i32::MAX as u32; 3]).unwrap_err();
	try_convert::<i8, u64, _>(&[i32::MAX as u64; 3]).unwrap_err();
	try_convert::<i8, i8, _>(&[i32::MAX as i8; 3]).unwrap();
	try_convert::<i8, i16, _>(&[i32::MAX as i16; 3]).unwrap();
	try_convert::<i8, i32, _>(&[i32::MAX as i32; 3]).unwrap_err();
	try_convert::<i8, i64, _>(&[i32::MAX as i64; 3]).unwrap_err();
}

#[test]
fn try_convert_i8_i64_max_test() {
	try_convert::<i8, u8, _>(&[i64::MAX as u8; 3]).unwrap_err();
	try_convert::<i8, u16, _>(&[i64::MAX as u16; 3]).unwrap_err();
	try_convert::<i8, u32, _>(&[i64::MAX as u32; 3]).unwrap_err();
	try_convert::<i8, u64, _>(&[i64::MAX as u64; 3]).unwrap_err();
	try_convert::<i8, i8, _>(&[i64::MAX as i8; 3]).unwrap();
	try_convert::<i8, i16, _>(&[i64::MAX as i16; 3]).unwrap();
	try_convert::<i8, i32, _>(&[i64::MAX as i32; 3]).unwrap();
	try_convert::<i8, i64, _>(&[i64::MAX as i64; 3]).unwrap_err();
}

#[test]
fn try_convert_i8_char_max_test() {
	try_convert::<i8, u8, _>(&[char::MAX as u8; 3]).unwrap_err();
	try_convert::<i8, u16, _>(&[char::MAX as u16; 3]).unwrap_err();
	try_convert::<i8, u32, _>(&[char::MAX as u32; 3]).unwrap_err();
	try_convert::<i8, u64, _>(&[char::MAX as u64; 3]).unwrap_err();
	try_convert::<i8, i8, _>(&[char::MAX as i8; 3]).unwrap();
	try_convert::<i8, i16, _>(&[char::MAX as i16; 3]).unwrap();
	try_convert::<i8, i32, _>(&[char::MAX as i32; 3]).unwrap_err();
	try_convert::<i8, i64, _>(&[char::MAX as i64; 3]).unwrap_err();
}

#[test]
fn try_convert_i16_u8_max_test() {
	try_convert::<i16, u8, _>(&[u8::MAX as u8; 3]).unwrap();
	try_convert::<i16, u16, _>(&[u8::MAX as u16; 3]).unwrap();
	try_convert::<i16, u32, _>(&[u8::MAX as u32; 3]).unwrap();
	try_convert::<i16, u64, _>(&[u8::MAX as u64; 3]).unwrap();
	try_convert::<i16, i8, _>(&[u8::MAX as i8; 3]).unwrap();
	try_convert::<i16, i16, _>(&[u8::MAX as i16; 3]).unwrap();
	try_convert::<i16, i32, _>(&[u8::MAX as i32; 3]).unwrap();
	try_convert::<i16, i64, _>(&[u8::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_i16_u16_max_test() {
	try_convert::<i16, u8, _>(&[u16::MAX as u8; 3]).unwrap();
	try_convert::<i16, u16, _>(&[u16::MAX as u16; 3]).unwrap_err();
	try_convert::<i16, u32, _>(&[u16::MAX as u32; 3]).unwrap_err();
	try_convert::<i16, u64, _>(&[u16::MAX as u64; 3]).unwrap_err();
	try_convert::<i16, i8, _>(&[u16::MAX as i8; 3]).unwrap();
	try_convert::<i16, i16, _>(&[u16::MAX as i16; 3]).unwrap();
	try_convert::<i16, i32, _>(&[u16::MAX as i32; 3]).unwrap_err();
	try_convert::<i16, i64, _>(&[u16::MAX as i64; 3]).unwrap_err();
}

#[test]
fn try_convert_i16_u32_max_test() {
	try_convert::<i16, u8, _>(&[u32::MAX as u8; 3]).unwrap();
	try_convert::<i16, u16, _>(&[u32::MAX as u16; 3]).unwrap_err();
	try_convert::<i16, u32, _>(&[u32::MAX as u32; 3]).unwrap_err();
	try_convert::<i16, u64, _>(&[u32::MAX as u64; 3]).unwrap_err();
	try_convert::<i16, i8, _>(&[u32::MAX as i8; 3]).unwrap();
	try_convert::<i16, i16, _>(&[u32::MAX as i16; 3]).unwrap();
	try_convert::<i16, i32, _>(&[u32::MAX as i32; 3]).unwrap();
	try_convert::<i16, i64, _>(&[u32::MAX as i64; 3]).unwrap_err();
}

#[test]
fn try_convert_i16_u64_max_test() {
	try_convert::<i16, u8, _>(&[u64::MAX as u8; 3]).unwrap();
	try_convert::<i16, u16, _>(&[u64::MAX as u16; 3]).unwrap_err();
	try_convert::<i16, u32, _>(&[u64::MAX as u32; 3]).unwrap_err();
	try_convert::<i16, u64, _>(&[u64::MAX as u64; 3]).unwrap_err();
	try_convert::<i16, i8, _>(&[u64::MAX as i8; 3]).unwrap();
	try_convert::<i16, i16, _>(&[u64::MAX as i16; 3]).unwrap();
	try_convert::<i16, i32, _>(&[u64::MAX as i32; 3]).unwrap();
	try_convert::<i16, i64, _>(&[u64::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_i16_i8_max_test() {
	try_convert::<i16, u8, _>(&[i8::MAX as u8; 3]).unwrap();
	try_convert::<i16, u16, _>(&[i8::MAX as u16; 3]).unwrap();
	try_convert::<i16, u32, _>(&[i8::MAX as u32; 3]).unwrap();
	try_convert::<i16, u64, _>(&[i8::MAX as u64; 3]).unwrap();
	try_convert::<i16, i8, _>(&[i8::MAX as i8; 3]).unwrap();
	try_convert::<i16, i16, _>(&[i8::MAX as i16; 3]).unwrap();
	try_convert::<i16, i32, _>(&[i8::MAX as i32; 3]).unwrap();
	try_convert::<i16, i64, _>(&[i8::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_i16_i16_max_test() {
	try_convert::<i16, u8, _>(&[i16::MAX as u8; 3]).unwrap();
	try_convert::<i16, u16, _>(&[i16::MAX as u16; 3]).unwrap();
	try_convert::<i16, u32, _>(&[i16::MAX as u32; 3]).unwrap();
	try_convert::<i16, u64, _>(&[i16::MAX as u64; 3]).unwrap();
	try_convert::<i16, i8, _>(&[i16::MAX as i8; 3]).unwrap();
	try_convert::<i16, i16, _>(&[i16::MAX as i16; 3]).unwrap();
	try_convert::<i16, i32, _>(&[i16::MAX as i32; 3]).unwrap();
	try_convert::<i16, i64, _>(&[i16::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_i16_i32_max_test() {
	try_convert::<i16, u8, _>(&[i32::MAX as u8; 3]).unwrap();
	try_convert::<i16, u16, _>(&[i32::MAX as u16; 3]).unwrap_err();
	try_convert::<i16, u32, _>(&[i32::MAX as u32; 3]).unwrap_err();
	try_convert::<i16, u64, _>(&[i32::MAX as u64; 3]).unwrap_err();
	try_convert::<i16, i8, _>(&[i32::MAX as i8; 3]).unwrap();
	try_convert::<i16, i16, _>(&[i32::MAX as i16; 3]).unwrap();
	try_convert::<i16, i32, _>(&[i32::MAX as i32; 3]).unwrap_err();
	try_convert::<i16, i64, _>(&[i32::MAX as i64; 3]).unwrap_err();
}

#[test]
fn try_convert_i16_i64_max_test() {
	try_convert::<i16, u8, _>(&[i64::MAX as u8; 3]).unwrap();
	try_convert::<i16, u16, _>(&[i64::MAX as u16; 3]).unwrap_err();
	try_convert::<i16, u32, _>(&[i64::MAX as u32; 3]).unwrap_err();
	try_convert::<i16, u64, _>(&[i64::MAX as u64; 3]).unwrap_err();
	try_convert::<i16, i8, _>(&[i64::MAX as i8; 3]).unwrap();
	try_convert::<i16, i16, _>(&[i64::MAX as i16; 3]).unwrap();
	try_convert::<i16, i32, _>(&[i64::MAX as i32; 3]).unwrap();
	try_convert::<i16, i64, _>(&[i64::MAX as i64; 3]).unwrap_err();
}

#[test]
fn try_convert_i16_char_max_test() {
	try_convert::<i16, u8, _>(&[char::MAX as u8; 3]).unwrap();
	try_convert::<i16, u16, _>(&[char::MAX as u16; 3]).unwrap_err();
	try_convert::<i16, u32, _>(&[char::MAX as u32; 3]).unwrap_err();
	try_convert::<i16, u64, _>(&[char::MAX as u64; 3]).unwrap_err();
	try_convert::<i16, i8, _>(&[char::MAX as i8; 3]).unwrap();
	try_convert::<i16, i16, _>(&[char::MAX as i16; 3]).unwrap();
	try_convert::<i16, i32, _>(&[char::MAX as i32; 3]).unwrap_err();
	try_convert::<i16, i64, _>(&[char::MAX as i64; 3]).unwrap_err();
}

#[test]
fn try_convert_i32_u8_max_test() {
	try_convert::<i32, u8, _>(&[u8::MAX as u8; 3]).unwrap();
	try_convert::<i32, u16, _>(&[u8::MAX as u16; 3]).unwrap();
	try_convert::<i32, u32, _>(&[u8::MAX as u32; 3]).unwrap();
	try_convert::<i32, u64, _>(&[u8::MAX as u64; 3]).unwrap();
	try_convert::<i32, i8, _>(&[u8::MAX as i8; 3]).unwrap();
	try_convert::<i32, i16, _>(&[u8::MAX as i16; 3]).unwrap();
	try_convert::<i32, i32, _>(&[u8::MAX as i32; 3]).unwrap();
	try_convert::<i32, i64, _>(&[u8::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_i32_u16_max_test() {
	try_convert::<i32, u8, _>(&[u16::MAX as u8; 3]).unwrap();
	try_convert::<i32, u16, _>(&[u16::MAX as u16; 3]).unwrap();
	try_convert::<i32, u32, _>(&[u16::MAX as u32; 3]).unwrap();
	try_convert::<i32, u64, _>(&[u16::MAX as u64; 3]).unwrap();
	try_convert::<i32, i8, _>(&[u16::MAX as i8; 3]).unwrap();
	try_convert::<i32, i16, _>(&[u16::MAX as i16; 3]).unwrap();
	try_convert::<i32, i32, _>(&[u16::MAX as i32; 3]).unwrap();
	try_convert::<i32, i64, _>(&[u16::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_i32_u32_max_test() {
	try_convert::<i32, u8, _>(&[u32::MAX as u8; 3]).unwrap();
	try_convert::<i32, u16, _>(&[u32::MAX as u16; 3]).unwrap();
	try_convert::<i32, u32, _>(&[u32::MAX as u32; 3]).unwrap_err();
	try_convert::<i32, u64, _>(&[u32::MAX as u64; 3]).unwrap_err();
	try_convert::<i32, i8, _>(&[u32::MAX as i8; 3]).unwrap();
	try_convert::<i32, i16, _>(&[u32::MAX as i16; 3]).unwrap();
	try_convert::<i32, i32, _>(&[u32::MAX as i32; 3]).unwrap();
	try_convert::<i32, i64, _>(&[u32::MAX as i64; 3]).unwrap_err();
}

#[test]
fn try_convert_i32_u64_max_test() {
	try_convert::<i32, u8, _>(&[u64::MAX as u8; 3]).unwrap();
	try_convert::<i32, u16, _>(&[u64::MAX as u16; 3]).unwrap();
	try_convert::<i32, u32, _>(&[u64::MAX as u32; 3]).unwrap_err();
	try_convert::<i32, u64, _>(&[u64::MAX as u64; 3]).unwrap_err();
	try_convert::<i32, i8, _>(&[u64::MAX as i8; 3]).unwrap();
	try_convert::<i32, i16, _>(&[u64::MAX as i16; 3]).unwrap();
	try_convert::<i32, i32, _>(&[u64::MAX as i32; 3]).unwrap();
	try_convert::<i32, i64, _>(&[u64::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_i32_i8_max_test() {
	try_convert::<i32, u8, _>(&[i8::MAX as u8; 3]).unwrap();
	try_convert::<i32, u16, _>(&[i8::MAX as u16; 3]).unwrap();
	try_convert::<i32, u32, _>(&[i8::MAX as u32; 3]).unwrap();
	try_convert::<i32, u64, _>(&[i8::MAX as u64; 3]).unwrap();
	try_convert::<i32, i8, _>(&[i8::MAX as i8; 3]).unwrap();
	try_convert::<i32, i16, _>(&[i8::MAX as i16; 3]).unwrap();
	try_convert::<i32, i32, _>(&[i8::MAX as i32; 3]).unwrap();
	try_convert::<i32, i64, _>(&[i8::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_i32_i16_max_test() {
	try_convert::<i32, u8, _>(&[i16::MAX as u8; 3]).unwrap();
	try_convert::<i32, u16, _>(&[i16::MAX as u16; 3]).unwrap();
	try_convert::<i32, u32, _>(&[i16::MAX as u32; 3]).unwrap();
	try_convert::<i32, u64, _>(&[i16::MAX as u64; 3]).unwrap();
	try_convert::<i32, i8, _>(&[i16::MAX as i8; 3]).unwrap();
	try_convert::<i32, i16, _>(&[i16::MAX as i16; 3]).unwrap();
	try_convert::<i32, i32, _>(&[i16::MAX as i32; 3]).unwrap();
	try_convert::<i32, i64, _>(&[i16::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_i32_i32_max_test() {
	try_convert::<i32, u8, _>(&[i32::MAX as u8; 3]).unwrap();
	try_convert::<i32, u16, _>(&[i32::MAX as u16; 3]).unwrap();
	try_convert::<i32, u32, _>(&[i32::MAX as u32; 3]).unwrap();
	try_convert::<i32, u64, _>(&[i32::MAX as u64; 3]).unwrap();
	try_convert::<i32, i8, _>(&[i32::MAX as i8; 3]).unwrap();
	try_convert::<i32, i16, _>(&[i32::MAX as i16; 3]).unwrap();
	try_convert::<i32, i32, _>(&[i32::MAX as i32; 3]).unwrap();
	try_convert::<i32, i64, _>(&[i32::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_i32_i64_max_test() {
	try_convert::<i32, u8, _>(&[i64::MAX as u8; 3]).unwrap();
	try_convert::<i32, u16, _>(&[i64::MAX as u16; 3]).unwrap();
	try_convert::<i32, u32, _>(&[i64::MAX as u32; 3]).unwrap_err();
	try_convert::<i32, u64, _>(&[i64::MAX as u64; 3]).unwrap_err();
	try_convert::<i32, i8, _>(&[i64::MAX as i8; 3]).unwrap();
	try_convert::<i32, i16, _>(&[i64::MAX as i16; 3]).unwrap();
	try_convert::<i32, i32, _>(&[i64::MAX as i32; 3]).unwrap();
	try_convert::<i32, i64, _>(&[i64::MAX as i64; 3]).unwrap_err();
}

#[test]
fn try_convert_i32_char_max_test() {
	try_convert::<i32, u8, _>(&[char::MAX as u8; 3]).unwrap();
	try_convert::<i32, u16, _>(&[char::MAX as u16; 3]).unwrap();
	try_convert::<i32, u32, _>(&[char::MAX as u32; 3]).unwrap();
	try_convert::<i32, u64, _>(&[char::MAX as u64; 3]).unwrap();
	try_convert::<i32, i8, _>(&[char::MAX as i8; 3]).unwrap();
	try_convert::<i32, i16, _>(&[char::MAX as i16; 3]).unwrap();
	try_convert::<i32, i32, _>(&[char::MAX as i32; 3]).unwrap();
	try_convert::<i32, i64, _>(&[char::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_i64_u8_max_test() {
	try_convert::<i64, u8, _>(&[u8::MAX as u8; 3]).unwrap();
	try_convert::<i64, u16, _>(&[u8::MAX as u16; 3]).unwrap();
	try_convert::<i64, u32, _>(&[u8::MAX as u32; 3]).unwrap();
	try_convert::<i64, u64, _>(&[u8::MAX as u64; 3]).unwrap();
	try_convert::<i64, i8, _>(&[u8::MAX as i8; 3]).unwrap();
	try_convert::<i64, i16, _>(&[u8::MAX as i16; 3]).unwrap();
	try_convert::<i64, i32, _>(&[u8::MAX as i32; 3]).unwrap();
	try_convert::<i64, i64, _>(&[u8::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_i64_u16_max_test() {
	try_convert::<i64, u8, _>(&[u16::MAX as u8; 3]).unwrap();
	try_convert::<i64, u16, _>(&[u16::MAX as u16; 3]).unwrap();
	try_convert::<i64, u32, _>(&[u16::MAX as u32; 3]).unwrap();
	try_convert::<i64, u64, _>(&[u16::MAX as u64; 3]).unwrap();
	try_convert::<i64, i8, _>(&[u16::MAX as i8; 3]).unwrap();
	try_convert::<i64, i16, _>(&[u16::MAX as i16; 3]).unwrap();
	try_convert::<i64, i32, _>(&[u16::MAX as i32; 3]).unwrap();
	try_convert::<i64, i64, _>(&[u16::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_i64_u32_max_test() {
	try_convert::<i64, u8, _>(&[u32::MAX as u8; 3]).unwrap();
	try_convert::<i64, u16, _>(&[u32::MAX as u16; 3]).unwrap();
	try_convert::<i64, u32, _>(&[u32::MAX as u32; 3]).unwrap();
	try_convert::<i64, u64, _>(&[u32::MAX as u64; 3]).unwrap();
	try_convert::<i64, i8, _>(&[u32::MAX as i8; 3]).unwrap();
	try_convert::<i64, i16, _>(&[u32::MAX as i16; 3]).unwrap();
	try_convert::<i64, i32, _>(&[u32::MAX as i32; 3]).unwrap();
	try_convert::<i64, i64, _>(&[u32::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_i64_u64_max_test() {
	try_convert::<i64, u8, _>(&[u64::MAX as u8; 3]).unwrap();
	try_convert::<i64, u16, _>(&[u64::MAX as u16; 3]).unwrap();
	try_convert::<i64, u32, _>(&[u64::MAX as u32; 3]).unwrap();
	try_convert::<i64, u64, _>(&[u64::MAX as u64; 3]).unwrap_err();
	try_convert::<i64, i8, _>(&[u64::MAX as i8; 3]).unwrap();
	try_convert::<i64, i16, _>(&[u64::MAX as i16; 3]).unwrap();
	try_convert::<i64, i32, _>(&[u64::MAX as i32; 3]).unwrap();
	try_convert::<i64, i64, _>(&[u64::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_i64_i8_max_test() {
	try_convert::<i64, u8, _>(&[i8::MAX as u8; 3]).unwrap();
	try_convert::<i64, u16, _>(&[i8::MAX as u16; 3]).unwrap();
	try_convert::<i64, u32, _>(&[i8::MAX as u32; 3]).unwrap();
	try_convert::<i64, u64, _>(&[i8::MAX as u64; 3]).unwrap();
	try_convert::<i64, i8, _>(&[i8::MAX as i8; 3]).unwrap();
	try_convert::<i64, i16, _>(&[i8::MAX as i16; 3]).unwrap();
	try_convert::<i64, i32, _>(&[i8::MAX as i32; 3]).unwrap();
	try_convert::<i64, i64, _>(&[i8::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_i64_i16_max_test() {
	try_convert::<i64, u8, _>(&[i16::MAX as u8; 3]).unwrap();
	try_convert::<i64, u16, _>(&[i16::MAX as u16; 3]).unwrap();
	try_convert::<i64, u32, _>(&[i16::MAX as u32; 3]).unwrap();
	try_convert::<i64, u64, _>(&[i16::MAX as u64; 3]).unwrap();
	try_convert::<i64, i8, _>(&[i16::MAX as i8; 3]).unwrap();
	try_convert::<i64, i16, _>(&[i16::MAX as i16; 3]).unwrap();
	try_convert::<i64, i32, _>(&[i16::MAX as i32; 3]).unwrap();
	try_convert::<i64, i64, _>(&[i16::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_i64_i32_max_test() {
	try_convert::<i64, u8, _>(&[i32::MAX as u8; 3]).unwrap();
	try_convert::<i64, u16, _>(&[i32::MAX as u16; 3]).unwrap();
	try_convert::<i64, u32, _>(&[i32::MAX as u32; 3]).unwrap();
	try_convert::<i64, u64, _>(&[i32::MAX as u64; 3]).unwrap();
	try_convert::<i64, i8, _>(&[i32::MAX as i8; 3]).unwrap();
	try_convert::<i64, i16, _>(&[i32::MAX as i16; 3]).unwrap();
	try_convert::<i64, i32, _>(&[i32::MAX as i32; 3]).unwrap();
	try_convert::<i64, i64, _>(&[i32::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_i64_i64_max_test() {
	try_convert::<i64, u8, _>(&[i64::MAX as u8; 3]).unwrap();
	try_convert::<i64, u16, _>(&[i64::MAX as u16; 3]).unwrap();
	try_convert::<i64, u32, _>(&[i64::MAX as u32; 3]).unwrap();
	try_convert::<i64, u64, _>(&[i64::MAX as u64; 3]).unwrap();
	try_convert::<i64, i8, _>(&[i64::MAX as i8; 3]).unwrap();
	try_convert::<i64, i16, _>(&[i64::MAX as i16; 3]).unwrap();
	try_convert::<i64, i32, _>(&[i64::MAX as i32; 3]).unwrap();
	try_convert::<i64, i64, _>(&[i64::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_i64_char_max_test() {
	try_convert::<i64, u8, _>(&[char::MAX as u8; 3]).unwrap();
	try_convert::<i64, u16, _>(&[char::MAX as u16; 3]).unwrap();
	try_convert::<i64, u32, _>(&[char::MAX as u32; 3]).unwrap();
	try_convert::<i64, u64, _>(&[char::MAX as u64; 3]).unwrap();
	try_convert::<i64, i8, _>(&[char::MAX as i8; 3]).unwrap();
	try_convert::<i64, i16, _>(&[char::MAX as i16; 3]).unwrap();
	try_convert::<i64, i32, _>(&[char::MAX as i32; 3]).unwrap();
	try_convert::<i64, i64, _>(&[char::MAX as i64; 3]).unwrap();
}

#[test]
fn try_convert_u8_minus_test() {
	try_convert::<u8, i8, _>(&[-1; 3]).unwrap_err();
	try_convert::<u8, i16, _>(&[-1; 3]).unwrap_err();
	try_convert::<u8, i32, _>(&[-1; 3]).unwrap_err();
	try_convert::<u8, i64, _>(&[-1; 3]).unwrap_err();
	try_convert::<u8, i8, _>(&[i8::MIN; 3]).unwrap_err();
	try_convert::<u8, i16, _>(&[i16::MIN; 3]).unwrap_err();
	try_convert::<u8, i32, _>(&[i32::MIN; 3]).unwrap_err();
	try_convert::<u8, i64, _>(&[i64::MIN; 3]).unwrap_err();
}

#[test]
fn try_convert_u16_minus_test() {
	try_convert::<u16, i8, _>(&[-1; 3]).unwrap_err();
	try_convert::<u16, i16, _>(&[-1; 3]).unwrap_err();
	try_convert::<u16, i32, _>(&[-1; 3]).unwrap_err();
	try_convert::<u16, i64, _>(&[-1; 3]).unwrap_err();
	try_convert::<u16, i8, _>(&[i8::MIN; 3]).unwrap_err();
	try_convert::<u16, i16, _>(&[i16::MIN; 3]).unwrap_err();
	try_convert::<u16, i32, _>(&[i32::MIN; 3]).unwrap_err();
	try_convert::<u16, i64, _>(&[i64::MIN; 3]).unwrap_err();
}

#[test]
fn try_convert_u32_minus_test() {
	try_convert::<u32, i8, _>(&[-1; 3]).unwrap_err();
	try_convert::<u32, i16, _>(&[-1; 3]).unwrap_err();
	try_convert::<u32, i32, _>(&[-1; 3]).unwrap_err();
	try_convert::<u32, i64, _>(&[-1; 3]).unwrap_err();
	try_convert::<u32, i8, _>(&[i8::MIN; 3]).unwrap_err();
	try_convert::<u32, i16, _>(&[i16::MIN; 3]).unwrap_err();
	try_convert::<u32, i32, _>(&[i32::MIN; 3]).unwrap_err();
	try_convert::<u32, i64, _>(&[i64::MIN; 3]).unwrap_err();
}

#[test]
fn try_convert_u64_minus_test() {
	try_convert::<u64, i8, _>(&[-1; 3]).unwrap_err();
	try_convert::<u64, i16, _>(&[-1; 3]).unwrap_err();
	try_convert::<u64, i32, _>(&[-1; 3]).unwrap_err();
	try_convert::<u64, i64, _>(&[-1; 3]).unwrap_err();
	try_convert::<u64, i8, _>(&[i8::MIN; 3]).unwrap_err();
	try_convert::<u64, i16, _>(&[i16::MIN; 3]).unwrap_err();
	try_convert::<u64, i32, _>(&[i32::MIN; 3]).unwrap_err();
	try_convert::<u64, i64, _>(&[i64::MIN; 3]).unwrap_err();
}

#[test]
fn try_convert_i8_minus_test() {
	try_convert::<i8, i8, _>(&[-1; 3]).unwrap();
	try_convert::<i8, i16, _>(&[-1; 3]).unwrap();
	try_convert::<i8, i32, _>(&[-1; 3]).unwrap();
	try_convert::<i8, i64, _>(&[-1; 3]).unwrap();
	try_convert::<i8, i8, _>(&[i8::MIN; 3]).unwrap();
	try_convert::<i8, i16, _>(&[i16::MIN; 3]).unwrap_err();
	try_convert::<i8, i32, _>(&[i32::MIN; 3]).unwrap_err();
	try_convert::<i8, i64, _>(&[i64::MIN; 3]).unwrap_err();
}

#[test]
fn try_convert_i16_minus_test() {
	try_convert::<i16, i8, _>(&[-1; 3]).unwrap();
	try_convert::<i16, i16, _>(&[-1; 3]).unwrap();
	try_convert::<i16, i32, _>(&[-1; 3]).unwrap();
	try_convert::<i16, i64, _>(&[-1; 3]).unwrap();
	try_convert::<i16, i8, _>(&[i8::MIN; 3]).unwrap();
	try_convert::<i16, i16, _>(&[i16::MIN; 3]).unwrap();
	try_convert::<i16, i32, _>(&[i32::MIN; 3]).unwrap_err();
	try_convert::<i16, i64, _>(&[i64::MIN; 3]).unwrap_err();
}

#[test]
fn try_convert_i32_minus_test() {
	try_convert::<i32, i8, _>(&[-1; 3]).unwrap();
	try_convert::<i32, i16, _>(&[-1; 3]).unwrap();
	try_convert::<i32, i32, _>(&[-1; 3]).unwrap();
	try_convert::<i32, i64, _>(&[-1; 3]).unwrap();
	try_convert::<i32, i8, _>(&[i8::MIN; 3]).unwrap();
	try_convert::<i32, i16, _>(&[i16::MIN; 3]).unwrap();
	try_convert::<i32, i32, _>(&[i32::MIN; 3]).unwrap();
	try_convert::<i32, i64, _>(&[i64::MIN; 3]).unwrap_err();
}

#[test]
fn try_convert_i64_minus_test() {
	try_convert::<i64, i8, _>(&[-1; 3]).unwrap();
	try_convert::<i64, i16, _>(&[-1; 3]).unwrap();
	try_convert::<i64, i32, _>(&[-1; 3]).unwrap();
	try_convert::<i64, i64, _>(&[-1; 3]).unwrap();
	try_convert::<i64, i8, _>(&[i8::MIN; 3]).unwrap();
	try_convert::<i64, i16, _>(&[i16::MIN; 3]).unwrap();
	try_convert::<i64, i32, _>(&[i32::MIN; 3]).unwrap();
	try_convert::<i64, i64, _>(&[i64::MIN; 3]).unwrap();
}
