#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Time<T>([T]);

impl<T> Time<T>
{
	#[inline]
	#[must_use]
	pub fn new(s: &[T]) -> &Self
	{
		unsafe { &*(s as *const [T] as *const Self) }
	}

	#[inline]
	#[must_use]
	pub fn new_mut(s: &mut [T]) -> &mut Self
	{
		unsafe { &mut *(s as *mut [T] as *mut Self) }
	}

	#[inline]
	#[must_use]
	pub fn as_slice(&self) -> &[T]
	{
		&self.0
	}

	#[inline]
	#[must_use]
	pub fn as_mut_slice(&mut self) -> &mut [T]
	{
		&mut self.0
	}
}

impl<T> std::convert::AsRef<Time<T>> for Time<T>
{
	#[inline]
	fn as_ref(&self) -> &Time<T>
	{
		self
	}
}

impl<T> std::convert::AsMut<Time<T>> for Time<T>
{
	#[inline]
	fn as_mut(&mut self) -> &mut Time<T>
	{
		self
	}
}

impl<T> PartialOrd for Time<T>
where T: Ord,
{
	fn partial_cmp(&self, rhs: &Time<T>) -> Option<std::cmp::Ordering>
	{
		let mut ordering = std::cmp::Ordering::Equal;
		for (t0, t1) in std::iter::zip(&self.0, &rhs.0) {
			let o = t0.cmp(t1);
			if o == std::cmp::Ordering::Equal {
				continue;
			}
			if ordering == std::cmp::Ordering::Equal {
				ordering = o;
				continue;
			}
			if ordering != o {
				return None;
			}
		}
		Some(ordering)
	}
}
impl<T> PartialOrd<&mut Time<T>> for &Time<T>
where T: Ord,
{
	#[inline]
	fn partial_cmp(&self, rhs: &&mut Time<T>) -> Option<std::cmp::Ordering>
	{
		<Time<T> as PartialOrd>::partial_cmp(self.as_ref(), rhs.as_ref())
	}
}
impl<T> PartialOrd<&Time<T>> for &mut Time<T>
where T: Ord,
{
	#[inline]
	fn partial_cmp(&self, rhs: &&Time<T>) -> Option<std::cmp::Ordering>
	{
		<Time<T> as PartialOrd>::partial_cmp(self.as_ref(), rhs.as_ref())
	}
}

#[cfg(test)]
mod tests {
	use crate::*;

	fn call_as_mut_time<T, U>(time: &mut T, index: usize)
	where T: AsMut<Time<U>> + ?Sized, U: From<bool> + std::ops::AddAssign<U>,
	{
		time.as_mut().as_mut_slice()[index] += U::from(true)
	}

	fn call_as_mut_slice<T, U>(time: &mut T, index: usize)
	where T: AsMut<[U]> + ?Sized, U: From<bool> + std::ops::AddAssign<U>,
	{
		time.as_mut()[index] += U::from(true)
	}

	fn call_as_ref_time<T, U>(time: &T) -> String
	where T: AsRef<Time<U>> + ?Sized + std::fmt::Debug, U: std::fmt::Debug,
	{
		format!("{:?}", time.as_ref())
	}

	fn call_as_ref_slice<T, U>(time: &T) -> String
	where T: AsRef<[U]> + ?Sized + std::fmt::Debug, U: std::fmt::Debug,
	{
		format!("{:?}", time.as_ref())
	}

	#[test]
	fn time_vec_test() {
		let mut vec1 = vec![0u64; 3];
		let mut vec2 = vec![0u64; 3];
		let vec3 = vec![0u64; 3];

		call_as_mut_time(&mut vec1, 0);
		call_as_mut_slice(&mut vec1, 0);
		assert_eq!(call_as_ref_time(&vec1), "Time([2, 0, 0])");
		assert_eq!(call_as_ref_time(&vec2), "Time([0, 0, 0])");
		assert_eq!(call_as_ref_time(&vec3), "Time([0, 0, 0])");
		assert_eq!(call_as_ref_slice(&vec1), "[2, 0, 0]");
		assert_eq!(call_as_ref_slice(&vec2), "[0, 0, 0]");
		assert_eq!(call_as_ref_slice(&vec3), "[0, 0, 0]");

		let time1 = Time::new(&vec1);
		let time2 = Time::new_mut(&mut vec2);
		let time3 = Time::new(&vec3);

		assert!(time1 == vec1);
		assert!(!(time1 < vec1));
		assert!(vec1 == time1);
		assert!(!(vec1 < time1));

		assert!(time2 == time3);
		assert!(!(time2 != time3));
		assert!(!(time2 < time3));
		assert!(time2 <= time3);
		assert!(!(time2 > time3));
		assert!(time2 >= time3);

		call_as_mut_time(time2, 1);
		call_as_mut_slice(time2, 1);
		assert_eq!(call_as_ref_time(time1), "Time([2, 0, 0])");
		assert_eq!(call_as_ref_time(time2), "Time([0, 2, 0])");
		assert_eq!(call_as_ref_time(time3), "Time([0, 0, 0])");
		assert_eq!(call_as_ref_slice(time1), "[2, 0, 0]");
		assert_eq!(call_as_ref_slice(time2), "[0, 2, 0]");
		assert_eq!(call_as_ref_slice(time3), "[0, 0, 0]");

		assert!(!(time1 == time2));
		assert!(time1 != time2);
		assert!(!(time1 < time2));
		assert!(!(time1 <= time2));
		assert!(!(time1 > time2));
		assert!(!(time1 >= time2));

		assert!(!(time1 == time3));
		assert!(time1 != time3);
		assert!(!(time1 < time3));
		assert!(!(time1 <= time3));
		assert!(time1 > time3);
		assert!(time1 >= time3);

		assert!(!(time3 == time1));
		assert!(time3 != time1);
		assert!(time3 < time1);
		assert!(time3 <= time1);
		assert!(!(time3 > time1));
		assert!(!(time3 >= time1));
	}

