use vec_clock::*;

#[test]
fn clock_time_test() {
	let mut vc = new(1, 0).unwrap();
	assert!(vc.time() == [1]);
	assert!(vc.time() == [2]);
	assert!(vc.time() == [3]);

	let mut vc = new(3, 0).unwrap();
	assert!(vc.time() == [1, 0, 0]);
	assert!(vc.time() == [2, 0, 0]);
	assert!(vc.time() == [3, 0, 0]);

	let mut vc = new(3, 1).unwrap();
	assert!(vc.time() == [0, 1, 0]);
	assert!(vc.time() == [0, 2, 0]);
	assert!(vc.time() == [0, 3, 0]);

	let mut vc = new(3, 2).unwrap();
	assert!(vc.time() == [0, 0, 1]);
	assert!(vc.time() == [0, 0, 2]);
	assert!(vc.time() == [0, 0, 3]);
}

#[test]
#[should_panic(expected = "attempt to add with overflow")]
fn clock_time_overflow_test() {
	let mut vc = new_by(&[0u64, !0, 0], 1).unwrap();
	vc.time();
}

#[test]
fn clock_time_by_test() {
	let mut vc = new(3, 1).unwrap();
	assert!(vc.time_by(&[0u64; 3]).unwrap() == [0, 1, 0]);
	assert!(vc.time_by(&[1u64, 1, 3]).unwrap() == [1, 2, 3]);
	assert!(vc.time_by(&[!0u64, 0, !0 - 1]).unwrap() == [!0, 3, !0 - 1]);
	assert!(vc.time_by(&[char::from(3); 3]).unwrap() == [!0, 4, !0 - 1]);

	let e = vc.time_by(&[0u64; 2]).unwrap_err();
	assert_unmatch_time_size(&e);

	let e = vc.time_by(&[0u64; 4]).unwrap_err();
	assert_unmatch_time_size(&e);

	let e = vc.time_by(&[0u64, u64::MAX, 0]).unwrap_err();
	assert_invalid_time_value(&e);
}

#[test]
#[should_panic(expected = "attempt to add with overflow")]
fn clock_time_by_overflow_test() {
	let mut vc = new_by(&[0u64, !0, 0], 1).unwrap();
	let _ = vc.time_by(&[0u64; 3]);
}

#[test]
fn clock_time_by_try_from_unsigned_test() {
	let mut vc = new_as::<u8>(3, 1).unwrap();
	assert!(vc.time_by_try_from(&[0; 3]).unwrap() == [0, 1, 0]);
	assert!(vc.time_by_try_from(&[1, 1, 3]).unwrap() == [1, 2, 3]);
	assert!(vc.time_by_try_from(&[!0u8, 0, !0 - 1]).unwrap() == [!0, 3, !0 - 1]);
	assert!(vc.time_by_try_from(&[char::from(3); 3]).unwrap() == [!0, 4, !0 - 1]);

	let e = vc.time_by_try_from(&[0; 2]).unwrap_err();
	assert_unmatch_time_size(&e);

	let e = vc.time_by_try_from(&[0; 4]).unwrap_err();
	assert_unmatch_time_size(&e);

	let e = vc.time_by_try_from(&[0, u8::MAX, 0]).unwrap_err();
	assert_invalid_time_value(&e);

	let e = vc.time_by_try_from(&[-1, 0, 0]).unwrap_err();
	assert_try_from_int_error(&e);

	let e = vc.time_by_try_from(&[0, -1, 0]).unwrap_err();
	assert_try_from_int_error(&e);

	let e = vc.time_by_try_from(&[0, 0, -1]).unwrap_err();
	assert_try_from_int_error(&e);

	let e = vc.time_by_try_from(&[u64::MAX, 0, 0]).unwrap_err();
	assert_try_from_int_error(&e);

	let e = vc.time_by_try_from(&[0, u64::MAX, 0]).unwrap_err();
	assert_try_from_int_error(&e);

	let e = vc.time_by_try_from(&[0, 0, u64::MAX]).unwrap_err();
	assert_try_from_int_error(&e);

	let e = vc.time_by_try_from(&[char::MAX, char::from(0), char::from(0)]).unwrap_err();
	assert_try_from_char_error(&e);

	let e = vc.time_by_try_from(&[char::from(0), char::MAX, char::from(0)]).unwrap_err();
	assert_try_from_char_error(&e);

	let e = vc.time_by_try_from(&[char::from(0), char::from(0), char::MAX]).unwrap_err();
	assert_try_from_char_error(&e);
}

#[test]
fn clock_time_by_try_from_signed_test() {
	let mut vc = new_as_by::<i8, _>(&[i8::MIN; 3], 1).unwrap();
	assert!(vc.time_by_try_from(&[i8::MIN + 0, i8::MIN + 0, i8::MIN + 0]).unwrap() == [i8::MIN + 0, i8::MIN + 1, i8::MIN + 0]);
	assert!(vc.time_by_try_from(&[i8::MIN + 1, i8::MIN + 0, i8::MIN + 3]).unwrap() == [i8::MIN + 1, i8::MIN + 2, i8::MIN + 3]);
	assert!(vc.time_by_try_from(&[i8::MAX - 0, i8::MIN + 2, i8::MAX - 1]).unwrap() == [i8::MAX - 0, i8::MIN + 3, i8::MAX - 1]);
	assert!(vc.time_by_try_from(&[i8::MIN + 3, i8::MIN + 3, i8::MIN + 3]).unwrap() == [i8::MAX - 0, i8::MIN + 4, i8::MAX - 1]);

	let e = vc.time_by_try_from(&[0; 2]).unwrap_err();
	assert_unmatch_time_size(&e);

	let e = vc.time_by_try_from(&[0; 4]).unwrap_err();
	assert_unmatch_time_size(&e);

	let e = vc.time_by_try_from(&[0, i8::MAX, 0]).unwrap_err();
	assert_invalid_time_value(&e);

	let e = vc.time_by_try_from(&[i64::MIN, 0, 0]).unwrap_err();
	assert_try_from_int_error(&e);

	let e = vc.time_by_try_from(&[0, i64::MIN, 0]).unwrap_err();
	assert_try_from_int_error(&e);

	let e = vc.time_by_try_from(&[0, 0, i64::MIN]).unwrap_err();
	assert_try_from_int_error(&e);

	let e = vc.time_by_try_from(&[u64::MAX, 0, 0]).unwrap_err();
	assert_try_from_int_error(&e);

	let e = vc.time_by_try_from(&[0, u64::MAX, 0]).unwrap_err();
	assert_try_from_int_error(&e);

	let e = vc.time_by_try_from(&[0, 0, u64::MAX]).unwrap_err();
	assert_try_from_int_error(&e);
}

#[test]
#[should_panic(expected = "attempt to add with overflow")]
fn clock_time_by_try_from_overflow_test() {
	let mut vc = new_by(&[0u64, !0, 0], 1).unwrap();
	let _ = vc.time_by_try_from(&[0; 3]);
}

#[test]
fn clock_nocheck_time_by_test() {
	let mut vc = new(3, 1).unwrap();
	assert!(vc.nocheck_time_by(&[0u64; 3]) == [0, 1, 0]);
	assert!(vc.nocheck_time_by(&[1u64, 1, 3]) == [1, 2, 3]);
	assert!(vc.nocheck_time_by(&[!0u64, 0, !0 - 1]) == [!0, 3, !0 - 1]);
	assert!(vc.nocheck_time_by(&[char::from(3); 3]) == [!0, 4, !0 - 1]);

	assert!(vc.nocheck_time_by(&[0u64; 2]) == [!0, 5, !0 - 1]);
	assert!(vc.nocheck_time_by(&[0u64; 4]) == [!0, 6, !0 - 1]);
}

#[test]
#[should_panic(expected = "attempt to add with overflow")]
fn clock_nocheck_time_by_try_from_overflow_test() {
	let mut vc = new(3, 1).unwrap();
	let _ = vc.nocheck_time_by(&[0u64, !0, 0]);
}

