use crate::*;

#[derive(Debug)]
pub struct VecTime<T>(Vec<T>);

impl<T> VecTime<T>
{
	/// Constructs new `VecTime<T>`.
	#[inline]
	#[must_use]
	pub fn new(vec: Vec<T>) -> Self
	{
		Self(vec)
	}

	/// Returns the number of elements in the VecTime.
	#[inline]
	pub fn len(&self) -> usize
	{
		self.0.len()
	}

	/// Extracts a slice.
	#[inline]
	pub fn as_slice(&self) -> &[T]
	{
		self.0.as_slice()
	}

	/// Extracts a mutable slice.
	#[inline]
	pub fn as_mut_slice(&mut self) -> &mut [T]
	{
		self.0.as_mut_slice()
	}

	/// Extracts a `Vec<T>`.
	#[inline]
	pub fn as_vec(&self) -> &Vec<T>
	{
		&self.0
	}

	/// Extracts a mutable `Vec<T>`
	#[inline]
	pub fn as_mut_vec(&mut self) -> &mut Vec<T>
	{
		&mut self.0
	}

	/// Compare time.
	pub fn compare<V>(&self, other: V) -> Result<CompareState>
	where T: Ord, V: AsRef<[T]>,
	{
		let mut ord = std::cmp::Ordering::Equal;
		let mut iter1 = self.0.iter();
		let mut iter2 = other.as_ref().iter();
		loop {
			let t1 = iter1.next();
			let t2 = iter2.next();
			if t1 == None && t2 == None {
				return match ord {
					std::cmp::Ordering::Equal => Ok(CompareState::Same),
					std::cmp::Ordering::Less => Ok(CompareState::Before),
					std::cmp::Ordering::Greater => Ok(CompareState::After),
				};
			}
			if t1 == None || t2 == None {
				return Err(Error::UnmatchTimeSize);
			}
			let o = t1.unwrap().cmp(t2.unwrap());
			if o == std::cmp::Ordering::Equal {
				continue;
			}
			if ord == std::cmp::Ordering::Equal {
				ord = o;
				continue;
			}
			if ord != o {
				return Ok(CompareState::Concurrent);
			}
		}
	}
}

impl<T> From<&VecTime<T>> for VecTime<T>
where T: Copy,
{
	#[inline]
	fn from(time: &VecTime<T>) -> Self
	{
		VecTime::new(time.0.clone())
	}
}

impl<T> From<&mut VecTime<T>> for VecTime<T>
where T: Copy,
{
	#[inline]
	fn from(time: &mut VecTime<T>) -> Self
	{
		VecTime::new(time.0.clone())
	}
}

impl<T> From<Vec<T>> for VecTime<T>
where T: Copy,
{
	#[inline]
	fn from(vec: Vec<T>) -> Self
	{
		VecTime::new(vec)
	}
}

impl<T> From<&Vec<T>> for VecTime<T>
where T: Copy,
{
	#[inline]
	fn from(vec: &Vec<T>) -> Self
	{
		VecTime::new(vec.clone())
	}
}

impl<T> From<&mut Vec<T>> for VecTime<T>
where T: Copy,
{
	#[inline]
	fn from(vec: &mut Vec<T>) -> Self
	{
		VecTime::new(vec.clone())
	}
}

impl<T> std::convert::AsRef<[T]> for VecTime<T>
{
	#[inline]
	fn as_ref(&self) -> &[T]
	{
		self.as_slice()
	}
}

impl<T> std::convert::AsMut<[T]> for VecTime<T>
{
	#[inline]
	fn as_mut(&mut self) -> &mut [T]
	{
		self.as_mut_slice()
	}
}

impl<T: Eq> PartialEq for VecTime<T> {
	#[inline]
	fn eq(&self, rhs: &VecTime<T>) -> bool {
		<VecTime<T> as PartialEq<[T]>>::eq(self, rhs.as_slice())
	}
}
impl<T: Ord> PartialOrd for VecTime<T> {
	#[inline]
	fn partial_cmp(&self, rhs: &VecTime<T>) -> Option<std::cmp::Ordering> {
		<VecTime<T> as PartialOrd<[T]>>::partial_cmp(self, rhs.as_slice())
	}
}

impl<T: Eq> PartialEq<[T]> for VecTime<T> {
	fn eq(&self, rhs: &[T]) -> bool {
		let mut iter1 = self.0.iter();
		let mut iter2 = rhs.iter();
		loop {
			let t1 = iter1.next();
			let t2 = iter2.next();
			if t1 == None && t2 == None {
				return true;
			}
			if t1 != t2 {
				return false;
			}
		}
	}
}
impl<T: Ord> PartialOrd<[T]> for VecTime<T> {
	fn partial_cmp(&self, rhs: &[T]) -> Option<std::cmp::Ordering> {
		match self.compare(rhs) {
			Ok(CompareState::Same) => Some(std::cmp::Ordering::Equal),
			Ok(CompareState::Before) => Some(std::cmp::Ordering::Less),
			Ok(CompareState::After) => Some(std::cmp::Ordering::Greater),
			_ => None,
		}
	}
}

