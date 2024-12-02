use crate::*;

pub fn new(n: usize, self_index: usize) -> Result<Clock<u64>>
{
	new_as::<u64>(n, self_index)
}

pub fn new_by<U, V>(initial: V, self_index: usize) -> Result<Clock<u64>>
where u64: From<U>, U: Copy, V: AsRef<[U]>,
{
	new_as_by::<u64, U, V>(initial, self_index)
}

pub fn new_by_try_from<U, V>(initial: V, self_index: usize) -> Result<Clock<u64>>
where
	u64: TryFrom<U>, U: Copy, V: AsRef<[U]>,
	Error: From<<u64 as TryFrom<U>>::Error>,
{
	new_as_by_try_from::<u64, U, V>(initial, self_index)
}

pub fn new_as<T>(n: usize, self_index: usize) -> Result<Clock<T>>
where T: Copy + Ord + From<bool> + From<T> + std::ops::AddAssign,
{
	new_as_by::<T, bool, _>(&vec![false; n], self_index)
}

pub fn new_as_by<T, U, V>(initial: V, self_index: usize) -> Result<Clock<T>>
where
	T: Copy + Ord + From<bool> + From<T> + From<U> + std::ops::AddAssign,
	U: Copy, V: AsRef<[U]>,
{
	if self_index >= initial.as_ref().len() {
		Err(Error::OutOfRangeIndex)
	} else {
		let time: Vec<T> = convert(initial);
		Ok(Clock {time, self_index})
	}
}

pub fn new_as_by_try_from<T, U, V>(initial: V, self_index: usize) -> Result<Clock<T>>
where
	T: Copy + Ord + From<bool> + From<T> + TryFrom<U> + std::ops::AddAssign,
	U: Copy, V: AsRef<[U]>,
	Error: From<<T as TryFrom<U>>::Error>,
{
	if self_index >= initial.as_ref().len() {
		Err(Error::OutOfRangeIndex)
	} else {
		let time: Vec<T> = convert_try_from(initial)?;
		Ok(Clock {time, self_index})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_func::*;

	#[test]
	fn new_test() {
		new(3, 0).unwrap();

		let e = new(3, 3).unwrap_err();
		assert_out_of_range_index(&e);

		let e = new(0, 0).unwrap_err();
		assert_out_of_range_index(&e);
	}

	#[test]
	fn new_by_test() {
		new_by(&[0u8; 3], 0).unwrap();

		let e = new_by(&[0u8; 3], 3).unwrap_err();
		assert_out_of_range_index(&e);

		let e = new_by::<u8, _>(&[], 0).unwrap_err();
		assert_out_of_range_index(&e);
	}

	#[test]
	fn new_by_try_from_test() {
		new_by_try_from(&[0; 3], 0).unwrap();

		let e = new_by_try_from(&[0; 3], 3).unwrap_err();
		assert_out_of_range_index(&e);

		let e = new_by_try_from::<u8, _>(&[], 0).unwrap_err();
		assert_out_of_range_index(&e);

		let e = new_by_try_from(&[-1; 3], 0).unwrap_err();
		assert_try_from_int_error(&e);
	}

	#[test]
	fn new_as_test() {
		new_as::<u8>(3, 0).unwrap();

		let e = new_as::<u8>(3, 3).unwrap_err();
		assert_out_of_range_index(&e);

		let e = new_as::<u8>(0, 0).unwrap_err();
		assert_out_of_range_index(&e);
	}

	#[test]
	fn new_as_by_test() {
		new_as_by::<u8, _, _>(&[0; 3], 0).unwrap();

		let e = new_as_by::<u8, _, _>(&[0; 3], 3).unwrap_err();
		assert_out_of_range_index(&e);

		let e = new_as_by::<u8, u8, _>(&[], 0).unwrap_err();
		assert_out_of_range_index(&e);
	}

	#[test]
	fn new_as_by_try_from_test() {
		new_as_by_try_from::<u8, _, _>(&[0; 3], 0).unwrap();

		let e = new_as_by_try_from::<u8, _, _>(&[0; 3], 3).unwrap_err();
		assert_out_of_range_index(&e);

		let e = new_as_by_try_from::<u8, u8, _>(&[], 0).unwrap_err();
		assert_out_of_range_index(&e);

		let e = new_as_by_try_from::<u8, _, _>(&[-1; 3], 0).unwrap_err();
		assert_try_from_int_error(&e);

		let e = new_as_by_try_from::<u8, _, _>(&[!0u64; 3], 0).unwrap_err();
		assert_try_from_int_error(&e);

		let e = new_as_by_try_from::<u8, _, _>(&[char::MAX; 3], 0).unwrap_err();
		assert_try_from_char_error(&e);
	}
}
