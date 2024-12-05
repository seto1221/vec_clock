use crate::*;

#[derive(Debug)]
pub struct VecClock<T = u64>
where T: Copy + Ord + From<bool> + From<T> + std::ops::AddAssign,
{
	pub(crate) time: Vec::<T>,
	pub(crate) self_index: usize,
}

impl<T> VecClock<T>
where T: Copy + Ord + From<bool> + From<T> + std::ops::AddAssign,
{
	pub fn time(&mut self) -> VecTime<T>
	{
		self.time[self.self_index] += T::from(true);
		VecTime::new(&self.time)
	}

	pub fn time_by<U>(&mut self, causal: U) -> Result<VecTime<T>>
	where U: AsRef<[T]>,
	{
		let causal_ref = causal.as_ref();
		if self.time.len() != causal_ref.len() {
			Err(Error::UnmatchTimeSize)
		} else if self.time[self.self_index] < T::from(causal_ref[self.self_index]) {
			Err(Error::InvalidTimeValue)
		} else {
			Ok(self.nocheck_time_by(causal_ref))
		}
	}

	pub fn nocheck_time_by<U>(&mut self, causal: U) -> VecTime<T>
	where U: AsRef<[T]>,
	{
		std::iter::zip(&mut self.time, causal.as_ref())
			.filter(|(&mut t0, &t1)| t0 < t1)
			.for_each(|(t0, t1)| *t0 = *t1);
		self.time()
	}

	#[inline]
	pub fn self_index(&self) -> usize
	{
		self.self_index
	}

	#[inline]
	pub fn len(&self) -> usize
	{
		self.time.len()
	}

	#[inline]
	pub fn as_slice(&self) -> &[T]
	{
		self.time.as_slice()
	}

	pub fn compare<U>(&self, other: U) -> Result<CompareState>
	where U: AsRef<[T]>, for<'a> VecTime<'a, T>: From<&'a Vec<T>>,
	{
		compare(&self.time, other)
	}
}

#[cfg(test)]
mod tests {
	use crate::*;
	use crate::test_func::*;

	#[test]
	fn clock_time_test() {
		let mut vc = new(vec![0u64; 1], 0).unwrap();
		assert!(vc.time() == [1]);
		assert!(vc.time() == [2]);
		assert!(vc.time() == [3]);

		let mut vc = new(vec![0u64; 3], 0).unwrap();
		assert!(vc.time() == [1, 0, 0]);
		assert!(vc.time() == [2, 0, 0]);
		assert!(vc.time() == [3, 0, 0]);

		let mut vc = new(vec![0u64; 3], 1).unwrap();
		assert!(vc.time() == [0, 1, 0]);
		assert!(vc.time() == [0, 2, 0]);
		assert!(vc.time() == [0, 3, 0]);

		let mut vc = new(vec![0u64; 3], 2).unwrap();
		assert!(vc.time() == [0, 0, 1]);
		assert!(vc.time() == [0, 0, 2]);
		assert!(vc.time() == [0, 0, 3]);
	}

	#[test]
	#[should_panic(expected = "attempt to add with overflow")]
	fn clock_time_overflow_test() {
		let mut vc = new(vec![0u64, !0, 0], 1).unwrap();
		vc.time();
	}

	#[test]
	fn clock_time_by_test() {
		let mut vc = new(vec![0u64; 3], 1).unwrap();
		assert!(vc.time_by(&[0u64; 3]).unwrap() == [0, 1, 0]);
		assert!(vc.time_by(&[1u64, 1, 3]).unwrap() == [1, 2, 3]);
		assert!(vc.time_by(&[!0u64, 0, !0 - 1]).unwrap() == [!0, 3, !0 - 1]);

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
		let mut vc = new(vec![0u64, !0, 0], 1).unwrap();
		let _ = vc.time_by(&[0u64; 3]);
	}

	#[test]
	fn clock_nocheck_time_by_test() {
		let mut vc = new(vec![0u64; 3], 1).unwrap();
		assert!(vc.nocheck_time_by(&[0u64; 3]) == [0, 1, 0]);
		assert!(vc.nocheck_time_by(&[1u64, 1, 3]) == [1, 2, 3]);
		assert!(vc.nocheck_time_by(&[!0u64, 0, !0 - 1]) == [!0, 3, !0 - 1]);

		assert!(vc.nocheck_time_by(&[0u64; 2]) != [!0, 5, !0 - 1]);
		assert!(vc.nocheck_time_by(&[0u64; 4]) != [!0, 6, !0 - 1]);
	}

	#[test]
	#[should_panic(expected = "attempt to add with overflow")]
	fn clock_nocheck_time_by_overflow_test() {
		let mut vc = new(vec![0u64; 3], 1).unwrap();
		let _ = vc.nocheck_time_by(&[0u64, !0, 0]);
	}

	#[test]
	fn clock_compare_test() {
		let vc = new(vec![1u64; 3], 1).unwrap();
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

		let e = vc.compare(&[0u64; 2]).unwrap_err();
		assert_unmatch_time_size(&e);

		let e = vc.compare(&[0u64; 4]).unwrap_err();
		assert_unmatch_time_size(&e);
	}
}