impl<T: Eq> PartialEq<Vec<T>> for VecTime<T> {
	#[inline]
	fn eq(&self, rhs: &Vec<T>) -> bool {
		<VecTime<T> as PartialEq<[T]>>::eq(self, rhs.as_slice())
	}
}
impl<T: Ord> PartialOrd<Vec<T>> for VecTime<T> {
	#[inline]
	fn partial_cmp(&self, rhs: &Vec<T>) -> Option<std::cmp::Ordering> {
		<VecTime<T> as PartialOrd<[T]>>::partial_cmp(self, rhs.as_slice())
	}
}

impl<T: Eq> PartialEq<VecTime<T>> for Vec<T> {
	#[inline]
	fn eq(&self, rhs: &VecTime<T>) -> bool {
		<VecTime<T> as PartialEq<[T]>>::eq(rhs, self.as_slice())
	}
}
impl<T: Ord> PartialOrd<VecTime<T>> for Vec<T> {
	#[inline]
	fn partial_cmp(&self, rhs: &VecTime<T>) -> Option<std::cmp::Ordering> {
		<VecTime<T> as PartialOrd<[T]>>::partial_cmp(rhs, self.as_slice())
			.map(|o| std::cmp::Ordering::reverse(o))
	}
}

impl<T: Eq> PartialEq<Vec<T>> for &VecTime<T> {
	#[inline]
	fn eq(&self, rhs: &Vec<T>) -> bool {
		<VecTime<T> as PartialEq<[T]>>::eq(self, rhs.as_slice())
	}
}
impl<T: Ord> PartialOrd<Vec<T>> for &VecTime<T> {
	#[inline]
	fn partial_cmp(&self, rhs: &Vec<T>) -> Option<std::cmp::Ordering> {
		<VecTime<T> as PartialOrd<[T]>>::partial_cmp(self, rhs.as_slice())
	}
}

impl<T: Eq> PartialEq<&VecTime<T>> for Vec<T> {
	#[inline]
	fn eq(&self, rhs: &&VecTime<T>) -> bool {
		<VecTime<T> as PartialEq<[T]>>::eq(rhs, self.as_slice())
	}
}
impl<T: Ord> PartialOrd<&VecTime<T>> for Vec<T> {
	#[inline]
	fn partial_cmp(&self, rhs: &&VecTime<T>) -> Option<std::cmp::Ordering> {
		<VecTime<T> as PartialOrd<[T]>>::partial_cmp(rhs, self.as_slice())
			.map(|o| std::cmp::Ordering::reverse(o))
	}
}

impl<T: Eq> PartialEq<Vec<T>> for &mut VecTime<T> {
	#[inline]
	fn eq(&self, rhs: &Vec<T>) -> bool {
		<VecTime<T> as PartialEq<[T]>>::eq(self, rhs.as_slice())
	}
}
impl<T: Ord> PartialOrd<Vec<T>> for &mut VecTime<T> {
	#[inline]
	fn partial_cmp(&self, rhs: &Vec<T>) -> Option<std::cmp::Ordering> {
		<VecTime<T> as PartialOrd<[T]>>::partial_cmp(self, rhs.as_slice())
	}
}

impl<T: Eq> PartialEq<&mut VecTime<T>> for Vec<T> {
	#[inline]
	fn eq(&self, rhs: &&mut VecTime<T>) -> bool {
		<VecTime<T> as PartialEq<[T]>>::eq(rhs, self.as_slice())
	}
}
impl<T: Ord> PartialOrd<&mut VecTime<T>> for Vec<T> {
	#[inline]
	fn partial_cmp(&self, rhs: &&mut VecTime<T>) -> Option<std::cmp::Ordering> {
		<VecTime<T> as PartialOrd<[T]>>::partial_cmp(rhs, self.as_slice())
			.map(|o| std::cmp::Ordering::reverse(o))
	}
}

impl<T: Eq, const N: usize> PartialEq<[T; N]> for VecTime<T> {
	#[inline]
	fn eq(&self, rhs: &[T; N]) -> bool {
		<VecTime<T> as PartialEq<[T]>>::eq(self, rhs)
	}
}
impl<T: Ord, const N: usize> PartialOrd<[T; N]> for VecTime<T> {
	#[inline]
	fn partial_cmp(&self, rhs: &[T; N]) -> Option<std::cmp::Ordering> {
		<VecTime<T> as PartialOrd<[T]>>::partial_cmp(self, rhs)
	}
}

