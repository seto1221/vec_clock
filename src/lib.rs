mod clock;
mod time;
mod error;

pub use clock::Clock;
pub use time::Time;
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum CompareState {
	Same,
	After,
	Before,
	Concurrent,
}

pub fn new(n: usize, self_index: usize) -> Result<Clock<u64>>
{
	new_as::<u64>(n, self_index)
}

pub fn new_by<U>(initial: &[U], self_index: usize) -> Result<Clock<u64>>
where u64: From<U>, U: Copy,
{
	new_as_by::<u64, U>(initial, self_index)
}

pub fn new_by_try_from<U>(initial: &[U], self_index: usize) -> Result<Clock<u64>>
where u64: TryFrom<U>, U: Copy, Error: From<<u64 as TryFrom<U>>::Error>,
{
	new_as_by_try_from::<u64, U>(initial, self_index)
}

pub fn new_as<T>(n: usize, self_index: usize) -> Result<Clock<T>>
where T: Copy + Ord + From<bool> + From<T> + std::ops::AddAssign,
{
	new_as_by::<T, bool>(&vec![false; n], self_index)
}

pub fn new_as_by<T, U>(initial: &[U], self_index: usize) -> Result<Clock<T>>
where
	T: Copy + Ord + From<bool> + From<T> + From<U> + std::ops::AddAssign,
	U: Copy,
{
	if self_index >= initial.len() {
		Err(Error::OutOfRangeIndex)
	} else {
		let time: Vec<T> = convert(initial);
		Ok(Clock {time, self_index})
	}
}

pub fn new_as_by_try_from<T, U>(initial: &[U], self_index: usize) -> Result<Clock<T>>
where
	T: Copy + Ord + From<bool> + From<T> + TryFrom<U> + std::ops::AddAssign,
	U: Copy, Error: From<<T as TryFrom<U>>::Error>,
{
	if self_index >= initial.len() {
		Err(Error::OutOfRangeIndex)
	} else {
		let time: Vec<T> = convert_try_from(initial)?;
		Ok(Clock {time, self_index})
	}
}

pub fn convert<T, U>(time: &[U]) -> Vec<T>
where T: From<U>, U: Copy
{
	time.into_iter()
		.map(|&v| T::from(v))
		.collect()
}

pub fn convert_try_from<T, U>(time: &[U]) -> Result<Vec<T>>
where T: TryFrom<U>, U: Copy, Error: From<<T as TryFrom<U>>::Error>,
{
	Ok(time.into_iter()
			.map(|&v| T::try_from(v))
			.collect::<std::result::Result<Vec<_>, _>>()?)
}

pub fn compare<T, U>(clock1: &[T], clock2: &[U]) -> CompareState
where T: Copy + Ord + From<U>, U: Copy,
{
	let mut relation = CompareState::Same;
	for (&t0, &t1) in std::iter::zip(clock1, clock2) {
		let t1 = T::from(t1);
		if t0 < t1 {
			if relation == CompareState::After {
				return CompareState::Concurrent;
			}
			relation = CompareState::Before;
		} else if t0 > t1 {
			if relation == CompareState::Before {
				return CompareState::Concurrent;
			}
			relation = CompareState::After;
		}
	}
	relation
}

pub fn compare_try_from<T, U>(clock1: &[T], clock2: &[U]) -> Result<CompareState>
where T: Copy + Ord + From<T> + TryFrom<U>, U: Copy, Error: From<<T as TryFrom<U>>::Error>,
{
		let clock2: Vec<T> = convert_try_from(clock2)?;
		Ok(compare(clock1, &clock2))
}
