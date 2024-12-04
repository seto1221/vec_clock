use crate::*;

#[derive(Debug)]
pub struct VecTime<'a, T>(&'a Vec<T>);

impl<'a, T> VecTime<'a, T>
{
	#[inline]
	#[must_use]
	pub fn new(vec: &'a Vec<T>) -> Self
	{
		Self(vec)
	}

	#[inline]
	pub fn len(&self) -> usize
	{
		self.0.len()
	}

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

impl<'a, T> From<&'a VecTime<'a, T>> for VecTime<'a, T>
{
	#[inline]
	fn from(time: &'a VecTime<'a, T>) -> Self
	{
		VecTime::new(&time.0)
	}
}

impl<'a, T> From<&'a mut VecTime<'a, T>> for VecTime<'a, T>
{
	#[inline]
	fn from(time: &'a mut VecTime<'a, T>) -> Self
	{
		VecTime::new(&time.0)
	}
}

impl<'a, T> From<&'a Vec<T>> for VecTime<'a, T>
{
	#[inline]
	fn from(vec: &'a Vec<T>) -> Self
	{
		VecTime::new(vec)
	}
}

impl<'a, T> From<&'a mut Vec<T>> for VecTime<'a, T>
{
	#[inline]
	fn from(vec: &'a mut Vec<T>) -> Self
	{
		VecTime::new(vec)
	}
}

impl<'a, T> std::convert::AsRef<[T]> for VecTime<'a, T>
{
	#[inline]
	fn as_ref(&self) -> &[T]
	{
		self.0.as_slice()
	}
}

impl<'a, T> PartialEq for VecTime<'a, T>
where T: Eq,
{
	#[inline]
	fn eq(&self, rhs: &VecTime<'a, T>) -> bool
	{
		<VecTime<'_, T> as PartialEq<[T]>>::eq(self, rhs.as_ref())
	}
}
impl<'a, T> PartialOrd for VecTime<'a, T>
where T: Ord,
{
	#[inline]
	fn partial_cmp(&self, rhs: &VecTime<'a, T>) -> Option<std::cmp::Ordering>
	{
		<VecTime<'_, T> as PartialOrd<[T]>>::partial_cmp(self, rhs.as_ref())
	}
}

impl<T> PartialEq<[T]> for VecTime<'_, T>
where T: Eq,
{
	fn eq(&self, rhs: &[T]) -> bool
	{
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
impl<T> PartialOrd<[T]> for VecTime<'_, T>
where T: Ord,
{
	fn partial_cmp(&self, rhs: &[T]) -> Option<std::cmp::Ordering>
	{
		match self.compare(rhs) {
			Ok(CompareState::Same) => Some(std::cmp::Ordering::Equal),
			Ok(CompareState::Before) => Some(std::cmp::Ordering::Less),
			Ok(CompareState::After) => Some(std::cmp::Ordering::Greater),
			_ => None,
		}
	}
}

impl<T> PartialEq<Vec<T>> for VecTime<'_, T>
where T: Eq,
{
	#[inline]
	fn eq(&self, rhs: &Vec<T>) -> bool
	{
		<VecTime<'_, T> as PartialEq<[T]>>::eq(self, rhs.as_slice())
	}
}
impl<T> PartialOrd<Vec<T>> for VecTime<'_, T>
where T: Ord,
{
	#[inline]
	fn partial_cmp(&self, rhs: &Vec<T>) -> Option<std::cmp::Ordering>
	{
		<VecTime<'_, T> as PartialOrd<[T]>>::partial_cmp(self, rhs.as_slice())
	}
}

impl<T> PartialEq<VecTime<'_, T>> for Vec<T>
where T: Eq,
{
	#[inline]
	fn eq(&self, rhs: &VecTime<'_, T>) -> bool
	{
		<VecTime<'_, T> as PartialEq<[T]>>::eq(rhs, self.as_slice())
	}
}
impl<T> PartialOrd<VecTime<'_, T>> for Vec<T>
where T: Ord,
{
	#[inline]
	fn partial_cmp(&self, rhs: &VecTime<'_, T>) -> Option<std::cmp::Ordering>
	{
		<VecTime<'_, T> as PartialOrd<[T]>>::partial_cmp(rhs, self.as_slice())
			.map(|o| std::cmp::Ordering::reverse(o))
	}
}

impl<T, const N: usize> PartialEq<[T; N]> for VecTime<'_, T>
where T: Eq,
{
	#[inline]
	fn eq(&self, rhs: &[T; N]) -> bool
	{
		<VecTime<'_, T> as PartialEq<[T]>>::eq(self, rhs)
	}
}
impl<T, const N: usize> PartialOrd<[T; N]> for VecTime<'_, T>
where T: Ord,
{
	#[inline]
	fn partial_cmp(&self, rhs: &[T; N]) -> Option<std::cmp::Ordering>
	{
		<VecTime<'_, T> as PartialOrd<[T]>>::partial_cmp(self, rhs)
	}
}

impl<T, const N: usize> PartialEq<VecTime<'_, T>> for [T; N]
where T: Eq,
{
	#[inline]
	fn eq(&self, rhs: &VecTime<'_, T>) -> bool
	{
		<VecTime<'_, T> as PartialEq<[T]>>::eq(rhs, self)
	}
}
impl<T, const N: usize> PartialOrd<VecTime<'_, T>> for [T; N]
where T: Ord,
{
	#[inline]
	fn partial_cmp(&self, rhs: &VecTime<'_, T>) -> Option<std::cmp::Ordering>
	{
		<VecTime<'_, T> as PartialOrd<[T]>>::partial_cmp(rhs, self)
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

	fn call_from<'a, T, U>(time: T) -> String
	where U: std::fmt::Debug + 'a, VecTime<'a, U>: From<T> + std::fmt::Debug,
	{
		format!("{:?}", VecTime::from(time))
	}

	fn call_from_ref<'a, T, U>(time: &'a T) -> String
	where T: 'a, U: std::fmt::Debug + 'a, VecTime<'a, U>: From<&'a T> + std::fmt::Debug,
	{
		format!("{:?}", VecTime::from(time))
	}

	fn call_from_mut<'a, T, U>(time: &'a mut T) -> String
	where T: 'a, U: std::fmt::Debug + 'a, VecTime<'a, U>: From<&'a mut T> + std::fmt::Debug,
	{
		format!("{:?}", VecTime::from(time))
	}

	#[test]
	fn time_test() {
		let mut vec1 = vec![1; 3];
		let mut time1 = VecTime::new(&vec1);
		let vec2 = vec![1; 3];
		let time2 = VecTime::new(&vec2);

		// Same
		assert!(time1 == time2);
		assert!(!(time1 != time2));
		assert!(!(time1 < time2));
		assert!(time1 <= time2);
		assert!(!(time1 > time2));
		assert!(time1 >= time2);

		vec1[1] -= 1;
		time1 = VecTime::new(&vec1);

		// Before
		assert!(!(time1 == time2));
		assert!(time1 != time2);
		assert!(time1 < time2);
		assert!(time1 <= time2);
		assert!(!(time1 > time2));
		assert!(!(time1 >= time2));

		vec1[0] += 1;
		time1 = VecTime::new(&vec1);

		// Concurrent
		assert!(!(time1 == time2));
		assert!(time1 != time2);
		assert!(!(time1 < time2));
		assert!(!(time1 <= time2));
		assert!(!(time1 > time2));
		assert!(!(time1 >= time2));

		vec1[1] += 1;
		time1 = VecTime::new(&vec1);

		// After
		assert!(!(time1 == time2));
		assert!(time1 != time2);
		assert!(!(time1 < time2));
		assert!(!(time1 <= time2));
		assert!(time1 > time2);
		assert!(time1 >= time2);

		vec1 = vec![1; 2];
		time1 = VecTime::new(&vec1);

		// Short length
		assert!(!(time1 == time2));
		assert!(time1 != time2);
		assert!(!(time1 < time2));
		assert!(!(time1 <= time2));
		assert!(!(time1 > time2));
		assert!(!(time1 >= time2));

		vec1 = vec![1; 4];
		time1 = VecTime::new(&vec1);

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
		let vec1 = vec![0; 3];
		let time1 = VecTime::new(&vec1);
		let vec2 = vec![1; 3];

		assert_eq!(time1.len(), vec2.len());

		assert!(!(time1 == vec2));
		assert!(time1 != vec2);
		assert!(time1 < vec2);
		assert!(time1 <= vec2);
		assert!(!(time1 > vec2));
		assert!(!(time1 >= vec2));

		assert!(!(vec2 == time1));
		assert!(vec2 != time1);
		assert!(!(vec2 < time1));
		assert!(!(vec2 <= time1));
		assert!(vec2 > time1);
		assert!(vec2 >= time1);
	}

	#[test]
	fn time_arr_test() {
		let vec1 = vec![0; 3];
		let time1 = VecTime::new(&vec1);
		let arr1 = [1; 3];

		assert_eq!(time1.len(), vec1.len());

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
	fn time_func_arg_test() {
		let mut vec1 = vec![0; 3];
		let time1 = VecTime::new(&vec1);

		assert_eq!(call_as_ref(&time1), "[0, 0, 0]");
		assert_eq!(call_as_ref(&vec1), "[0, 0, 0]");

		assert_eq!(call_from(&time1), "VecTime([0, 0, 0])");
		assert_eq!(call_from(&vec1), "VecTime([0, 0, 0])");
		assert_eq!(call_from(time1), "VecTime([0, 0, 0])");

		let mut time1 = VecTime::new(&vec1);

		assert_eq!(call_from_ref(&time1), "VecTime([0, 0, 0])");
		assert_eq!(call_from_ref(&vec1), "VecTime([0, 0, 0])");
		assert_eq!(call_from_mut(&mut time1), "VecTime([0, 0, 0])");
		assert_eq!(call_from_mut(&mut vec1), "VecTime([0, 0, 0])");
	}
}