impl<T: Eq, const N: usize> PartialEq<VecTime<T>> for [T; N] {
	#[inline]
	fn eq(&self, rhs: &VecTime<T>) -> bool {
		<VecTime<T> as PartialEq<[T]>>::eq(rhs, self)
	}
}
impl<T: Ord, const N: usize> PartialOrd<VecTime<T>> for [T; N] {
	#[inline]
	fn partial_cmp(&self, rhs: &VecTime<T>) -> Option<std::cmp::Ordering> {
		<VecTime<T> as PartialOrd<[T]>>::partial_cmp(rhs, self)
			.map(|o| std::cmp::Ordering::reverse(o))
	}
}

impl<T: Eq, const N: usize> PartialEq<[T; N]> for &VecTime<T> {
	#[inline]
	fn eq(&self, rhs: &[T; N]) -> bool {
		<VecTime<T> as PartialEq<[T]>>::eq(self, rhs)
	}
}
impl<T: Ord, const N: usize> PartialOrd<[T; N]> for &VecTime<T> {
	#[inline]
	fn partial_cmp(&self, rhs: &[T; N]) -> Option<std::cmp::Ordering> {
		<VecTime<T> as PartialOrd<[T]>>::partial_cmp(self, rhs)
	}
}

impl<T: Eq, const N: usize> PartialEq<&VecTime<T>> for [T; N] {
	#[inline]
	fn eq(&self, rhs: &&VecTime<T>) -> bool {
		<VecTime<T> as PartialEq<[T]>>::eq(rhs, self)
	}
}
impl<T: Ord, const N: usize> PartialOrd<&VecTime<T>> for [T; N] {
	#[inline]
	fn partial_cmp(&self, rhs: &&VecTime<T>) -> Option<std::cmp::Ordering> {
		<VecTime<T> as PartialOrd<[T]>>::partial_cmp(rhs, self)
			.map(|o| std::cmp::Ordering::reverse(o))
	}
}

impl<T: Eq, const N: usize> PartialEq<[T; N]> for &mut VecTime<T> {
	#[inline]
	fn eq(&self, rhs: &[T; N]) -> bool {
		<VecTime<T> as PartialEq<[T]>>::eq(self, rhs)
	}
}
impl<T: Ord, const N: usize> PartialOrd<[T; N]> for &mut VecTime<T> {
	#[inline]
	fn partial_cmp(&self, rhs: &[T; N]) -> Option<std::cmp::Ordering> {
		<VecTime<T> as PartialOrd<[T]>>::partial_cmp(self, rhs)
	}
}