#[test]
fn clock_compare_test() {
	let vc = new_by(&[1u64; 3], 1).unwrap();
	assert_eq!(vc.compare(&[0u64, 0, 0]).unwrap(), CompareState::After);
	assert_eq!(vc.compare(&[0u64, 0, 1]).unwrap(), CompareState::After);
	assert_eq!(vc.compare(&[0u64, 0, 2]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare(&[0u64, 1, 0]).unwrap(), CompareState::After);
	assert_eq!(vc.compare(&[0u64, 1, 1]).unwrap(), CompareState::After);
	assert_eq!(vc.compare(&[0u64, 1, 2]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare(&[0u64, 2, 0]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare(&[0u64, 2, 1]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare(&[0u64, 2, 2]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare(&[1u64, 0, 0]).unwrap(), CompareState::After);
	assert_eq!(vc.compare(&[1u64, 0, 1]).unwrap(), CompareState::After);
	assert_eq!(vc.compare(&[1u64, 0, 2]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare(&[1u64, 1, 0]).unwrap(), CompareState::After);
	assert_eq!(vc.compare(&[1u64, 1, 1]).unwrap(), CompareState::Same);
	assert_eq!(vc.compare(&[1u64, 1, 2]).unwrap(), CompareState::Before);
	assert_eq!(vc.compare(&[1u64, 2, 0]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare(&[1u64, 2, 1]).unwrap(), CompareState::Before);
	assert_eq!(vc.compare(&[1u64, 2, 2]).unwrap(), CompareState::Before);
	assert_eq!(vc.compare(&[2u64, 0, 0]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare(&[2u64, 0, 1]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare(&[2u64, 0, 2]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare(&[2u64, 1, 0]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare(&[2u64, 1, 1]).unwrap(), CompareState::Before);
	assert_eq!(vc.compare(&[2u64, 1, 2]).unwrap(), CompareState::Before);
	assert_eq!(vc.compare(&[2u64, 2, 0]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare(&[2u64, 2, 1]).unwrap(), CompareState::Before);
	assert_eq!(vc.compare(&[2u64, 2, 2]).unwrap(), CompareState::Before);

	assert_eq!(vc.compare(&[1u8; 3]).unwrap(), CompareState::Same);
	assert_eq!(vc.compare(&[1u16; 3]).unwrap(), CompareState::Same);
	assert_eq!(vc.compare(&[1u32; 3]).unwrap(), CompareState::Same);
	assert_eq!(vc.compare(&[1u64; 3]).unwrap(), CompareState::Same);
	assert_eq!(vc.compare(&[true; 3]).unwrap(), CompareState::Same);
	assert_eq!(vc.compare(&[char::from(1); 3]).unwrap(), CompareState::Same);

	let e = vc.compare(&[0u64; 2]).unwrap_err();
	assert_unmatch_time_size(&e);

	let e = vc.compare(&[0u64; 4]).unwrap_err();
	assert_unmatch_time_size(&e);
}

#[test]
fn clock_compare_try_from_unsigned_test() {
	let vc = new_as_by::<u8, _>(&[1u8; 3], 1).unwrap();
	assert_eq!(vc.compare_try_from(&[0, 0, 0]).unwrap(), CompareState::After);
	assert_eq!(vc.compare_try_from(&[0, 0, 1]).unwrap(), CompareState::After);
	assert_eq!(vc.compare_try_from(&[0, 0, 2]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare_try_from(&[0, 1, 0]).unwrap(), CompareState::After);
	assert_eq!(vc.compare_try_from(&[0, 1, 1]).unwrap(), CompareState::After);
	assert_eq!(vc.compare_try_from(&[0, 1, 2]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare_try_from(&[0, 2, 0]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare_try_from(&[0, 2, 1]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare_try_from(&[0, 2, 2]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare_try_from(&[1, 0, 0]).unwrap(), CompareState::After);
	assert_eq!(vc.compare_try_from(&[1, 0, 1]).unwrap(), CompareState::After);
	assert_eq!(vc.compare_try_from(&[1, 0, 2]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare_try_from(&[1, 1, 0]).unwrap(), CompareState::After);
	assert_eq!(vc.compare_try_from(&[1, 1, 1]).unwrap(), CompareState::Same);
	assert_eq!(vc.compare_try_from(&[1, 1, 2]).unwrap(), CompareState::Before);
	assert_eq!(vc.compare_try_from(&[1, 2, 0]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare_try_from(&[1, 2, 1]).unwrap(), CompareState::Before);
	assert_eq!(vc.compare_try_from(&[1, 2, 2]).unwrap(), CompareState::Before);
	assert_eq!(vc.compare_try_from(&[2, 0, 0]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare_try_from(&[2, 0, 1]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare_try_from(&[2, 0, 2]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare_try_from(&[2, 1, 0]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare_try_from(&[2, 1, 1]).unwrap(), CompareState::Before);
	assert_eq!(vc.compare_try_from(&[2, 1, 2]).unwrap(), CompareState::Before);
	assert_eq!(vc.compare_try_from(&[2, 2, 0]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare_try_from(&[2, 2, 1]).unwrap(), CompareState::Before);
	assert_eq!(vc.compare_try_from(&[2, 2, 2]).unwrap(), CompareState::Before);

	assert_eq!(vc.compare_try_from(&[1u8; 3]).unwrap(), CompareState::Same);
	assert_eq!(vc.compare_try_from(&[1u16; 3]).unwrap(), CompareState::Same);
	assert_eq!(vc.compare_try_from(&[1u32; 3]).unwrap(), CompareState::Same);
	assert_eq!(vc.compare_try_from(&[1u64; 3]).unwrap(), CompareState::Same);
	assert_eq!(vc.compare_try_from(&[1i8; 3]).unwrap(), CompareState::Same);
	assert_eq!(vc.compare_try_from(&[1i16; 3]).unwrap(), CompareState::Same);
	assert_eq!(vc.compare_try_from(&[1i32; 3]).unwrap(), CompareState::Same);
	assert_eq!(vc.compare_try_from(&[1i64; 3]).unwrap(), CompareState::Same);
	assert_eq!(vc.compare_try_from(&[true; 3]).unwrap(), CompareState::Same);
	assert_eq!(vc.compare_try_from(&[char::from(1); 3]).unwrap(), CompareState::Same);

	let e = vc.compare_try_from(&[0; 2]).unwrap_err();
	assert_unmatch_time_size(&e);

	let e = vc.compare_try_from(&[0; 4]).unwrap_err();
	assert_unmatch_time_size(&e);

	let e = vc.compare_try_from(&[-1, 0, 0]).unwrap_err();
	assert_try_from_int_error(&e);

	let e = vc.compare_try_from(&[!0u64, 0, 0]).unwrap_err();
	assert_try_from_int_error(&e);

	let e = vc.compare_try_from(&[char::MAX, char::from(0), char::from(0)]).unwrap_err();
	assert_try_from_char_error(&e);
}

#[test]
fn clock_compare_try_from_signed_test() {
	let vc = new_as_by::<i8, _>(&[i8::MIN + 1; 3], 1).unwrap();
	assert_eq!(vc.compare_try_from(&[i8::MIN + 0, i8::MIN + 0, i8::MIN + 0]).unwrap(), CompareState::After);
	assert_eq!(vc.compare_try_from(&[i8::MIN + 0, i8::MIN + 0, i8::MIN + 1]).unwrap(), CompareState::After);
	assert_eq!(vc.compare_try_from(&[i8::MIN + 0, i8::MIN + 0, i8::MIN + 2]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare_try_from(&[i8::MIN + 0, i8::MIN + 1, i8::MIN + 0]).unwrap(), CompareState::After);
	assert_eq!(vc.compare_try_from(&[i8::MIN + 0, i8::MIN + 1, i8::MIN + 1]).unwrap(), CompareState::After);
	assert_eq!(vc.compare_try_from(&[i8::MIN + 0, i8::MIN + 1, i8::MIN + 2]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare_try_from(&[i8::MIN + 0, i8::MIN + 2, i8::MIN + 0]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare_try_from(&[i8::MIN + 0, i8::MIN + 2, i8::MIN + 1]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare_try_from(&[i8::MIN + 0, i8::MIN + 2, i8::MIN + 2]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare_try_from(&[i8::MIN + 1, i8::MIN + 0, i8::MIN + 0]).unwrap(), CompareState::After);
	assert_eq!(vc.compare_try_from(&[i8::MIN + 1, i8::MIN + 0, i8::MIN + 1]).unwrap(), CompareState::After);
	assert_eq!(vc.compare_try_from(&[i8::MIN + 1, i8::MIN + 0, i8::MIN + 2]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare_try_from(&[i8::MIN + 1, i8::MIN + 1, i8::MIN + 0]).unwrap(), CompareState::After);
	assert_eq!(vc.compare_try_from(&[i8::MIN + 1, i8::MIN + 1, i8::MIN + 1]).unwrap(), CompareState::Same);
	assert_eq!(vc.compare_try_from(&[i8::MIN + 1, i8::MIN + 1, i8::MIN + 2]).unwrap(), CompareState::Before);
	assert_eq!(vc.compare_try_from(&[i8::MIN + 1, i8::MIN + 2, i8::MIN + 0]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare_try_from(&[i8::MIN + 1, i8::MIN + 2, i8::MIN + 1]).unwrap(), CompareState::Before);
	assert_eq!(vc.compare_try_from(&[i8::MIN + 1, i8::MIN + 2, i8::MIN + 2]).unwrap(), CompareState::Before);
	assert_eq!(vc.compare_try_from(&[i8::MIN + 2, i8::MIN + 0, i8::MIN + 0]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare_try_from(&[i8::MIN + 2, i8::MIN + 0, i8::MIN + 1]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare_try_from(&[i8::MIN + 2, i8::MIN + 0, i8::MIN + 2]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare_try_from(&[i8::MIN + 2, i8::MIN + 1, i8::MIN + 0]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare_try_from(&[i8::MIN + 2, i8::MIN + 1, i8::MIN + 1]).unwrap(), CompareState::Before);
	assert_eq!(vc.compare_try_from(&[i8::MIN + 2, i8::MIN + 1, i8::MIN + 2]).unwrap(), CompareState::Before);
	assert_eq!(vc.compare_try_from(&[i8::MIN + 2, i8::MIN + 2, i8::MIN + 0]).unwrap(), CompareState::Concurrent);
	assert_eq!(vc.compare_try_from(&[i8::MIN + 2, i8::MIN + 2, i8::MIN + 1]).unwrap(), CompareState::Before);
	assert_eq!(vc.compare_try_from(&[i8::MIN + 2, i8::MIN + 2, i8::MIN + 2]).unwrap(), CompareState::Before);

	assert_eq!(vc.compare_try_from(&[1u8; 3]).unwrap(), CompareState::Before);
	assert_eq!(vc.compare_try_from(&[1u16; 3]).unwrap(), CompareState::Before);
	assert_eq!(vc.compare_try_from(&[1u32; 3]).unwrap(), CompareState::Before);
	assert_eq!(vc.compare_try_from(&[1u64; 3]).unwrap(), CompareState::Before);
	assert_eq!(vc.compare_try_from(&[1i8; 3]).unwrap(), CompareState::Before);
	assert_eq!(vc.compare_try_from(&[1i16; 3]).unwrap(), CompareState::Before);
	assert_eq!(vc.compare_try_from(&[1i32; 3]).unwrap(), CompareState::Before);
	assert_eq!(vc.compare_try_from(&[1i64; 3]).unwrap(), CompareState::Before);
	assert_eq!(vc.compare_try_from(&[true; 3]).unwrap(), CompareState::Before);

	let e = vc.compare_try_from(&[0; 2]).unwrap_err();
	assert_unmatch_time_size(&e);

	let e = vc.compare_try_from(&[0; 4]).unwrap_err();
	assert_unmatch_time_size(&e);

	let e = vc.compare_try_from(&[i64::MIN, 0, 0]).unwrap_err();
	assert_try_from_int_error(&e);

	let e = vc.compare_try_from(&[!0u64, 0, 0]).unwrap_err();
	assert_try_from_int_error(&e);
}

#[test]
fn clock_nocheck_compare_test() {
	let vc = new_by(&[1u64; 3], 1).unwrap();
	assert_eq!(vc.nocheck_compare(&[0u64, 0, 0]), CompareState::After);
	assert_eq!(vc.nocheck_compare(&[0u64, 0, 1]), CompareState::After);
	assert_eq!(vc.nocheck_compare(&[0u64, 0, 2]), CompareState::Concurrent);
	assert_eq!(vc.nocheck_compare(&[0u64, 1, 0]), CompareState::After);
	assert_eq!(vc.nocheck_compare(&[0u64, 1, 1]), CompareState::After);
	assert_eq!(vc.nocheck_compare(&[0u64, 1, 2]), CompareState::Concurrent);
	assert_eq!(vc.nocheck_compare(&[0u64, 2, 0]), CompareState::Concurrent);
	assert_eq!(vc.nocheck_compare(&[0u64, 2, 1]), CompareState::Concurrent);
	assert_eq!(vc.nocheck_compare(&[0u64, 2, 2]), CompareState::Concurrent);
	assert_eq!(vc.nocheck_compare(&[1u64, 0, 0]), CompareState::After);
	assert_eq!(vc.nocheck_compare(&[1u64, 0, 1]), CompareState::After);
	assert_eq!(vc.nocheck_compare(&[1u64, 0, 2]), CompareState::Concurrent);
	assert_eq!(vc.nocheck_compare(&[1u64, 1, 0]), CompareState::After);
	assert_eq!(vc.nocheck_compare(&[1u64, 1, 1]), CompareState::Same);
	assert_eq!(vc.nocheck_compare(&[1u64, 1, 2]), CompareState::Before);
	assert_eq!(vc.nocheck_compare(&[1u64, 2, 0]), CompareState::Concurrent);
	assert_eq!(vc.nocheck_compare(&[1u64, 2, 1]), CompareState::Before);
	assert_eq!(vc.nocheck_compare(&[1u64, 2, 2]), CompareState::Before);
	assert_eq!(vc.nocheck_compare(&[2u64, 0, 0]), CompareState::Concurrent);
	assert_eq!(vc.nocheck_compare(&[2u64, 0, 1]), CompareState::Concurrent);
	assert_eq!(vc.nocheck_compare(&[2u64, 0, 2]), CompareState::Concurrent);
	assert_eq!(vc.nocheck_compare(&[2u64, 1, 0]), CompareState::Concurrent);
	assert_eq!(vc.nocheck_compare(&[2u64, 1, 1]), CompareState::Before);
	assert_eq!(vc.nocheck_compare(&[2u64, 1, 2]), CompareState::Before);
	assert_eq!(vc.nocheck_compare(&[2u64, 2, 0]), CompareState::Concurrent);
	assert_eq!(vc.nocheck_compare(&[2u64, 2, 1]), CompareState::Before);
	assert_eq!(vc.nocheck_compare(&[2u64, 2, 2]), CompareState::Before);

	assert_eq!(vc.nocheck_compare(&[1u8; 3]), CompareState::Same);
	assert_eq!(vc.nocheck_compare(&[1u16; 3]), CompareState::Same);
	assert_eq!(vc.nocheck_compare(&[1u32; 3]), CompareState::Same);
	assert_eq!(vc.nocheck_compare(&[1u64; 3]), CompareState::Same);
	assert_eq!(vc.nocheck_compare(&[true; 3]), CompareState::Same);
	assert_eq!(vc.nocheck_compare(&[char::from(1); 3]), CompareState::Same);

	assert_eq!(vc.nocheck_compare(&[0u64, 2]), CompareState::Concurrent);
	assert_eq!(vc.nocheck_compare(&[1u64, 0]), CompareState::After);
	assert_eq!(vc.nocheck_compare(&[1u64, 1]), CompareState::Same);
	assert_eq!(vc.nocheck_compare(&[1u64, 2]), CompareState::Before);
	assert_eq!(vc.nocheck_compare(&[2u64, 0]), CompareState::Concurrent);

	assert_eq!(vc.nocheck_compare(&[0u64, 1, 2, 0]), CompareState::Concurrent);
	assert_eq!(vc.nocheck_compare(&[0u64, 1, 2, 1]), CompareState::Concurrent);
	assert_eq!(vc.nocheck_compare(&[0u64, 1, 2, 2]), CompareState::Concurrent);
	assert_eq!(vc.nocheck_compare(&[1u64, 1, 0, 0]), CompareState::After);
	assert_eq!(vc.nocheck_compare(&[1u64, 1, 0, 1]), CompareState::After);
	assert_eq!(vc.nocheck_compare(&[1u64, 1, 0, 2]), CompareState::After);
	assert_eq!(vc.nocheck_compare(&[1u64, 1, 1, 0]), CompareState::Same);
	assert_eq!(vc.nocheck_compare(&[1u64, 1, 1, 1]), CompareState::Same);
	assert_eq!(vc.nocheck_compare(&[1u64, 1, 1, 2]), CompareState::Same);
	assert_eq!(vc.nocheck_compare(&[1u64, 1, 2, 0]), CompareState::Before);
	assert_eq!(vc.nocheck_compare(&[1u64, 1, 2, 1]), CompareState::Before);
	assert_eq!(vc.nocheck_compare(&[1u64, 1, 2, 2]), CompareState::Before);
	assert_eq!(vc.nocheck_compare(&[2u64, 1, 0, 0]), CompareState::Concurrent);
	assert_eq!(vc.nocheck_compare(&[2u64, 1, 0, 1]), CompareState::Concurrent);
	assert_eq!(vc.nocheck_compare(&[2u64, 1, 0, 2]), CompareState::Concurrent);
}

#[test]
fn new_test() {
	new(3, 1).unwrap();

	let e = new(3, 3).unwrap_err();
	assert_out_of_range_index(&e);

	let e = new(0, 0).unwrap_err();
	assert_out_of_range_index(&e);
}

#[test]
fn new_by_error_type_test() {
	let e = new_by(&[0u8; 3], 3).unwrap_err();
	assert_out_of_range_index(&e);

	let e = new_by::<u8>(&[], 0).unwrap_err();
	assert_out_of_range_index(&e);
}

#[test]
fn new_by_empty_test() {
	new_by::<u8>(&[], 0).unwrap_err();
	new_by::<u16>(&[], 0).unwrap_err();
	new_by::<u32>(&[], 0).unwrap_err();
	new_by::<u64>(&[], 0).unwrap_err();
	new_by::<char>(&[], 0).unwrap_err();
	new_by::<bool>(&[], 0).unwrap_err();
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
fn new_by_try_from_error_type_test() {
	let e = new_by_try_from(&[0; 3], 3).unwrap_err();
	assert_out_of_range_index(&e);

	let e = new_by_try_from::<u8>(&[], 0).unwrap_err();
	assert_out_of_range_index(&e);

	let e = new_by_try_from(&[-1; 3], 1).unwrap_err();
	assert_try_from_int_error(&e);
}

#[test]
fn new_by_try_from_empty_test() {
	new_by_try_from::<i8>(&[], 0).unwrap_err();
	new_by_try_from::<i16>(&[], 0).unwrap_err();
	new_by_try_from::<i32>(&[], 0).unwrap_err();
	new_by_try_from::<i64>(&[], 0).unwrap_err();
	new_by_try_from::<u8>(&[], 0).unwrap_err();
	new_by_try_from::<u16>(&[], 0).unwrap_err();
	new_by_try_from::<u32>(&[], 0).unwrap_err();
	new_by_try_from::<u64>(&[], 0).unwrap_err();
	new_by_try_from::<char>(&[], 0).unwrap_err();
	new_by_try_from::<bool>(&[], 0).unwrap_err();
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

	let e = new_as::<u8>(3, 3).unwrap_err();
	assert_out_of_range_index(&e);

	new_as::<u8>(3, 3).unwrap_err();
	new_as::<u16>(3, 3).unwrap_err();
	new_as::<u32>(3, 3).unwrap_err();
	new_as::<u64>(3, 3).unwrap_err();
	new_as::<i8>(3, 3).unwrap_err();
	new_as::<i16>(3, 3).unwrap_err();
	new_as::<i32>(3, 3).unwrap_err();
	new_as::<i64>(3, 3).unwrap_err();

	let e = new_as::<u8>(0, 0).unwrap_err();
	assert_out_of_range_index(&e);

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
fn new_as_by_error_type_test() {
	let e = new_as_by::<u8, _>(&[0; 3], 3).unwrap_err();
	assert_out_of_range_index(&e);

	let e = new_as_by::<u8, u8>(&[], 0).unwrap_err();
	assert_out_of_range_index(&e);
}

#[test]
fn new_as_by_empty_test() {
	new_as_by::<u8, u8>(&[], 0).unwrap_err();
	new_as_by::<u8, bool>(&[], 0).unwrap_err();

	new_as_by::<u16, u8>(&[], 0).unwrap_err();
	new_as_by::<u16, u16>(&[], 0).unwrap_err();
	new_as_by::<u16, bool>(&[], 0).unwrap_err();

	new_as_by::<u32, u8>(&[], 0).unwrap_err();
	new_as_by::<u32, u16>(&[], 0).unwrap_err();
	new_as_by::<u32, u32>(&[], 0).unwrap_err();
	new_as_by::<u32, char>(&[], 0).unwrap_err();
	new_as_by::<u32, bool>(&[], 0).unwrap_err();

	new_as_by::<u64, u8>(&[], 0).unwrap_err();
	new_as_by::<u64, u16>(&[], 0).unwrap_err();
	new_as_by::<u64, u32>(&[], 0).unwrap_err();
	new_as_by::<u64, u64>(&[], 0).unwrap_err();
	new_as_by::<u64, char>(&[], 0).unwrap_err();
	new_as_by::<u64, bool>(&[], 0).unwrap_err();

	new_as_by::<i8, i8>(&[], 0).unwrap_err();
	new_as_by::<i8, bool>(&[], 0).unwrap_err();

	new_as_by::<i16, u8>(&[], 0).unwrap_err();
	new_as_by::<i16, i8>(&[], 0).unwrap_err();
	new_as_by::<i16, i16>(&[], 0).unwrap_err();
	new_as_by::<i16, bool>(&[], 0).unwrap_err();

	new_as_by::<i32, u8>(&[], 0).unwrap_err();
	new_as_by::<i32, u16>(&[], 0).unwrap_err();
	new_as_by::<i32, i8>(&[], 0).unwrap_err();
	new_as_by::<i32, i16>(&[], 0).unwrap_err();
	new_as_by::<i32, i32>(&[], 0).unwrap_err();
	new_as_by::<i32, bool>(&[], 0).unwrap_err();

	new_as_by::<i64, u8>(&[], 0).unwrap_err();
	new_as_by::<i64, u16>(&[], 0).unwrap_err();
	new_as_by::<i64, u32>(&[], 0).unwrap_err();
	new_as_by::<i64, i8>(&[], 0).unwrap_err();
	new_as_by::<i64, i16>(&[], 0).unwrap_err();
	new_as_by::<i64, i32>(&[], 0).unwrap_err();
	new_as_by::<i64, i64>(&[], 0).unwrap_err();
	new_as_by::<i64, bool>(&[], 0).unwrap_err();
}

#[test]
fn new_as_by_zero_test() {
	new_as_by::<u8, u8>(&[0; 3], 1).unwrap();
	new_as_by::<u8, bool>(&[false; 3], 1).unwrap();

	new_as_by::<u16, u8>(&[0; 3], 1).unwrap();
	new_as_by::<u16, u16>(&[0; 3], 1).unwrap();
	new_as_by::<u16, bool>(&[false; 3], 1).unwrap();

	new_as_by::<u32, u8>(&[0; 3], 1).unwrap();
	new_as_by::<u32, u16>(&[0; 3], 1).unwrap();
	new_as_by::<u32, u32>(&[0; 3], 1).unwrap();
	new_as_by::<u32, char>(&[char::from(0); 3], 1).unwrap();
	new_as_by::<u32, bool>(&[false; 3], 1).unwrap();

	new_as_by::<u64, u8>(&[0; 3], 1).unwrap();
	new_as_by::<u64, u16>(&[0; 3], 1).unwrap();
	new_as_by::<u64, u32>(&[0; 3], 1).unwrap();
	new_as_by::<u64, u64>(&[0; 3], 1).unwrap();
	new_as_by::<u64, char>(&[char::from(0); 3], 1).unwrap();
	new_as_by::<u64, bool>(&[false; 3], 1).unwrap();

	new_as_by::<i8, i8>(&[0; 3], 1).unwrap();
	new_as_by::<i8, bool>(&[false; 3], 1).unwrap();

	new_as_by::<i16, u8>(&[0; 3], 1).unwrap();
	new_as_by::<i16, i8>(&[0; 3], 1).unwrap();
	new_as_by::<i16, i16>(&[0; 3], 1).unwrap();
	new_as_by::<i16, bool>(&[false; 3], 1).unwrap();

	new_as_by::<i32, u8>(&[0; 3], 1).unwrap();
	new_as_by::<i32, u16>(&[0; 3], 1).unwrap();
	new_as_by::<i32, i8>(&[0; 3], 1).unwrap();
	new_as_by::<i32, i16>(&[0; 3], 1).unwrap();
	new_as_by::<i32, i32>(&[0; 3], 1).unwrap();
	new_as_by::<i32, bool>(&[false; 3], 1).unwrap();

	new_as_by::<i64, u8>(&[0; 3], 1).unwrap();
	new_as_by::<i64, u16>(&[0; 3], 1).unwrap();
	new_as_by::<i64, u32>(&[0; 3], 1).unwrap();
	new_as_by::<i64, i8>(&[0; 3], 1).unwrap();
	new_as_by::<i64, i16>(&[0; 3], 1).unwrap();
	new_as_by::<i64, i32>(&[0; 3], 1).unwrap();
	new_as_by::<i64, i64>(&[0; 3], 1).unwrap();
	new_as_by::<i64, bool>(&[false; 3], 1).unwrap();
}

#[test]
fn new_as_by_one_test() {
	new_as_by::<u8, u8>(&[1; 3], 1).unwrap();
	new_as_by::<u8, bool>(&[true; 3], 1).unwrap();

	new_as_by::<u16, u8>(&[1; 3], 1).unwrap();
	new_as_by::<u16, u16>(&[1; 3], 1).unwrap();
	new_as_by::<u16, bool>(&[true; 3], 1).unwrap();

	new_as_by::<u32, u8>(&[1; 3], 1).unwrap();
	new_as_by::<u32, u16>(&[1; 3], 1).unwrap();
	new_as_by::<u32, u32>(&[1; 3], 1).unwrap();
	new_as_by::<u32, char>(&[char::from(1); 3], 1).unwrap();
	new_as_by::<u32, bool>(&[true; 3], 1).unwrap();

	new_as_by::<u64, u8>(&[1; 3], 1).unwrap();
	new_as_by::<u64, u16>(&[1; 3], 1).unwrap();
	new_as_by::<u64, u32>(&[1; 3], 1).unwrap();
	new_as_by::<u64, u64>(&[1; 3], 1).unwrap();
	new_as_by::<u64, char>(&[char::from(1); 3], 1).unwrap();
	new_as_by::<u64, bool>(&[true; 3], 1).unwrap();

	new_as_by::<i8, i8>(&[1; 3], 1).unwrap();
	new_as_by::<i8, bool>(&[true; 3], 1).unwrap();

	new_as_by::<i16, u8>(&[1; 3], 1).unwrap();
	new_as_by::<i16, i8>(&[1; 3], 1).unwrap();
	new_as_by::<i16, i16>(&[1; 3], 1).unwrap();
	new_as_by::<i16, bool>(&[true; 3], 1).unwrap();

	new_as_by::<i32, u8>(&[1; 3], 1).unwrap();
	new_as_by::<i32, u16>(&[1; 3], 1).unwrap();
	new_as_by::<i32, i8>(&[1; 3], 1).unwrap();
	new_as_by::<i32, i16>(&[1; 3], 1).unwrap();
	new_as_by::<i32, i32>(&[1; 3], 1).unwrap();
	new_as_by::<i32, bool>(&[true; 3], 1).unwrap();

	new_as_by::<i64, u8>(&[1; 3], 1).unwrap();
	new_as_by::<i64, u16>(&[1; 3], 1).unwrap();
	new_as_by::<i64, u32>(&[1; 3], 1).unwrap();
	new_as_by::<i64, i8>(&[1; 3], 1).unwrap();
	new_as_by::<i64, i16>(&[1; 3], 1).unwrap();
	new_as_by::<i64, i32>(&[1; 3], 1).unwrap();
	new_as_by::<i64, i64>(&[1; 3], 1).unwrap();
	new_as_by::<i64, bool>(&[true; 3], 1).unwrap();
}

#[test]
fn new_as_by_u8_max_test() {
	new_as_by::<u8, u8>(&[u8::MAX as u8; 3], 1).unwrap();

	new_as_by::<u16, u8>(&[u8::MAX as u8; 3], 1).unwrap();
	new_as_by::<u16, u16>(&[u8::MAX as u16; 3], 1).unwrap();

	new_as_by::<u32, u8>(&[u8::MAX as u8; 3], 1).unwrap();
	new_as_by::<u32, u16>(&[u8::MAX as u16; 3], 1).unwrap();
	new_as_by::<u32, u32>(&[u8::MAX as u32; 3], 1).unwrap();

	new_as_by::<u64, u8>(&[u8::MAX as u8; 3], 1).unwrap();
	new_as_by::<u64, u16>(&[u8::MAX as u16; 3], 1).unwrap();
	new_as_by::<u64, u32>(&[u8::MAX as u32; 3], 1).unwrap();
	new_as_by::<u64, u64>(&[u8::MAX as u64; 3], 1).unwrap();

	new_as_by::<i8, i8>(&[u8::MAX as i8; 3], 1).unwrap();

	new_as_by::<i16, u8>(&[u8::MAX as u8; 3], 1).unwrap();
	new_as_by::<i16, i8>(&[u8::MAX as i8; 3], 1).unwrap();
	new_as_by::<i16, i16>(&[u8::MAX as i16; 3], 1).unwrap();

	new_as_by::<i32, u8>(&[u8::MAX as u8; 3], 1).unwrap();
	new_as_by::<i32, u16>(&[u8::MAX as u16; 3], 1).unwrap();
	new_as_by::<i32, i8>(&[u8::MAX as i8; 3], 1).unwrap();
	new_as_by::<i32, i16>(&[u8::MAX as i16; 3], 1).unwrap();
	new_as_by::<i32, i32>(&[u8::MAX as i32; 3], 1).unwrap();

	new_as_by::<i64, u8>(&[u8::MAX as u8; 3], 1).unwrap();
	new_as_by::<i64, u16>(&[u8::MAX as u16; 3], 1).unwrap();
	new_as_by::<i64, u32>(&[u8::MAX as u32; 3], 1).unwrap();
	new_as_by::<i64, i8>(&[u8::MAX as i8; 3], 1).unwrap();
	new_as_by::<i64, i16>(&[u8::MAX as i16; 3], 1).unwrap();
	new_as_by::<i64, i32>(&[u8::MAX as i32; 3], 1).unwrap();
	new_as_by::<i64, i64>(&[u8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_u16_max_test() {
	new_as_by::<u8, u8>(&[u16::MAX as u8; 3], 1).unwrap();

	new_as_by::<u16, u8>(&[u16::MAX as u8; 3], 1).unwrap();
	new_as_by::<u16, u16>(&[u16::MAX as u16; 3], 1).unwrap();

	new_as_by::<u32, u8>(&[u16::MAX as u8; 3], 1).unwrap();
	new_as_by::<u32, u16>(&[u16::MAX as u16; 3], 1).unwrap();
	new_as_by::<u32, u32>(&[u16::MAX as u32; 3], 1).unwrap();

	new_as_by::<u64, u8>(&[u16::MAX as u8; 3], 1).unwrap();
	new_as_by::<u64, u16>(&[u16::MAX as u16; 3], 1).unwrap();
	new_as_by::<u64, u32>(&[u16::MAX as u32; 3], 1).unwrap();
	new_as_by::<u64, u64>(&[u16::MAX as u64; 3], 1).unwrap();

	new_as_by::<i8, i8>(&[u16::MAX as i8; 3], 1).unwrap();

	new_as_by::<i16, u8>(&[u16::MAX as u8; 3], 1).unwrap();
	new_as_by::<i16, i8>(&[u16::MAX as i8; 3], 1).unwrap();
	new_as_by::<i16, i16>(&[u16::MAX as i16; 3], 1).unwrap();

	new_as_by::<i32, u8>(&[u16::MAX as u8; 3], 1).unwrap();
	new_as_by::<i32, u16>(&[u16::MAX as u16; 3], 1).unwrap();
	new_as_by::<i32, i8>(&[u16::MAX as i8; 3], 1).unwrap();
	new_as_by::<i32, i16>(&[u16::MAX as i16; 3], 1).unwrap();
	new_as_by::<i32, i32>(&[u16::MAX as i32; 3], 1).unwrap();

	new_as_by::<i64, u8>(&[u16::MAX as u8; 3], 1).unwrap();
	new_as_by::<i64, u16>(&[u16::MAX as u16; 3], 1).unwrap();
	new_as_by::<i64, u32>(&[u16::MAX as u32; 3], 1).unwrap();
	new_as_by::<i64, i8>(&[u16::MAX as i8; 3], 1).unwrap();
	new_as_by::<i64, i16>(&[u16::MAX as i16; 3], 1).unwrap();
	new_as_by::<i64, i32>(&[u16::MAX as i32; 3], 1).unwrap();
	new_as_by::<i64, i64>(&[u16::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_u32_max_test() {
	new_as_by::<u8, u8>(&[u32::MAX as u8; 3], 1).unwrap();

	new_as_by::<u16, u8>(&[u32::MAX as u8; 3], 1).unwrap();
	new_as_by::<u16, u16>(&[u32::MAX as u16; 3], 1).unwrap();

	new_as_by::<u32, u8>(&[u32::MAX as u8; 3], 1).unwrap();
	new_as_by::<u32, u16>(&[u32::MAX as u16; 3], 1).unwrap();
	new_as_by::<u32, u32>(&[u32::MAX as u32; 3], 1).unwrap();

	new_as_by::<u64, u8>(&[u32::MAX as u8; 3], 1).unwrap();
	new_as_by::<u64, u16>(&[u32::MAX as u16; 3], 1).unwrap();
	new_as_by::<u64, u32>(&[u32::MAX as u32; 3], 1).unwrap();
	new_as_by::<u64, u64>(&[u32::MAX as u64; 3], 1).unwrap();

	new_as_by::<i8, i8>(&[u32::MAX as i8; 3], 1).unwrap();

	new_as_by::<i16, u8>(&[u32::MAX as u8; 3], 1).unwrap();
	new_as_by::<i16, i8>(&[u32::MAX as i8; 3], 1).unwrap();
	new_as_by::<i16, i16>(&[u32::MAX as i16; 3], 1).unwrap();

	new_as_by::<i32, u8>(&[u32::MAX as u8; 3], 1).unwrap();
	new_as_by::<i32, u16>(&[u32::MAX as u16; 3], 1).unwrap();
	new_as_by::<i32, i8>(&[u32::MAX as i8; 3], 1).unwrap();
	new_as_by::<i32, i16>(&[u32::MAX as i16; 3], 1).unwrap();
	new_as_by::<i32, i32>(&[u32::MAX as i32; 3], 1).unwrap();

	new_as_by::<i64, u8>(&[u32::MAX as u8; 3], 1).unwrap();
	new_as_by::<i64, u16>(&[u32::MAX as u16; 3], 1).unwrap();
	new_as_by::<i64, u32>(&[u32::MAX as u32; 3], 1).unwrap();
	new_as_by::<i64, i8>(&[u32::MAX as i8; 3], 1).unwrap();
	new_as_by::<i64, i16>(&[u32::MAX as i16; 3], 1).unwrap();
	new_as_by::<i64, i32>(&[u32::MAX as i32; 3], 1).unwrap();
	new_as_by::<i64, i64>(&[u32::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_u64_max_test() {
	new_as_by::<u8, u8>(&[u64::MAX as u8; 3], 1).unwrap();

	new_as_by::<u16, u8>(&[u64::MAX as u8; 3], 1).unwrap();
	new_as_by::<u16, u16>(&[u64::MAX as u16; 3], 1).unwrap();

	new_as_by::<u32, u8>(&[u64::MAX as u8; 3], 1).unwrap();
	new_as_by::<u32, u16>(&[u64::MAX as u16; 3], 1).unwrap();
	new_as_by::<u32, u32>(&[u64::MAX as u32; 3], 1).unwrap();

	new_as_by::<u64, u8>(&[u64::MAX as u8; 3], 1).unwrap();
	new_as_by::<u64, u16>(&[u64::MAX as u16; 3], 1).unwrap();
	new_as_by::<u64, u32>(&[u64::MAX as u32; 3], 1).unwrap();
	new_as_by::<u64, u64>(&[u64::MAX as u64; 3], 1).unwrap();

	new_as_by::<i8, i8>(&[u64::MAX as i8; 3], 1).unwrap();

	new_as_by::<i16, u8>(&[u64::MAX as u8; 3], 1).unwrap();
	new_as_by::<i16, i8>(&[u64::MAX as i8; 3], 1).unwrap();
	new_as_by::<i16, i16>(&[u64::MAX as i16; 3], 1).unwrap();

	new_as_by::<i32, u8>(&[u64::MAX as u8; 3], 1).unwrap();
	new_as_by::<i32, u16>(&[u64::MAX as u16; 3], 1).unwrap();
	new_as_by::<i32, i8>(&[u64::MAX as i8; 3], 1).unwrap();
	new_as_by::<i32, i16>(&[u64::MAX as i16; 3], 1).unwrap();
	new_as_by::<i32, i32>(&[u64::MAX as i32; 3], 1).unwrap();

	new_as_by::<i64, u8>(&[u64::MAX as u8; 3], 1).unwrap();
	new_as_by::<i64, u16>(&[u64::MAX as u16; 3], 1).unwrap();
	new_as_by::<i64, u32>(&[u64::MAX as u32; 3], 1).unwrap();
	new_as_by::<i64, i8>(&[u64::MAX as i8; 3], 1).unwrap();
	new_as_by::<i64, i16>(&[u64::MAX as i16; 3], 1).unwrap();
	new_as_by::<i64, i32>(&[u64::MAX as i32; 3], 1).unwrap();
	new_as_by::<i64, i64>(&[u64::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_i8_max_test() {
	new_as_by::<u8, u8>(&[i8::MAX as u8; 3], 1).unwrap();

	new_as_by::<u16, u8>(&[i8::MAX as u8; 3], 1).unwrap();
	new_as_by::<u16, u16>(&[i8::MAX as u16; 3], 1).unwrap();

	new_as_by::<u32, u8>(&[i8::MAX as u8; 3], 1).unwrap();
	new_as_by::<u32, u16>(&[i8::MAX as u16; 3], 1).unwrap();
	new_as_by::<u32, u32>(&[i8::MAX as u32; 3], 1).unwrap();

	new_as_by::<u64, u8>(&[i8::MAX as u8; 3], 1).unwrap();
	new_as_by::<u64, u16>(&[i8::MAX as u16; 3], 1).unwrap();
	new_as_by::<u64, u32>(&[i8::MAX as u32; 3], 1).unwrap();
	new_as_by::<u64, u64>(&[i8::MAX as u64; 3], 1).unwrap();

	new_as_by::<i8, i8>(&[i8::MAX as i8; 3], 1).unwrap();

	new_as_by::<i16, u8>(&[i8::MAX as u8; 3], 1).unwrap();
	new_as_by::<i16, i8>(&[i8::MAX as i8; 3], 1).unwrap();
	new_as_by::<i16, i16>(&[i8::MAX as i16; 3], 1).unwrap();

	new_as_by::<i32, u8>(&[i8::MAX as u8; 3], 1).unwrap();
	new_as_by::<i32, u16>(&[i8::MAX as u16; 3], 1).unwrap();
	new_as_by::<i32, i8>(&[i8::MAX as i8; 3], 1).unwrap();
	new_as_by::<i32, i16>(&[i8::MAX as i16; 3], 1).unwrap();
	new_as_by::<i32, i32>(&[i8::MAX as i32; 3], 1).unwrap();

	new_as_by::<i64, u8>(&[i8::MAX as u8; 3], 1).unwrap();
	new_as_by::<i64, u16>(&[i8::MAX as u16; 3], 1).unwrap();
	new_as_by::<i64, u32>(&[i8::MAX as u32; 3], 1).unwrap();
	new_as_by::<i64, i8>(&[i8::MAX as i8; 3], 1).unwrap();
	new_as_by::<i64, i16>(&[i8::MAX as i16; 3], 1).unwrap();
	new_as_by::<i64, i32>(&[i8::MAX as i32; 3], 1).unwrap();
	new_as_by::<i64, i64>(&[i8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_i16_max_test() {
	new_as_by::<u8, u8>(&[i16::MAX as u8; 3], 1).unwrap();

	new_as_by::<u16, u8>(&[i16::MAX as u8; 3], 1).unwrap();
	new_as_by::<u16, u16>(&[i16::MAX as u16; 3], 1).unwrap();

	new_as_by::<u32, u8>(&[i16::MAX as u8; 3], 1).unwrap();
	new_as_by::<u32, u16>(&[i16::MAX as u16; 3], 1).unwrap();
	new_as_by::<u32, u32>(&[i16::MAX as u32; 3], 1).unwrap();

	new_as_by::<u64, u8>(&[i16::MAX as u8; 3], 1).unwrap();
	new_as_by::<u64, u16>(&[i16::MAX as u16; 3], 1).unwrap();
	new_as_by::<u64, u32>(&[i16::MAX as u32; 3], 1).unwrap();
	new_as_by::<u64, u64>(&[i16::MAX as u64; 3], 1).unwrap();

	new_as_by::<i8, i8>(&[i16::MAX as i8; 3], 1).unwrap();

	new_as_by::<i16, u8>(&[i16::MAX as u8; 3], 1).unwrap();
	new_as_by::<i16, i8>(&[i16::MAX as i8; 3], 1).unwrap();
	new_as_by::<i16, i16>(&[i16::MAX as i16; 3], 1).unwrap();

	new_as_by::<i32, u8>(&[i16::MAX as u8; 3], 1).unwrap();
	new_as_by::<i32, u16>(&[i16::MAX as u16; 3], 1).unwrap();
	new_as_by::<i32, i8>(&[i16::MAX as i8; 3], 1).unwrap();
	new_as_by::<i32, i16>(&[i16::MAX as i16; 3], 1).unwrap();
	new_as_by::<i32, i32>(&[i16::MAX as i32; 3], 1).unwrap();

	new_as_by::<i64, u8>(&[i16::MAX as u8; 3], 1).unwrap();
	new_as_by::<i64, u16>(&[i16::MAX as u16; 3], 1).unwrap();
	new_as_by::<i64, u32>(&[i16::MAX as u32; 3], 1).unwrap();
	new_as_by::<i64, i8>(&[i16::MAX as i8; 3], 1).unwrap();
	new_as_by::<i64, i16>(&[i16::MAX as i16; 3], 1).unwrap();
	new_as_by::<i64, i32>(&[i16::MAX as i32; 3], 1).unwrap();
	new_as_by::<i64, i64>(&[i16::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_i32_max_test() {
	new_as_by::<u8, u8>(&[i32::MAX as u8; 3], 1).unwrap();

	new_as_by::<u16, u8>(&[i32::MAX as u8; 3], 1).unwrap();
	new_as_by::<u16, u16>(&[i32::MAX as u16; 3], 1).unwrap();

	new_as_by::<u32, u8>(&[i32::MAX as u8; 3], 1).unwrap();
	new_as_by::<u32, u16>(&[i32::MAX as u16; 3], 1).unwrap();
	new_as_by::<u32, u32>(&[i32::MAX as u32; 3], 1).unwrap();

	new_as_by::<u64, u8>(&[i32::MAX as u8; 3], 1).unwrap();
	new_as_by::<u64, u16>(&[i32::MAX as u16; 3], 1).unwrap();
	new_as_by::<u64, u32>(&[i32::MAX as u32; 3], 1).unwrap();
	new_as_by::<u64, u64>(&[i32::MAX as u64; 3], 1).unwrap();

	new_as_by::<i8, i8>(&[i32::MAX as i8; 3], 1).unwrap();

	new_as_by::<i16, u8>(&[i32::MAX as u8; 3], 1).unwrap();
	new_as_by::<i16, i8>(&[i32::MAX as i8; 3], 1).unwrap();
	new_as_by::<i16, i16>(&[i32::MAX as i16; 3], 1).unwrap();

	new_as_by::<i32, u8>(&[i32::MAX as u8; 3], 1).unwrap();
	new_as_by::<i32, u16>(&[i32::MAX as u16; 3], 1).unwrap();
	new_as_by::<i32, i8>(&[i32::MAX as i8; 3], 1).unwrap();
	new_as_by::<i32, i16>(&[i32::MAX as i16; 3], 1).unwrap();
	new_as_by::<i32, i32>(&[i32::MAX as i32; 3], 1).unwrap();

	new_as_by::<i64, u8>(&[i32::MAX as u8; 3], 1).unwrap();
	new_as_by::<i64, u16>(&[i32::MAX as u16; 3], 1).unwrap();
	new_as_by::<i64, u32>(&[i32::MAX as u32; 3], 1).unwrap();
	new_as_by::<i64, i8>(&[i32::MAX as i8; 3], 1).unwrap();
	new_as_by::<i64, i16>(&[i32::MAX as i16; 3], 1).unwrap();
	new_as_by::<i64, i32>(&[i32::MAX as i32; 3], 1).unwrap();
	new_as_by::<i64, i64>(&[i32::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_i64_max_test() {
	new_as_by::<u8, u8>(&[i64::MAX as u8; 3], 1).unwrap();

	new_as_by::<u16, u8>(&[i64::MAX as u8; 3], 1).unwrap();
	new_as_by::<u16, u16>(&[i64::MAX as u16; 3], 1).unwrap();

	new_as_by::<u32, u8>(&[i64::MAX as u8; 3], 1).unwrap();
	new_as_by::<u32, u16>(&[i64::MAX as u16; 3], 1).unwrap();
	new_as_by::<u32, u32>(&[i64::MAX as u32; 3], 1).unwrap();

	new_as_by::<u64, u8>(&[i64::MAX as u8; 3], 1).unwrap();
	new_as_by::<u64, u16>(&[i64::MAX as u16; 3], 1).unwrap();
	new_as_by::<u64, u32>(&[i64::MAX as u32; 3], 1).unwrap();
	new_as_by::<u64, u64>(&[i64::MAX as u64; 3], 1).unwrap();

	new_as_by::<i8, i8>(&[i64::MAX as i8; 3], 1).unwrap();

	new_as_by::<i16, u8>(&[i64::MAX as u8; 3], 1).unwrap();
	new_as_by::<i16, i8>(&[i64::MAX as i8; 3], 1).unwrap();
	new_as_by::<i16, i16>(&[i64::MAX as i16; 3], 1).unwrap();

	new_as_by::<i32, u8>(&[i64::MAX as u8; 3], 1).unwrap();
	new_as_by::<i32, u16>(&[i64::MAX as u16; 3], 1).unwrap();
	new_as_by::<i32, i8>(&[i64::MAX as i8; 3], 1).unwrap();
	new_as_by::<i32, i16>(&[i64::MAX as i16; 3], 1).unwrap();
	new_as_by::<i32, i32>(&[i64::MAX as i32; 3], 1).unwrap();

	new_as_by::<i64, u8>(&[i64::MAX as u8; 3], 1).unwrap();
	new_as_by::<i64, u16>(&[i64::MAX as u16; 3], 1).unwrap();
	new_as_by::<i64, u32>(&[i64::MAX as u32; 3], 1).unwrap();
	new_as_by::<i64, i8>(&[i64::MAX as i8; 3], 1).unwrap();
	new_as_by::<i64, i16>(&[i64::MAX as i16; 3], 1).unwrap();
	new_as_by::<i64, i32>(&[i64::MAX as i32; 3], 1).unwrap();
	new_as_by::<i64, i64>(&[i64::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_char_max_test() {
	new_as_by::<u8, u8>(&[char::MAX as u8; 3], 1).unwrap();

	new_as_by::<u16, u8>(&[char::MAX as u8; 3], 1).unwrap();
	new_as_by::<u16, u16>(&[char::MAX as u16; 3], 1).unwrap();

	new_as_by::<u32, u8>(&[char::MAX as u8; 3], 1).unwrap();
	new_as_by::<u32, u16>(&[char::MAX as u16; 3], 1).unwrap();
	new_as_by::<u32, u32>(&[char::MAX as u32; 3], 1).unwrap();

	new_as_by::<u64, u8>(&[char::MAX as u8; 3], 1).unwrap();
	new_as_by::<u64, u16>(&[char::MAX as u16; 3], 1).unwrap();
	new_as_by::<u64, u32>(&[char::MAX as u32; 3], 1).unwrap();
	new_as_by::<u64, u64>(&[char::MAX as u64; 3], 1).unwrap();

	new_as_by::<i8, i8>(&[char::MAX as i8; 3], 1).unwrap();

	new_as_by::<i16, u8>(&[char::MAX as u8; 3], 1).unwrap();
	new_as_by::<i16, i8>(&[char::MAX as i8; 3], 1).unwrap();
	new_as_by::<i16, i16>(&[char::MAX as i16; 3], 1).unwrap();

	new_as_by::<i32, u8>(&[char::MAX as u8; 3], 1).unwrap();
	new_as_by::<i32, u16>(&[char::MAX as u16; 3], 1).unwrap();
	new_as_by::<i32, i8>(&[char::MAX as i8; 3], 1).unwrap();
	new_as_by::<i32, i16>(&[char::MAX as i16; 3], 1).unwrap();
	new_as_by::<i32, i32>(&[char::MAX as i32; 3], 1).unwrap();

	new_as_by::<i64, u8>(&[char::MAX as u8; 3], 1).unwrap();
	new_as_by::<i64, u16>(&[char::MAX as u16; 3], 1).unwrap();
	new_as_by::<i64, u32>(&[char::MAX as u32; 3], 1).unwrap();
	new_as_by::<i64, i8>(&[char::MAX as i8; 3], 1).unwrap();
	new_as_by::<i64, i16>(&[char::MAX as i16; 3], 1).unwrap();
	new_as_by::<i64, i32>(&[char::MAX as i32; 3], 1).unwrap();
	new_as_by::<i64, i64>(&[char::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_minus_test() {
	new_as_by::<i8, i8>(&[-1; 3], 1).unwrap();
	new_as_by::<i8, i8>(&[i8::MIN; 3], 1).unwrap();

	new_as_by::<i16, i8>(&[-1; 3], 1).unwrap();
	new_as_by::<i16, i16>(&[-1; 3], 1).unwrap();
	new_as_by::<i16, i8>(&[i8::MIN; 3], 1).unwrap();
	new_as_by::<i16, i16>(&[i16::MIN; 3], 1).unwrap();

	new_as_by::<i32, i8>(&[-1; 3], 1).unwrap();
	new_as_by::<i32, i16>(&[-1; 3], 1).unwrap();
	new_as_by::<i32, i32>(&[-1; 3], 1).unwrap();
	new_as_by::<i32, i8>(&[i8::MIN; 3], 1).unwrap();
	new_as_by::<i32, i16>(&[i16::MIN; 3], 1).unwrap();
	new_as_by::<i32, i32>(&[i32::MIN; 3], 1).unwrap();

	new_as_by::<i64, i8>(&[-1; 3], 1).unwrap();
	new_as_by::<i64, i16>(&[-1; 3], 1).unwrap();
	new_as_by::<i64, i32>(&[-1; 3], 1).unwrap();
	new_as_by::<i64, i64>(&[-1; 3], 1).unwrap();
	new_as_by::<i64, i8>(&[i8::MIN; 3], 1).unwrap();
	new_as_by::<i64, i16>(&[i16::MIN; 3], 1).unwrap();
	new_as_by::<i64, i32>(&[i32::MIN; 3], 1).unwrap();
	new_as_by::<i64, i64>(&[i64::MIN; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u8_error_type_test() {
	let e = new_as_by_try_from::<u8, _>(&[0; 3], 3).unwrap_err();
	assert_out_of_range_index(&e);

	let e = new_as_by_try_from::<u8, u8>(&[], 0).unwrap_err();
	assert_out_of_range_index(&e);

	let e = new_as_by_try_from::<u8, _>(&[-1; 3], 1).unwrap_err();
	assert_try_from_int_error(&e);

	let e = new_as_by_try_from::<u8, _>(&[char::MAX; 3], 1).unwrap_err();
	assert_try_from_char_error(&e);
}

#[test]
fn new_as_by_try_from_u16_error_type_test() {
	let e = new_as_by_try_from::<u16, _>(&[0; 3], 3).unwrap_err();
	assert_out_of_range_index(&e);

	let e = new_as_by_try_from::<u16, u8>(&[], 0).unwrap_err();
	assert_out_of_range_index(&e);

	let e = new_as_by_try_from::<u16, _>(&[-1; 3], 1).unwrap_err();
	assert_try_from_int_error(&e);

	let e = new_as_by_try_from::<u16, _>(&[char::MAX; 3], 1).unwrap_err();
	assert_try_from_char_error(&e);
}

#[test]
fn new_as_by_try_from_u32_error_type_test() {
	let e = new_as_by_try_from::<u32, _>(&[0; 3], 3).unwrap_err();
	assert_out_of_range_index(&e);

	let e = new_as_by_try_from::<u32, u8>(&[], 0).unwrap_err();
	assert_out_of_range_index(&e);

	let e = new_as_by_try_from::<u32, _>(&[-1; 3], 1).unwrap_err();
	assert_try_from_int_error(&e);
}

#[test]
fn new_as_by_try_from_u64_error_type_test() {
	let e = new_as_by_try_from::<u64, _>(&[0; 3], 3).unwrap_err();
	assert_out_of_range_index(&e);

	let e = new_as_by_try_from::<u64, u8>(&[], 0).unwrap_err();
	assert_out_of_range_index(&e);

	let e = new_as_by_try_from::<u64, _>(&[-1; 3], 1).unwrap_err();
	assert_try_from_int_error(&e);
}

#[test]
fn new_as_by_try_from_i8_error_type_test() {
	let e = new_as_by_try_from::<i8, _>(&[0; 3], 3).unwrap_err();
	assert_out_of_range_index(&e);

	let e = new_as_by_try_from::<i8, u8>(&[], 0).unwrap_err();
	assert_out_of_range_index(&e);

	let e = new_as_by_try_from::<i8, _>(&[u64::MAX; 3], 1).unwrap_err();
	assert_try_from_int_error(&e);
}

#[test]
fn new_as_by_try_from_i16_error_type_test() {
	let e = new_as_by_try_from::<i16, _>(&[0; 3], 3).unwrap_err();
	assert_out_of_range_index(&e);

	let e = new_as_by_try_from::<i16, u8>(&[], 0).unwrap_err();
	assert_out_of_range_index(&e);

	let e = new_as_by_try_from::<i16, _>(&[u64::MAX; 3], 1).unwrap_err();
	assert_try_from_int_error(&e);
}

#[test]
fn new_as_by_try_from_i32_error_type_test() {
	let e = new_as_by_try_from::<i32, _>(&[0; 3], 3).unwrap_err();
	assert_out_of_range_index(&e);

	let e = new_as_by_try_from::<i32, u8>(&[], 0).unwrap_err();
	assert_out_of_range_index(&e);

	let e = new_as_by_try_from::<i32, _>(&[u64::MAX; 3], 1).unwrap_err();
	assert_try_from_int_error(&e);
}

#[test]
fn new_as_by_try_from_i64_error_type_test() {
	let e = new_as_by_try_from::<i64, _>(&[0; 3], 3).unwrap_err();
	assert_out_of_range_index(&e);

	let e = new_as_by_try_from::<i64, u8>(&[], 0).unwrap_err();
	assert_out_of_range_index(&e);

	let e = new_as_by_try_from::<i64, _>(&[u64::MAX; 3], 1).unwrap_err();
	assert_try_from_int_error(&e);
}

#[test]
fn new_as_by_try_from_u8_empty_test() {
	new_as_by_try_from::<u8, u8>(&[], 0).unwrap_err();
	new_as_by_try_from::<u8, u16>(&[], 0).unwrap_err();
	new_as_by_try_from::<u8, u32>(&[], 0).unwrap_err();
	new_as_by_try_from::<u8, u64>(&[], 0).unwrap_err();
	new_as_by_try_from::<u8, i8>(&[], 0).unwrap_err();
	new_as_by_try_from::<u8, i16>(&[], 0).unwrap_err();
	new_as_by_try_from::<u8, i32>(&[], 0).unwrap_err();
	new_as_by_try_from::<u8, i64>(&[], 0).unwrap_err();
	new_as_by_try_from::<u8, char>(&[], 0).unwrap_err();
	new_as_by_try_from::<u8, bool>(&[], 0).unwrap_err();
}

#[test]
fn new_as_by_try_from_u16_empty_test() {
	new_as_by_try_from::<u16, u8>(&[], 0).unwrap_err();
	new_as_by_try_from::<u16, u16>(&[], 0).unwrap_err();
	new_as_by_try_from::<u16, u32>(&[], 0).unwrap_err();
	new_as_by_try_from::<u16, u64>(&[], 0).unwrap_err();
	new_as_by_try_from::<u16, i8>(&[], 0).unwrap_err();
	new_as_by_try_from::<u16, i16>(&[], 0).unwrap_err();
	new_as_by_try_from::<u16, i32>(&[], 0).unwrap_err();
	new_as_by_try_from::<u16, i64>(&[], 0).unwrap_err();
	new_as_by_try_from::<u16, char>(&[], 0).unwrap_err();
	new_as_by_try_from::<u16, bool>(&[], 0).unwrap_err();
}

#[test]
fn new_as_by_try_from_u32_empty_test() {
	new_as_by_try_from::<u32, u8>(&[], 0).unwrap_err();
	new_as_by_try_from::<u32, u16>(&[], 0).unwrap_err();
	new_as_by_try_from::<u32, u32>(&[], 0).unwrap_err();
	new_as_by_try_from::<u32, u64>(&[], 0).unwrap_err();
	new_as_by_try_from::<u32, i8>(&[], 0).unwrap_err();
	new_as_by_try_from::<u32, i16>(&[], 0).unwrap_err();
	new_as_by_try_from::<u32, i32>(&[], 0).unwrap_err();
	new_as_by_try_from::<u32, i64>(&[], 0).unwrap_err();
	new_as_by_try_from::<u32, char>(&[], 0).unwrap_err();
	new_as_by_try_from::<u32, bool>(&[], 0).unwrap_err();
}

#[test]
fn new_as_by_try_from_u64_empty_test() {
	new_as_by_try_from::<u64, u8>(&[], 0).unwrap_err();
	new_as_by_try_from::<u64, u16>(&[], 0).unwrap_err();
	new_as_by_try_from::<u64, u32>(&[], 0).unwrap_err();
	new_as_by_try_from::<u64, u64>(&[], 0).unwrap_err();
	new_as_by_try_from::<u64, i8>(&[], 0).unwrap_err();
	new_as_by_try_from::<u64, i16>(&[], 0).unwrap_err();
	new_as_by_try_from::<u64, i32>(&[], 0).unwrap_err();
	new_as_by_try_from::<u64, i64>(&[], 0).unwrap_err();
	new_as_by_try_from::<u64, char>(&[], 0).unwrap_err();
	new_as_by_try_from::<u64, bool>(&[], 0).unwrap_err();
}

#[test]
fn new_as_by_try_from_i8_empty_test() {
	new_as_by_try_from::<i8, u8>(&[], 0).unwrap_err();
	new_as_by_try_from::<i8, u16>(&[], 0).unwrap_err();
	new_as_by_try_from::<i8, u32>(&[], 0).unwrap_err();
	new_as_by_try_from::<i8, u64>(&[], 0).unwrap_err();
	new_as_by_try_from::<i8, i8>(&[], 0).unwrap_err();
	new_as_by_try_from::<i8, i16>(&[], 0).unwrap_err();
	new_as_by_try_from::<i8, i32>(&[], 0).unwrap_err();
	new_as_by_try_from::<i8, i64>(&[], 0).unwrap_err();
	new_as_by_try_from::<i8, bool>(&[], 0).unwrap_err();
}

#[test]
fn new_as_by_try_from_i16_empty_test() {
	new_as_by_try_from::<i16, u8>(&[], 0).unwrap_err();
	new_as_by_try_from::<i16, u16>(&[], 0).unwrap_err();
	new_as_by_try_from::<i16, u32>(&[], 0).unwrap_err();
	new_as_by_try_from::<i16, u64>(&[], 0).unwrap_err();
	new_as_by_try_from::<i16, i8>(&[], 0).unwrap_err();
	new_as_by_try_from::<i16, i16>(&[], 0).unwrap_err();
	new_as_by_try_from::<i16, i32>(&[], 0).unwrap_err();
	new_as_by_try_from::<i16, i64>(&[], 0).unwrap_err();
	new_as_by_try_from::<i16, bool>(&[], 0).unwrap_err();
}

#[test]
fn new_as_by_try_from_i32_empty_test() {
	new_as_by_try_from::<i32, u8>(&[], 0).unwrap_err();
	new_as_by_try_from::<i32, u16>(&[], 0).unwrap_err();
	new_as_by_try_from::<i32, u32>(&[], 0).unwrap_err();
	new_as_by_try_from::<i32, u64>(&[], 0).unwrap_err();
	new_as_by_try_from::<i32, i8>(&[], 0).unwrap_err();
	new_as_by_try_from::<i32, i16>(&[], 0).unwrap_err();
	new_as_by_try_from::<i32, i32>(&[], 0).unwrap_err();
	new_as_by_try_from::<i32, i64>(&[], 0).unwrap_err();
	new_as_by_try_from::<i32, bool>(&[], 0).unwrap_err();
}

#[test]
fn new_as_by_try_from_i64_empty_test() {
	new_as_by_try_from::<i64, u8>(&[], 0).unwrap_err();
	new_as_by_try_from::<i64, u16>(&[], 0).unwrap_err();
	new_as_by_try_from::<i64, u32>(&[], 0).unwrap_err();
	new_as_by_try_from::<i64, u64>(&[], 0).unwrap_err();
	new_as_by_try_from::<i64, i8>(&[], 0).unwrap_err();
	new_as_by_try_from::<i64, i16>(&[], 0).unwrap_err();
	new_as_by_try_from::<i64, i32>(&[], 0).unwrap_err();
	new_as_by_try_from::<i64, i64>(&[], 0).unwrap_err();
	new_as_by_try_from::<i64, bool>(&[], 0).unwrap_err();
}

#[test]
fn new_as_by_try_from_u8_zero_test() {
	new_as_by_try_from::<u8, u8>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u8, u16>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u8, u32>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u8, u64>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u8, i8>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u8, i16>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u8, i32>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u8, i64>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u8, char>(&[char::from(0); 3], 1).unwrap();
	new_as_by_try_from::<u8, bool>(&[false; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u16_zero_test() {
	new_as_by_try_from::<u16, u8>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u16, u16>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u16, u32>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u16, u64>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u16, i8>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u16, i16>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u16, i32>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u16, i64>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u16, char>(&[char::from(0); 3], 1).unwrap();
	new_as_by_try_from::<u16, bool>(&[false; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u32_zero_test() {
	new_as_by_try_from::<u32, u8>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u32, u16>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u32, u32>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u32, u64>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u32, i8>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u32, i16>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u32, i32>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u32, i64>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u32, char>(&[char::from(0); 3], 1).unwrap();
	new_as_by_try_from::<u32, bool>(&[false; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u64_zero_test() {
	new_as_by_try_from::<u64, u8>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u64, u16>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u64, u32>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u64, u64>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u64, i8>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u64, i16>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u64, i32>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u64, i64>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<u64, char>(&[char::from(0); 3], 1).unwrap();
	new_as_by_try_from::<u64, bool>(&[false; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i8_zero_test() {
	new_as_by_try_from::<i8, u8>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i8, u16>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i8, u32>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i8, u64>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i8, i8>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i8, i16>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i8, i32>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i8, i64>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i8, bool>(&[false; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i16_zero_test() {
	new_as_by_try_from::<i16, u8>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i16, u16>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i16, u32>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i16, u64>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i16, i8>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i16, i16>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i16, i32>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i16, i64>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i16, bool>(&[false; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i32_zero_test() {
	new_as_by_try_from::<i32, u8>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i32, u16>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i32, u32>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i32, u64>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i32, i8>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i32, i16>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i32, i32>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i32, i64>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i32, bool>(&[false; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i64_zero_test() {
	new_as_by_try_from::<i64, u8>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i64, u16>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i64, u32>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i64, u64>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i64, i8>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i64, i16>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i64, i32>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i64, i64>(&[0; 3], 1).unwrap();
	new_as_by_try_from::<i64, bool>(&[false; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u8_one_test() {
	new_as_by_try_from::<u8, u8>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u8, u16>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u8, u32>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u8, u64>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u8, i8>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u8, i16>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u8, i32>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u8, i64>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u8, char>(&[char::from(1); 3], 1).unwrap();
	new_as_by_try_from::<u8, bool>(&[true; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u16_one_test() {
	new_as_by_try_from::<u16, u8>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u16, u16>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u16, u32>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u16, u64>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u16, i8>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u16, i16>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u16, i32>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u16, i64>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u16, char>(&[char::from(1); 3], 1).unwrap();
	new_as_by_try_from::<u16, bool>(&[true; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u32_one_test() {
	new_as_by_try_from::<u32, u8>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u32, u16>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u32, u32>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u32, u64>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u32, i8>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u32, i16>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u32, i32>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u32, i64>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u32, char>(&[char::from(1); 3], 1).unwrap();
	new_as_by_try_from::<u32, bool>(&[true; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u64_one_test() {
	new_as_by_try_from::<u64, u8>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u64, u16>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u64, u32>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u64, u64>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u64, i8>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u64, i16>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u64, i32>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u64, i64>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<u64, char>(&[char::from(1); 3], 1).unwrap();
	new_as_by_try_from::<u64, bool>(&[true; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i8_one_test() {
	new_as_by_try_from::<i8, u8>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i8, u16>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i8, u32>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i8, u64>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i8, i8>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i8, i16>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i8, i32>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i8, i64>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i8, bool>(&[true; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i16_one_test() {
	new_as_by_try_from::<i16, u8>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i16, u16>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i16, u32>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i16, u64>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i16, i8>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i16, i16>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i16, i32>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i16, i64>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i16, bool>(&[true; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i32_one_test() {
	new_as_by_try_from::<i32, u8>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i32, u16>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i32, u32>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i32, u64>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i32, i8>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i32, i16>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i32, i32>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i32, i64>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i32, bool>(&[true; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i64_one_test() {
	new_as_by_try_from::<i64, u8>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i64, u16>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i64, u32>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i64, u64>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i64, i8>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i64, i16>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i64, i32>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i64, i64>(&[1; 3], 1).unwrap();
	new_as_by_try_from::<i64, bool>(&[true; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u8_u8_max_test() {
	new_as_by_try_from::<u8, u8>(&[u8::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u8, u16>(&[u8::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u8, u32>(&[u8::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u8, u64>(&[u8::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u8, i8>(&[u8::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i16>(&[u8::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<u8, i32>(&[u8::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u8, i64>(&[u8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u8_u16_max_test() {
	new_as_by_try_from::<u8, u8>(&[u16::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u8, u16>(&[u16::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, u32>(&[u16::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, u64>(&[u16::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i8>(&[u16::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i16>(&[u16::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i32>(&[u16::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i64>(&[u16::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u8_u32_max_test() {
	new_as_by_try_from::<u8, u8>(&[u32::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u8, u16>(&[u32::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, u32>(&[u32::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, u64>(&[u32::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i8>(&[u32::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i16>(&[u32::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i32>(&[u32::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i64>(&[u32::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u8_u64_max_test() {
	new_as_by_try_from::<u8, u8>(&[u64::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u8, u16>(&[u64::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, u32>(&[u64::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, u64>(&[u64::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i8>(&[u64::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i16>(&[u64::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i32>(&[u64::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i64>(&[u64::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u8_i8_max_test() {
	new_as_by_try_from::<u8, u8>(&[i8::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u8, u16>(&[i8::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u8, u32>(&[i8::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u8, u64>(&[i8::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u8, i8>(&[i8::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<u8, i16>(&[i8::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<u8, i32>(&[i8::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u8, i64>(&[i8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u8_i16_max_test() {
	new_as_by_try_from::<u8, u8>(&[i16::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u8, u16>(&[i16::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, u32>(&[i16::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, u64>(&[i16::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i8>(&[i16::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i16>(&[i16::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i32>(&[i16::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i64>(&[i16::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u8_i32_max_test() {
	new_as_by_try_from::<u8, u8>(&[i32::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u8, u16>(&[i32::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, u32>(&[i32::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, u64>(&[i32::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i8>(&[i32::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i16>(&[i32::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i32>(&[i32::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i64>(&[i32::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u8_i64_max_test() {
	new_as_by_try_from::<u8, u8>(&[i64::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u8, u16>(&[i64::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, u32>(&[i64::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, u64>(&[i64::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i8>(&[i64::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i16>(&[i64::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i32>(&[i64::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i64>(&[i64::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u8_char_max_test() {
	new_as_by_try_from::<u8, u8>(&[char::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u8, u16>(&[char::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, u32>(&[char::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, u64>(&[char::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i8>(&[char::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i16>(&[char::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i32>(&[char::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i64>(&[char::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u16_u8_max_test() {
	new_as_by_try_from::<u16, u8>(&[u8::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u16, u16>(&[u8::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u16, u32>(&[u8::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u16, u64>(&[u8::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u16, i8>(&[u8::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i16>(&[u8::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<u16, i32>(&[u8::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u16, i64>(&[u8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u16_u16_max_test() {
	new_as_by_try_from::<u16, u8>(&[u16::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u16, u16>(&[u16::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u16, u32>(&[u16::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u16, u64>(&[u16::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u16, i8>(&[u16::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i16>(&[u16::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i32>(&[u16::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u16, i64>(&[u16::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u16_u32_max_test() {
	new_as_by_try_from::<u16, u8>(&[u32::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u16, u16>(&[u32::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u16, u32>(&[u32::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, u64>(&[u32::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i8>(&[u32::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i16>(&[u32::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i32>(&[u32::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i64>(&[u32::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u16_u64_max_test() {
	new_as_by_try_from::<u16, u8>(&[u64::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u16, u16>(&[u64::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u16, u32>(&[u64::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, u64>(&[u64::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i8>(&[u64::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i16>(&[u64::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i32>(&[u64::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i64>(&[u64::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u16_i8_max_test() {
	new_as_by_try_from::<u16, u8>(&[i8::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u16, u16>(&[i8::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u16, u32>(&[i8::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u16, u64>(&[i8::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u16, i8>(&[i8::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<u16, i16>(&[i8::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<u16, i32>(&[i8::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u16, i64>(&[i8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u16_i16_max_test() {
	new_as_by_try_from::<u16, u8>(&[i16::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u16, u16>(&[i16::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u16, u32>(&[i16::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u16, u64>(&[i16::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u16, i8>(&[i16::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i16>(&[i16::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<u16, i32>(&[i16::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u16, i64>(&[i16::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u16_i32_max_test() {
	new_as_by_try_from::<u16, u8>(&[i32::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u16, u16>(&[i32::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u16, u32>(&[i32::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, u64>(&[i32::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i8>(&[i32::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i16>(&[i32::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i32>(&[i32::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i64>(&[i32::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u16_i64_max_test() {
	new_as_by_try_from::<u16, u8>(&[i64::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u16, u16>(&[i64::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u16, u32>(&[i64::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, u64>(&[i64::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i8>(&[i64::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i16>(&[i64::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i32>(&[i64::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i64>(&[i64::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u16_char_max_test() {
	new_as_by_try_from::<u16, u8>(&[char::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u16, u16>(&[char::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u16, u32>(&[char::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, u64>(&[char::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i8>(&[char::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i16>(&[char::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i32>(&[char::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i64>(&[char::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u32_u8_max_test() {
	new_as_by_try_from::<u32, u8>(&[u8::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u32, u16>(&[u8::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u32, u32>(&[u8::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u32, u64>(&[u8::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u32, i8>(&[u8::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i16>(&[u8::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<u32, i32>(&[u8::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u32, i64>(&[u8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u32_u16_max_test() {
	new_as_by_try_from::<u32, u8>(&[u16::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u32, u16>(&[u16::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u32, u32>(&[u16::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u32, u64>(&[u16::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u32, i8>(&[u16::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i16>(&[u16::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i32>(&[u16::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u32, i64>(&[u16::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u32_u32_max_test() {
	new_as_by_try_from::<u32, u8>(&[u32::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u32, u16>(&[u32::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u32, u32>(&[u32::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u32, u64>(&[u32::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u32, i8>(&[u32::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i16>(&[u32::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i32>(&[u32::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i64>(&[u32::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u32_u64_max_test() {
	new_as_by_try_from::<u32, u8>(&[u64::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u32, u16>(&[u64::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u32, u32>(&[u64::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u32, u64>(&[u64::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i8>(&[u64::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i16>(&[u64::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i32>(&[u64::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i64>(&[u64::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u32_i8_max_test() {
	new_as_by_try_from::<u32, u8>(&[i8::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u32, u16>(&[i8::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u32, u32>(&[i8::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u32, u64>(&[i8::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u32, i8>(&[i8::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<u32, i16>(&[i8::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<u32, i32>(&[i8::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u32, i64>(&[i8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u32_i16_max_test() {
	new_as_by_try_from::<u32, u8>(&[i16::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u32, u16>(&[i16::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u32, u32>(&[i16::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u32, u64>(&[i16::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u32, i8>(&[i16::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i16>(&[i16::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<u32, i32>(&[i16::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u32, i64>(&[i16::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u32_i32_max_test() {
	new_as_by_try_from::<u32, u8>(&[i32::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u32, u16>(&[i32::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u32, u32>(&[i32::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u32, u64>(&[i32::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u32, i8>(&[i32::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i16>(&[i32::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i32>(&[i32::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u32, i64>(&[i32::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u32_i64_max_test() {
	new_as_by_try_from::<u32, u8>(&[i64::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u32, u16>(&[i64::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u32, u32>(&[i64::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u32, u64>(&[i64::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i8>(&[i64::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i16>(&[i64::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i32>(&[i64::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i64>(&[i64::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u32_char_max_test() {
	new_as_by_try_from::<u32, u8>(&[char::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u32, u16>(&[char::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u32, u32>(&[char::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u32, u64>(&[char::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u32, i8>(&[char::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i16>(&[char::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i32>(&[char::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u32, i64>(&[char::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u64_u8_max_test() {
	new_as_by_try_from::<u64, u8>(&[u8::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u64, u16>(&[u8::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u64, u32>(&[u8::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u64, u64>(&[u8::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u64, i8>(&[u8::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i16>(&[u8::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<u64, i32>(&[u8::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u64, i64>(&[u8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u64_u16_max_test() {
	new_as_by_try_from::<u64, u8>(&[u16::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u64, u16>(&[u16::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u64, u32>(&[u16::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u64, u64>(&[u16::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u64, i8>(&[u16::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i16>(&[u16::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i32>(&[u16::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u64, i64>(&[u16::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u64_u32_max_test() {
	new_as_by_try_from::<u64, u8>(&[u32::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u64, u16>(&[u32::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u64, u32>(&[u32::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u64, u64>(&[u32::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u64, i8>(&[u32::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i16>(&[u32::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i32>(&[u32::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i64>(&[u32::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u64_u64_max_test() {
	new_as_by_try_from::<u64, u8>(&[u64::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u64, u16>(&[u64::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u64, u32>(&[u64::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u64, u64>(&[u64::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u64, i8>(&[u64::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i16>(&[u64::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i32>(&[u64::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i64>(&[u64::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u64_i8_max_test() {
	new_as_by_try_from::<u64, u8>(&[i8::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u64, u16>(&[i8::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u64, u32>(&[i8::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u64, u64>(&[i8::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u64, i8>(&[i8::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<u64, i16>(&[i8::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<u64, i32>(&[i8::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u64, i64>(&[i8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u64_i16_max_test() {
	new_as_by_try_from::<u64, u8>(&[i16::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u64, u16>(&[i16::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u64, u32>(&[i16::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u64, u64>(&[i16::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u64, i8>(&[i16::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i16>(&[i16::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<u64, i32>(&[i16::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u64, i64>(&[i16::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u64_i32_max_test() {
	new_as_by_try_from::<u64, u8>(&[i32::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u64, u16>(&[i32::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u64, u32>(&[i32::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u64, u64>(&[i32::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u64, i8>(&[i32::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i16>(&[i32::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i32>(&[i32::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u64, i64>(&[i32::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u64_i64_max_test() {
	new_as_by_try_from::<u64, u8>(&[i64::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u64, u16>(&[i64::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u64, u32>(&[i64::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u64, u64>(&[i64::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u64, i8>(&[i64::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i16>(&[i64::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i32>(&[i64::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i64>(&[i64::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u64_char_max_test() {
	new_as_by_try_from::<u64, u8>(&[char::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<u64, u16>(&[char::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<u64, u32>(&[char::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<u64, u64>(&[char::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<u64, i8>(&[char::MAX as i8; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i16>(&[char::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i32>(&[char::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<u64, i64>(&[char::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i8_u8_max_test() {
	new_as_by_try_from::<i8, u8>(&[u8::MAX as u8; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u16>(&[u8::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u32>(&[u8::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u64>(&[u8::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i8>(&[u8::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i8, i16>(&[u8::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i32>(&[u8::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i64>(&[u8::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i8_u16_max_test() {
	new_as_by_try_from::<i8, u8>(&[u16::MAX as u8; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u16>(&[u16::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u32>(&[u16::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u64>(&[u16::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i8>(&[u16::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i8, i16>(&[u16::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i8, i32>(&[u16::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i64>(&[u16::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i8_u32_max_test() {
	new_as_by_try_from::<i8, u8>(&[u32::MAX as u8; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u16>(&[u32::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u32>(&[u32::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u64>(&[u32::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i8>(&[u32::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i8, i16>(&[u32::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i8, i32>(&[u32::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i8, i64>(&[u32::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i8_u64_max_test() {
	new_as_by_try_from::<i8, u8>(&[u64::MAX as u8; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u16>(&[u64::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u32>(&[u64::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u64>(&[u64::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i8>(&[u64::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i8, i16>(&[u64::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i8, i32>(&[u64::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i8, i64>(&[u64::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i8_i8_max_test() {
	new_as_by_try_from::<i8, u8>(&[i8::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i8, u16>(&[i8::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i8, u32>(&[i8::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i8, u64>(&[i8::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i8, i8>(&[i8::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i8, i16>(&[i8::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i8, i32>(&[i8::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i8, i64>(&[i8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i8_i16_max_test() {
	new_as_by_try_from::<i8, u8>(&[i16::MAX as u8; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u16>(&[i16::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u32>(&[i16::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u64>(&[i16::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i8>(&[i16::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i8, i16>(&[i16::MAX as i16; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i32>(&[i16::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i64>(&[i16::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i8_i32_max_test() {
	new_as_by_try_from::<i8, u8>(&[i32::MAX as u8; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u16>(&[i32::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u32>(&[i32::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u64>(&[i32::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i8>(&[i32::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i8, i16>(&[i32::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i8, i32>(&[i32::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i64>(&[i32::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i8_i64_max_test() {
	new_as_by_try_from::<i8, u8>(&[i64::MAX as u8; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u16>(&[i64::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u32>(&[i64::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u64>(&[i64::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i8>(&[i64::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i8, i16>(&[i64::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i8, i32>(&[i64::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i8, i64>(&[i64::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i8_char_max_test() {
	new_as_by_try_from::<i8, u8>(&[char::MAX as u8; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u16>(&[char::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u32>(&[char::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, u64>(&[char::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i8>(&[char::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i8, i16>(&[char::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i8, i32>(&[char::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i64>(&[char::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i16_u8_max_test() {
	new_as_by_try_from::<i16, u8>(&[u8::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i16, u16>(&[u8::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i16, u32>(&[u8::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i16, u64>(&[u8::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i16, i8>(&[u8::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i16, i16>(&[u8::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i16, i32>(&[u8::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i16, i64>(&[u8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i16_u16_max_test() {
	new_as_by_try_from::<i16, u8>(&[u16::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i16, u16>(&[u16::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, u32>(&[u16::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, u64>(&[u16::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, i8>(&[u16::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i16, i16>(&[u16::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i16, i32>(&[u16::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, i64>(&[u16::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i16_u32_max_test() {
	new_as_by_try_from::<i16, u8>(&[u32::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i16, u16>(&[u32::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, u32>(&[u32::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, u64>(&[u32::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, i8>(&[u32::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i16, i16>(&[u32::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i16, i32>(&[u32::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i16, i64>(&[u32::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i16_u64_max_test() {
	new_as_by_try_from::<i16, u8>(&[u64::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i16, u16>(&[u64::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, u32>(&[u64::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, u64>(&[u64::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, i8>(&[u64::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i16, i16>(&[u64::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i16, i32>(&[u64::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i16, i64>(&[u64::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i16_i8_max_test() {
	new_as_by_try_from::<i16, u8>(&[i8::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i16, u16>(&[i8::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i16, u32>(&[i8::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i16, u64>(&[i8::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i16, i8>(&[i8::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i16, i16>(&[i8::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i16, i32>(&[i8::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i16, i64>(&[i8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i16_i16_max_test() {
	new_as_by_try_from::<i16, u8>(&[i16::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i16, u16>(&[i16::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i16, u32>(&[i16::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i16, u64>(&[i16::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i16, i8>(&[i16::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i16, i16>(&[i16::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i16, i32>(&[i16::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i16, i64>(&[i16::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i16_i32_max_test() {
	new_as_by_try_from::<i16, u8>(&[i32::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i16, u16>(&[i32::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, u32>(&[i32::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, u64>(&[i32::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, i8>(&[i32::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i16, i16>(&[i32::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i16, i32>(&[i32::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, i64>(&[i32::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i16_i64_max_test() {
	new_as_by_try_from::<i16, u8>(&[i64::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i16, u16>(&[i64::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, u32>(&[i64::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, u64>(&[i64::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, i8>(&[i64::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i16, i16>(&[i64::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i16, i32>(&[i64::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i16, i64>(&[i64::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i16_char_max_test() {
	new_as_by_try_from::<i16, u8>(&[char::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i16, u16>(&[char::MAX as u16; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, u32>(&[char::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, u64>(&[char::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, i8>(&[char::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i16, i16>(&[char::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i16, i32>(&[char::MAX as i32; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, i64>(&[char::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i32_u8_max_test() {
	new_as_by_try_from::<i32, u8>(&[u8::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i32, u16>(&[u8::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i32, u32>(&[u8::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i32, u64>(&[u8::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i32, i8>(&[u8::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i32, i16>(&[u8::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i32, i32>(&[u8::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i32, i64>(&[u8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i32_u16_max_test() {
	new_as_by_try_from::<i32, u8>(&[u16::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i32, u16>(&[u16::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i32, u32>(&[u16::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i32, u64>(&[u16::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i32, i8>(&[u16::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i32, i16>(&[u16::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i32, i32>(&[u16::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i32, i64>(&[u16::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i32_u32_max_test() {
	new_as_by_try_from::<i32, u8>(&[u32::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i32, u16>(&[u32::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i32, u32>(&[u32::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i32, u64>(&[u32::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i32, i8>(&[u32::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i32, i16>(&[u32::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i32, i32>(&[u32::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i32, i64>(&[u32::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i32_u64_max_test() {
	new_as_by_try_from::<i32, u8>(&[u64::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i32, u16>(&[u64::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i32, u32>(&[u64::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i32, u64>(&[u64::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i32, i8>(&[u64::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i32, i16>(&[u64::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i32, i32>(&[u64::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i32, i64>(&[u64::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i32_i8_max_test() {
	new_as_by_try_from::<i32, u8>(&[i8::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i32, u16>(&[i8::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i32, u32>(&[i8::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i32, u64>(&[i8::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i32, i8>(&[i8::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i32, i16>(&[i8::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i32, i32>(&[i8::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i32, i64>(&[i8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i32_i16_max_test() {
	new_as_by_try_from::<i32, u8>(&[i16::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i32, u16>(&[i16::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i32, u32>(&[i16::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i32, u64>(&[i16::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i32, i8>(&[i16::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i32, i16>(&[i16::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i32, i32>(&[i16::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i32, i64>(&[i16::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i32_i32_max_test() {
	new_as_by_try_from::<i32, u8>(&[i32::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i32, u16>(&[i32::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i32, u32>(&[i32::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i32, u64>(&[i32::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i32, i8>(&[i32::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i32, i16>(&[i32::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i32, i32>(&[i32::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i32, i64>(&[i32::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i32_i64_max_test() {
	new_as_by_try_from::<i32, u8>(&[i64::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i32, u16>(&[i64::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i32, u32>(&[i64::MAX as u32; 3], 1).unwrap_err();
	new_as_by_try_from::<i32, u64>(&[i64::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i32, i8>(&[i64::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i32, i16>(&[i64::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i32, i32>(&[i64::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i32, i64>(&[i64::MAX as i64; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i32_char_max_test() {
	new_as_by_try_from::<i32, u8>(&[char::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i32, u16>(&[char::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i32, u32>(&[char::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i32, u64>(&[char::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i32, i8>(&[char::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i32, i16>(&[char::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i32, i32>(&[char::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i32, i64>(&[char::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i64_u8_max_test() {
	new_as_by_try_from::<i64, u8>(&[u8::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i64, u16>(&[u8::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i64, u32>(&[u8::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i64, u64>(&[u8::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i64, i8>(&[u8::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i64, i16>(&[u8::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i64, i32>(&[u8::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i64, i64>(&[u8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i64_u16_max_test() {
	new_as_by_try_from::<i64, u8>(&[u16::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i64, u16>(&[u16::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i64, u32>(&[u16::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i64, u64>(&[u16::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i64, i8>(&[u16::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i64, i16>(&[u16::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i64, i32>(&[u16::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i64, i64>(&[u16::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i64_u32_max_test() {
	new_as_by_try_from::<i64, u8>(&[u32::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i64, u16>(&[u32::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i64, u32>(&[u32::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i64, u64>(&[u32::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i64, i8>(&[u32::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i64, i16>(&[u32::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i64, i32>(&[u32::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i64, i64>(&[u32::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i64_u64_max_test() {
	new_as_by_try_from::<i64, u8>(&[u64::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i64, u16>(&[u64::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i64, u32>(&[u64::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i64, u64>(&[u64::MAX as u64; 3], 1).unwrap_err();
	new_as_by_try_from::<i64, i8>(&[u64::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i64, i16>(&[u64::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i64, i32>(&[u64::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i64, i64>(&[u64::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i64_i8_max_test() {
	new_as_by_try_from::<i64, u8>(&[i8::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i64, u16>(&[i8::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i64, u32>(&[i8::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i64, u64>(&[i8::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i64, i8>(&[i8::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i64, i16>(&[i8::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i64, i32>(&[i8::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i64, i64>(&[i8::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i64_i16_max_test() {
	new_as_by_try_from::<i64, u8>(&[i16::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i64, u16>(&[i16::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i64, u32>(&[i16::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i64, u64>(&[i16::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i64, i8>(&[i16::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i64, i16>(&[i16::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i64, i32>(&[i16::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i64, i64>(&[i16::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i64_i32_max_test() {
	new_as_by_try_from::<i64, u8>(&[i32::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i64, u16>(&[i32::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i64, u32>(&[i32::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i64, u64>(&[i32::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i64, i8>(&[i32::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i64, i16>(&[i32::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i64, i32>(&[i32::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i64, i64>(&[i32::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i64_i64_max_test() {
	new_as_by_try_from::<i64, u8>(&[i64::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i64, u16>(&[i64::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i64, u32>(&[i64::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i64, u64>(&[i64::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i64, i8>(&[i64::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i64, i16>(&[i64::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i64, i32>(&[i64::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i64, i64>(&[i64::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_i64_char_max_test() {
	new_as_by_try_from::<i64, u8>(&[char::MAX as u8; 3], 1).unwrap();
	new_as_by_try_from::<i64, u16>(&[char::MAX as u16; 3], 1).unwrap();
	new_as_by_try_from::<i64, u32>(&[char::MAX as u32; 3], 1).unwrap();
	new_as_by_try_from::<i64, u64>(&[char::MAX as u64; 3], 1).unwrap();
	new_as_by_try_from::<i64, i8>(&[char::MAX as i8; 3], 1).unwrap();
	new_as_by_try_from::<i64, i16>(&[char::MAX as i16; 3], 1).unwrap();
	new_as_by_try_from::<i64, i32>(&[char::MAX as i32; 3], 1).unwrap();
	new_as_by_try_from::<i64, i64>(&[char::MAX as i64; 3], 1).unwrap();
}

#[test]
fn new_as_by_try_from_u8_minus_test() {
	new_as_by_try_from::<u8, i8>(&[-1; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i16>(&[-1; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i32>(&[-1; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i64>(&[-1; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i8>(&[i8::MIN; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i16>(&[i16::MIN; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i32>(&[i32::MIN; 3], 1).unwrap_err();
	new_as_by_try_from::<u8, i64>(&[i64::MIN; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u16_minus_test() {
	new_as_by_try_from::<u16, i8>(&[-1; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i16>(&[-1; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i32>(&[-1; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i64>(&[-1; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i8>(&[i8::MIN; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i16>(&[i16::MIN; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i32>(&[i32::MIN; 3], 1).unwrap_err();
	new_as_by_try_from::<u16, i64>(&[i64::MIN; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u32_minus_test() {
	new_as_by_try_from::<u32, i8>(&[-1; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i16>(&[-1; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i32>(&[-1; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i64>(&[-1; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i8>(&[i8::MIN; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i16>(&[i16::MIN; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i32>(&[i32::MIN; 3], 1).unwrap_err();
	new_as_by_try_from::<u32, i64>(&[i64::MIN; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_u64_minus_test() {
	new_as_by_try_from::<u64, i8>(&[-1; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i16>(&[-1; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i32>(&[-1; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i64>(&[-1; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i8>(&[i8::MIN; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i16>(&[i16::MIN; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i32>(&[i32::MIN; 3], 1).unwrap_err();
	new_as_by_try_from::<u64, i64>(&[i64::MIN; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i8_minus_test() {
	new_as_by_try_from::<i8, i8>(&[-1; 3], 1).unwrap();
	new_as_by_try_from::<i8, i16>(&[-1; 3], 1).unwrap();
	new_as_by_try_from::<i8, i32>(&[-1; 3], 1).unwrap();
	new_as_by_try_from::<i8, i64>(&[-1; 3], 1).unwrap();
	new_as_by_try_from::<i8, i8>(&[i8::MIN; 3], 1).unwrap();
	new_as_by_try_from::<i8, i16>(&[i16::MIN; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i32>(&[i32::MIN; 3], 1).unwrap_err();
	new_as_by_try_from::<i8, i64>(&[i64::MIN; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i16_minus_test() {
	new_as_by_try_from::<i16, i8>(&[-1; 3], 1).unwrap();
	new_as_by_try_from::<i16, i16>(&[-1; 3], 1).unwrap();
	new_as_by_try_from::<i16, i32>(&[-1; 3], 1).unwrap();
	new_as_by_try_from::<i16, i64>(&[-1; 3], 1).unwrap();
	new_as_by_try_from::<i16, i8>(&[i8::MIN; 3], 1).unwrap();
	new_as_by_try_from::<i16, i16>(&[i16::MIN; 3], 1).unwrap();
	new_as_by_try_from::<i16, i32>(&[i32::MIN; 3], 1).unwrap_err();
	new_as_by_try_from::<i16, i64>(&[i64::MIN; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i32_minus_test() {
	new_as_by_try_from::<i32, i8>(&[-1; 3], 1).unwrap();
	new_as_by_try_from::<i32, i16>(&[-1; 3], 1).unwrap();
	new_as_by_try_from::<i32, i32>(&[-1; 3], 1).unwrap();
	new_as_by_try_from::<i32, i64>(&[-1; 3], 1).unwrap();
	new_as_by_try_from::<i32, i8>(&[i8::MIN; 3], 1).unwrap();
	new_as_by_try_from::<i32, i16>(&[i16::MIN; 3], 1).unwrap();
	new_as_by_try_from::<i32, i32>(&[i32::MIN; 3], 1).unwrap();
	new_as_by_try_from::<i32, i64>(&[i64::MIN; 3], 1).unwrap_err();
}

#[test]
fn new_as_by_try_from_i64_minus_test() {
	new_as_by_try_from::<i64, i8>(&[-1; 3], 1).unwrap();
	new_as_by_try_from::<i64, i16>(&[-1; 3], 1).unwrap();
	new_as_by_try_from::<i64, i32>(&[-1; 3], 1).unwrap();
	new_as_by_try_from::<i64, i64>(&[-1; 3], 1).unwrap();
	new_as_by_try_from::<i64, i8>(&[i8::MIN; 3], 1).unwrap();
	new_as_by_try_from::<i64, i16>(&[i16::MIN; 3], 1).unwrap();
	new_as_by_try_from::<i64, i32>(&[i32::MIN; 3], 1).unwrap();
	new_as_by_try_from::<i64, i64>(&[i64::MIN; 3], 1).unwrap();
}

#[test]
fn convert_test() {
	assert_eq!(convert::<u8, _>(&[0; 3]), [0; 3]);
}

#[test]
fn convert_try_from_test() {
	assert_eq!(convert_try_from::<u8, _>(&[0; 3]).unwrap(), [0; 3]);

	let e = convert_try_from::<u8, _>(&[-1; 3]).unwrap_err();
	assert_try_from_int_error(&e);

	let e = convert_try_from::<u8, _>(&[!0u64; 3]).unwrap_err();
	assert_try_from_int_error(&e);

	let e = convert_try_from::<u8, _>(&[char::MAX; 3]).unwrap_err();
	assert_try_from_char_error(&e);
}

#[test]
fn compare_test() {
	assert_eq!(compare::<u8, _>(&[1, 1, 1], &[2, 1, 0]), CompareState::Concurrent);
	assert_eq!(compare::<u8, _>(&[1, 1, 1], &[1, 1, 0]), CompareState::After);
	assert_eq!(compare::<u8, _>(&[1, 1, 1], &[1, 1, 1]), CompareState::Same);
	assert_eq!(compare::<u8, _>(&[1, 1, 1], &[1, 1, 2]), CompareState::Before);
	assert_eq!(compare::<u8, _>(&[1, 1, 1], &[0, 1, 2]), CompareState::Concurrent);
}

#[test]
fn compare_try_from_test() {
	assert_eq!(compare_try_from::<u8, _>(&[1, 1, 1], &[2, 1, 0]).unwrap(), CompareState::Concurrent);
	assert_eq!(compare_try_from::<u8, _>(&[1, 1, 1], &[1, 1, 0]).unwrap(), CompareState::After);
	assert_eq!(compare_try_from::<u8, _>(&[1, 1, 1], &[1, 1, 1]).unwrap(), CompareState::Same);
	assert_eq!(compare_try_from::<u8, _>(&[1, 1, 1], &[1, 1, 2]).unwrap(), CompareState::Before);
	assert_eq!(compare_try_from::<u8, _>(&[1, 1, 1], &[0, 1, 2]).unwrap(), CompareState::Concurrent);

	let e = compare_try_from::<u8, _>(&[1, 1, 1], &[-1; 3]).unwrap_err();
	assert_try_from_int_error(&e);

	let e = compare_try_from::<u8, _>(&[1, 1, 1], &[!0u64; 3]).unwrap_err();
	assert_try_from_int_error(&e);

	let e = compare_try_from::<u8, _>(&[1, 1, 1], &[char::MAX; 3]).unwrap_err();
	assert_try_from_char_error(&e);
}

fn assert_out_of_range_index(e: &Error) {
	match e {
		Error::OutOfRangeIndex => (),
		_ => panic!("assertion failed: assert_out_of_range_index({:?})", e),
	}
}

fn assert_unmatch_time_size(e: &Error) {
	match e {
		Error::UnmatchTimeSize => (),
		_ => panic!("assertion failed: assert_unmatch_time_size({:?})", e),
	}
}

fn assert_invalid_time_value(e: &Error) {
	match e {
		Error::InvalidTimeValue => (),
		_ => panic!("assertion failed: assert_invalid_time_value({:?})", e),
	}
}

fn assert_try_from_int_error(e: &Error) {
	match e {
		Error::ExternalError(e) => {
			let e0 = format!("{:?}", e);
			let e1 = format!("{:?}", u8::try_from(-1).unwrap_err());
			assert_eq!(e0, e1)
		}
		_ => panic!("assertion failed: assert_try_from_int_error({:?})", e),
	}
}

fn assert_try_from_char_error(e: &Error) {
	match e {
		Error::ExternalError(e) => {
			let e0 = format!("{:?}", e);
			let e1 = format!("{:?}", u8::try_from(char::MAX).unwrap_err());
			assert_eq!(e0, e1)
		}
		_ => panic!("assertion failed: assert_try_from_char_error({:?})", e),
	}
}