	#[test]
	fn time_arr_test() {
		let mut arr1 = [0u64; 3];
		let mut arr2 = [0u64; 3];
		let arr3 = [0u64; 3];

		call_as_mut_time(&mut arr1, 0);
		call_as_mut_slice(&mut arr1, 0);
		assert_eq!(call_as_ref_time(&arr1), "Time([2, 0, 0])");
		assert_eq!(call_as_ref_time(&arr2), "Time([0, 0, 0])");
		assert_eq!(call_as_ref_time(&arr3), "Time([0, 0, 0])");
		assert_eq!(call_as_ref_slice(&arr1), "[2, 0, 0]");
		assert_eq!(call_as_ref_slice(&arr2), "[0, 0, 0]");
		assert_eq!(call_as_ref_slice(&arr3), "[0, 0, 0]");

		let time1 = Time::new(&arr1);
		let time2 = Time::new_mut(&mut arr2);
		let time3 = Time::new(&arr3);

		assert!(time1 == arr1);
		assert!(!(time1 < arr1));
		assert!(arr1 == time1);
		assert!(!(arr1 < time1));

		assert!(time2 == time3);
		assert!(!(time2 != time3));
		assert!(!(time2 < time3));
		assert!(time2 <= time3);
		assert!(!(time2 > time3));
		assert!(time2 >= time3);

		call_as_mut_time(time2, 1);
		call_as_mut_slice(time2, 1);
		assert_eq!(call_as_ref_time(time1), "Time([2, 0, 0])");
		assert_eq!(call_as_ref_time(time2), "Time([0, 2, 0])");
		assert_eq!(call_as_ref_time(time3), "Time([0, 0, 0])");
		assert_eq!(call_as_ref_slice(time1), "[2, 0, 0]");
		assert_eq!(call_as_ref_slice(time2), "[0, 2, 0]");
		assert_eq!(call_as_ref_slice(time3), "[0, 0, 0]");

		assert!(!(time1 == time2));
		assert!(time1 != time2);
		assert!(!(time1 < time2));
		assert!(!(time1 <= time2));
		assert!(!(time1 > time2));
		assert!(!(time1 >= time2));

		assert!(!(time1 == time3));
		assert!(time1 != time3);
		assert!(!(time1 < time3));
		assert!(!(time1 <= time3));
		assert!(time1 > time3);
		assert!(time1 >= time3);

		assert!(!(time3 == time1));
		assert!(time3 != time1);
		assert!(time3 < time1);
		assert!(time3 <= time1);
		assert!(!(time3 > time1));
		assert!(!(time3 >= time1));
	}

	#[test]
	fn time_slice_test() {
		let mut vec1 = vec![0u64; 3];
		let mut vec2 = vec![0u64; 3];
		let vec3 = vec![0u64; 3];

		let mut slice1 = vec1.as_mut_slice();
		let mut slice2 = vec2.as_mut_slice();
		let slice3 = vec3.as_slice();

		call_as_mut_time(&mut slice1, 0);
		call_as_mut_slice(&mut slice1, 0);
		assert_eq!(call_as_ref_time(&slice1), "Time([2, 0, 0])");
		assert_eq!(call_as_ref_time(&slice2), "Time([0, 0, 0])");
		assert_eq!(call_as_ref_time(&slice3), "Time([0, 0, 0])");
		assert_eq!(call_as_ref_slice(&slice1), "[2, 0, 0]");
		assert_eq!(call_as_ref_slice(&slice2), "[0, 0, 0]");
		assert_eq!(call_as_ref_slice(&slice3), "[0, 0, 0]");

		let time1 = Time::new(&slice1);
		let time2 = Time::new_mut(&mut slice2);
		let time3 = Time::new(&slice3);

		assert!(time1 == slice1);
		assert!(!(time1 < slice1));
		assert!(slice1 == time1);
		assert!(!(slice1 < time1));

		assert!(time2 == time3);
		assert!(!(time2 != time3));
		assert!(!(time2 < time3));
		assert!(time2 <= time3);
		assert!(!(time2 > time3));
		assert!(time2 >= time3);

		call_as_mut_time(time2, 1);
		call_as_mut_slice(time2, 1);
		assert_eq!(call_as_ref_time(time1), "Time([2, 0, 0])");
		assert_eq!(call_as_ref_time(time2), "Time([0, 2, 0])");
		assert_eq!(call_as_ref_time(time3), "Time([0, 0, 0])");
		assert_eq!(call_as_ref_slice(time1), "[2, 0, 0]");
		assert_eq!(call_as_ref_slice(time2), "[0, 2, 0]");
		assert_eq!(call_as_ref_slice(time3), "[0, 0, 0]");

		assert!(!(time1 == time2));
		assert!(time1 != time2);
		assert!(!(time1 < time2));
		assert!(!(time1 <= time2));
		assert!(!(time1 > time2));
		assert!(!(time1 >= time2));

		assert!(!(time1 == time3));
		assert!(time1 != time3);
		assert!(!(time1 < time3));
		assert!(!(time1 <= time3));
		assert!(time1 > time3);
		assert!(time1 >= time3);

		assert!(!(time3 == time1));
		assert!(time3 != time1);
		assert!(time3 < time1);
		assert!(time3 <= time1);
		assert!(!(time3 > time1));
		assert!(!(time3 >= time1));
	}
}