impl<T: Eq, const N: usize> PartialEq<&mut VecTime<T>> for [T; N] {
	#[inline]
	fn eq(&self, rhs: &&mut VecTime<T>) -> bool {
		<VecTime<T> as PartialEq<[T]>>::eq(rhs, self)
	}
}
impl<T: Ord, const N: usize> PartialOrd<&mut VecTime<T>> for [T; N] {
	#[inline]
	fn partial_cmp(&self, rhs: &&mut VecTime<T>) -> Option<std::cmp::Ordering> {
		<VecTime<T> as PartialOrd<[T]>>::partial_cmp(rhs, self)
			.map(|o| std::cmp::Ordering::reverse(o))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn call_as_ref<T, U>(time: &T) -> String
	where T: AsRef<[U]> + ?Sized + std::fmt::Debug, U: std::fmt::Debug,
	{
		format!("{:?}", time.as_ref())
	}

	fn call_as_mut<T, U>(time: &mut T) -> String
	where T: AsMut<[U]> + ?Sized + std::fmt::Debug, U: std::fmt::Debug,
	{
		format!("{:?}", time.as_mut())
	}

	fn call_from<T, U>(time: T) -> String
	where U: std::fmt::Debug, VecTime<U>: From<T> + std::fmt::Debug,
	{
		format!("{:?}", VecTime::from(time))
	}

	#[test]
	fn time_test() {
		let mut time1 = VecTime::new(vec![1; 3]);
		let time2 = VecTime::new(vec![1; 3]);

		// Same
		assert!(time1 == time2);
		assert!(!(time1 != time2));
		assert!(!(time1 < time2));
		assert!(time1 <= time2);
		assert!(!(time1 > time2));
		assert!(time1 >= time2);

		time1.as_mut_slice()[1] -= 1;

		// Before
		assert!(!(time1 == time2));
		assert!(time1 != time2);
		assert!(time1 < time2);
		assert!(time1 <= time2);
		assert!(!(time1 > time2));
		assert!(!(time1 >= time2));

		time1.as_mut_slice()[0] += 1;

		// Concurrent
		assert!(!(time1 == time2));
		assert!(time1 != time2);
		assert!(!(time1 < time2));
		assert!(!(time1 <= time2));
		assert!(!(time1 > time2));
		assert!(!(time1 >= time2));

		time1.as_mut_slice()[1] += 1;

		// After
		assert!(!(time1 == time2));
		assert!(time1 != time2);
		assert!(!(time1 < time2));
		assert!(!(time1 <= time2));
		assert!(time1 > time2);
		assert!(time1 >= time2);

		time1 = VecTime::new(vec![1; 2]);

		// Short length
		assert!(!(time1 == time2));
		assert!(time1 != time2);
		assert!(!(time1 < time2));
		assert!(!(time1 <= time2));
		assert!(!(time1 > time2));
		assert!(!(time1 >= time2));

		time1 = VecTime::new(vec![1; 4]);

		// Long length
		assert!(!(time1 == time2));
		assert!(time1 != time2);
		assert!(!(time1 < time2));
		assert!(!(time1 <= time2));
		assert!(!(time1 > time2));
		assert!(!(time1 >= time2));
	}

	#[test]
	fn time_vec_test() {
		let time1 = VecTime::new(vec![0; 3]);
		let vec1 = vec![1; 3];

		assert_eq!(time1.len(), vec1.len());

		assert!(!(time1 == vec1));
		assert!(time1 != vec1);
		assert!(time1 < vec1);
		assert!(time1 <= vec1);
		assert!(!(time1 > vec1));
		assert!(!(time1 >= vec1));

		assert!(!(vec1 == time1));
		assert!(vec1 != time1);
		assert!(!(vec1 < time1));
		assert!(!(vec1 <= time1));
		assert!(vec1 > time1);
		assert!(vec1 >= time1);
	}

	#[test]
	fn time_arr_test() {
		let time1 = VecTime::new(vec![0; 3]);
		let arr1 = [1; 3];

		assert_eq!(time1.len(), arr1.len());

		assert!(!(time1 == arr1));
		assert!(time1 != arr1);
		assert!(time1 < arr1);
		assert!(time1 <= arr1);
		assert!(!(time1 > arr1));
		assert!(!(time1 >= arr1));

		assert!(!(arr1 == time1));
		assert!(arr1 != time1);
		assert!(!(arr1 < time1));
		assert!(!(arr1 <= time1));
		assert!(arr1 > time1);
		assert!(arr1 >= time1);
	}

	#[test]
	fn time_ref_test() {
		let time1 = VecTime::new(vec![0; 3]);
		let time_ref1 = &time1;

		let vec1 = vec![0; 3];
		assert!(time_ref1 == vec1);
		assert!(time_ref1 <= vec1);
		assert!(vec1 == time_ref1);
		assert!(vec1 <= time_ref1);

		let arr1 = [0; 3];
		assert!(time_ref1 == arr1);
		assert!(time_ref1 <= arr1);
		assert!(arr1 == time_ref1);
		assert!(arr1 <= time_ref1);

		assert_eq!(format!("{:?}", time_ref1.as_slice()), "[0, 0, 0]");
		assert_eq!(format!("{:?}", time_ref1.as_vec()), "[0, 0, 0]");
	}

	#[test]
	fn time_mut_test() {
		let mut time1 = VecTime::new(vec![0; 3]);
		let time_mut1 = &mut time1;

		let vec1 = vec![0; 3];
		assert!(time_mut1 == vec1);
		assert!(time_mut1 <= vec1);
		assert!(vec1 == time_mut1);
		assert!(vec1 <= time_mut1);

		let arr1 = [0; 3];
		assert!(time_mut1 == arr1);
		assert!(time_mut1 <= arr1);
		assert!(arr1 == time_mut1);
		assert!(arr1 <= time_mut1);

		assert_eq!(format!("{:?}", time_mut1.as_slice()), "[0, 0, 0]");
		assert_eq!(format!("{:?}", time_mut1.as_mut_slice()), "[0, 0, 0]");
		assert_eq!(format!("{:?}", time_mut1.as_vec()), "[0, 0, 0]");
		assert_eq!(format!("{:?}", time_mut1.as_mut_vec()), "[0, 0, 0]");
	}

	#[test]
	fn time_func_arg_test() {
		let mut vec1 = vec![0; 3];
		let mut time1 = VecTime::new(vec![0; 3]);

		assert_eq!(call_as_ref(&time1), "[0, 0, 0]");
		assert_eq!(call_as_ref(&vec1), "[0, 0, 0]");

		assert_eq!(call_as_mut(&mut time1), "[0, 0, 0]");
		assert_eq!(call_as_mut(&mut vec1), "[0, 0, 0]");

		assert_eq!(call_from(&time1), "VecTime([0, 0, 0])");
		assert_eq!(call_from(&vec1), "VecTime([0, 0, 0])");
		assert_eq!(call_from(time1), "VecTime([0, 0, 0])");
		assert_eq!(call_from(vec1), "VecTime([0, 0, 0])");
	}
}
