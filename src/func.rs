use crate::*;

/// Constructs new `VecClock<T>` from `Vec<T>`.
pub fn new<T>(vec: Vec<T>, self_index: usize) -> Result<VecClock<T>>
where T: Copy + Ord + From<bool> + std::ops::AddAssign,
{
	if self_index >= vec.len() {
		Err(Error::OutOfRangeIndex)
	} else {
		Ok(VecClock {time: vec, self_index})
	}
}

/// Convert to `Vec<T>` from `AsRef<[U]>` trait object.
pub fn convert<T, U, V>(time: V) -> Vec<T>
where T: From<U>, U: Copy, V: AsRef<[U]>,
{
	time.as_ref()
		.into_iter()
		.map(|&v| T::from(v))
		.collect()
}

/// Try convert to `Vec<T>` from `AsRef<[U]>` trait object.
pub fn try_convert<T, U, V>(time: V) -> Result<Vec<T>>
where
	T: TryFrom<U>, U: Copy, V: AsRef<[U]>,
	Error: From<<T as TryFrom<U>>::Error>,
{
	Ok(time.as_ref()
			.into_iter()
			.map(|&v| T::try_from(v))
			.collect::<std::result::Result<Vec<_>, _>>()?)
}

/// Compare time.
pub fn compare<'a, T, U, V>(time1: U, time2: V) -> Result<CompareState>
where T: Ord + 'a, V: AsRef<[T]>, VecTime<'a, T>: From<U>,
{
	VecTime::from(time1).compare(time2)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_func::*;

	#[test]
	fn new_test() {
		new(vec![0u8; 3], 0).unwrap();

		let e = new(vec![0u8; 3], 3).unwrap_err();
		assert_out_of_range_index(&e);

		let e = new::<u8>(vec![], 0).unwrap_err();
		assert_out_of_range_index(&e);
	}

	#[test]
	fn convert_test() {
		let mut arr1 = [0u8; 3];
		let mut vec1 = vec![0u8; 3];
		let mut time1 = VecTime::new(&vec1);

		assert_eq!(convert::<u64, _, _>(&time1), [0; 3]);
		assert_eq!(convert::<u64, _, _>(&mut time1), [0; 3]);
		assert_eq!(convert::<u64, _, _>(time1), [0; 3]);

		assert_eq!(convert::<i16, _, _>(&vec1), [0; 3]);
		assert_eq!(convert::<i32, _, _>(&mut vec1), [0; 3]);
		assert_eq!(convert::<i64, _, _>(vec1), [0; 3]);

		assert_eq!(convert::<u8, _, _>(&arr1), [0; 3]);
		assert_eq!(convert::<u16, _, _>(&mut arr1), [0; 3]);
		assert_eq!(convert::<u32, _, _>(arr1), [0; 3]);
	}

	#[test]
	fn try_convert_test() {
		assert_eq!(try_convert::<u8, _, _>([0; 3]).unwrap(), [0; 3]);

		let e = try_convert::<u8, _, _>([-1; 3]).unwrap_err();
		assert_try_from_int_error(&e);

		let e = try_convert::<u8, _, _>([!0u64; 3]).unwrap_err();
		assert_try_from_int_error(&e);

		let e = try_convert::<u8, _, _>([char::MAX; 3]).unwrap_err();
		assert_try_from_char_error(&e);
	}

	#[test]
	fn compare_test() {
		assert_eq!(compare::<u8, _, _>(&vec![1; 3], &[2, 1, 0]).unwrap(), CompareState::Concurrent);
		assert_eq!(compare::<u8, _, _>(&vec![1; 3], &[1, 1, 0]).unwrap(), CompareState::After);
		assert_eq!(compare::<u8, _, _>(&vec![1; 3], &[1, 1, 1]).unwrap(), CompareState::Same);
		assert_eq!(compare::<u8, _, _>(&vec![1; 3], &[1, 1, 2]).unwrap(), CompareState::Before);
		assert_eq!(compare::<u8, _, _>(&vec![1; 3], &[0, 1, 2]).unwrap(), CompareState::Concurrent);

		let e = compare::<u8, _, _>(&vec![1; 3], &[0; 2]).unwrap_err();
		assert_unmatch_time_size(&e);

		let e = compare::<u8, _, _>(&vec![1; 3], &[0; 4]).unwrap_err();
		assert_unmatch_time_size(&e);
	}
}
