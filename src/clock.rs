use crate::*;

#[derive(Debug)]
pub struct Clock<T = u64>
where T: Copy + Ord + From<bool> + From<T> + std::ops::AddAssign,
{
	pub(crate) time: Vec::<T>,
	pub(crate) self_index: usize,
}

impl<T> Clock<T>
where T: Copy + Ord + From<bool> + From<T> + std::ops::AddAssign,
{
	pub fn time(&mut self) -> &[T]
	{
		self.time[self.self_index] += T::from(true);
		&self.time
	}

	pub fn time_by<U>(&mut self, causal: &[U]) -> Result<&[T]>
	where T: From<U>, U: Copy,
	{
		if self.time.len() != causal.len() {
			Err(Error::UnmatchTimeSize)
		} else if self.time[self.self_index] < T::from(causal[self.self_index]) {
			Err(Error::InvalidTimeValue)
		} else {
			Ok(self.nocheck_time_by(causal))
		}
	}

	pub fn time_by_try_from<U>(&mut self, causal: &[U]) -> Result<&[T]>
	where T: TryFrom<U>, U: Copy, Error: From<<T as TryFrom<U>>::Error>,
	{
		let causal: Vec<T> = convert_try_from(causal)?;
		self.time_by(&causal)
	}

	pub fn nocheck_time_by<U>(&mut self, causal: &[U]) -> &[T]
	where T: From<U>, U: Copy,
	{
		std::iter::zip(&mut self.time, causal)
			.filter(|(&mut t0, &t1)| t0 < T::from(t1))
			.for_each(|(t0, t1)| *t0 = T::from(*t1));
		self.time()
	}

	pub fn compare<U>(&self, other: &[U]) -> Result<CompareState>
	where T: From<U>, U: Copy,
	{
		if self.time.len() != other.len() {
			Err(Error::UnmatchTimeSize)
		} else {
			Ok(compare(&self.time, other))
		}
	}

	pub fn compare_try_from<U>(&self, other: &[U]) -> Result<CompareState>
	where T: TryFrom<U>, U: Copy, Error: From<<T as TryFrom<U>>::Error>,
	{
		if self.time.len() != other.len() {
			Err(Error::UnmatchTimeSize)
		} else {
			compare_try_from(&self.time, other)
		}
	}

	pub fn nocheck_compare<U>(&self, other: &[U]) -> CompareState
	where T: From<U>, U: Copy,
	{
		compare(&self.time, other)
	}
}
