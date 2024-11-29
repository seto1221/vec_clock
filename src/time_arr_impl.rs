use crate::*;

/*
   Array -> Time
   (Time -> Array is not implimented.)
*/

impl<T, const N: usize> std::convert::AsRef<Time<T>> for [T; N]
{
	#[inline]
	fn as_ref(&self) -> &Time<T>
	{
		Time::new(self)
	}
}

impl<T, const N: usize> std::convert::AsMut<Time<T>> for [T; N]
{
	#[inline]
	fn as_mut(&mut self) -> &mut Time<T>
	{
		Time::new_mut(self)
	}
}

/*
   Time == Array
*/

impl<T, const N: usize> PartialEq<[T; N]> for Time<T>
where T: Eq, [T; N]: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &[T; N]) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(self, AsRef::as_ref(rhs))
	}
}
impl<T, const N: usize> PartialOrd<[T; N]> for Time<T>
where T: Ord, [T; N]: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &[T; N]) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(self, AsRef::as_ref(rhs))
	}
}

impl<T, const N: usize> PartialEq<[T; N]> for &Time<T>
where T: Eq, [T; N]: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &[T; N]) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(self, AsRef::as_ref(rhs))
	}
}
impl<T, const N: usize> PartialOrd<[T; N]> for &Time<T>
where T: Ord, [T; N]: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &[T; N]) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(self, AsRef::as_ref(rhs))
	}
}

impl<T, const N: usize> PartialEq<[T; N]> for &mut Time<T>
where T: Eq, [T; N]: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &[T; N]) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(self, AsRef::as_ref(rhs))
	}
}
impl<T, const N: usize> PartialOrd<[T; N]> for &mut Time<T>
where T: Ord, [T; N]: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &[T; N]) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(self, AsRef::as_ref(rhs))
	}
}

impl<T, const N: usize> PartialEq<&[T; N]> for Time<T>
where T: Eq, [T; N]: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &&[T; N]) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(self, AsRef::as_ref(rhs))
	}
}
impl<T, const N: usize> PartialOrd<&[T; N]> for Time<T>
where T: Ord, [T; N]: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &&[T; N]) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(self, AsRef::as_ref(rhs))
	}
}

impl<T, const N: usize> PartialEq<&mut [T; N]> for Time<T>
where T: Eq, [T; N]: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &&mut [T; N]) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(self, AsRef::as_ref(rhs))
	}
}
impl<T, const N: usize> PartialOrd<&mut [T; N]> for Time<T>
where T: Ord, [T; N]: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &&mut [T; N]) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(self, AsRef::as_ref(rhs))
	}
}

/*
   Array == Time
*/

impl<T, const N: usize> PartialEq<Time<T>> for [T; N]
where T: Eq, [T; N]: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &Time<T>) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(AsRef::as_ref(self), rhs)
	}
}
impl<T, const N: usize> PartialOrd<Time<T>> for [T; N]
where T: Ord, [T; N]: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &Time<T>) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(AsRef::as_ref(self), rhs)
	}
}

impl<T, const N: usize> PartialEq<Time<T>> for &[T; N]
where T: Eq, [T; N]: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &Time<T>) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(AsRef::as_ref(self), rhs)
	}
}
impl<T, const N: usize> PartialOrd<Time<T>> for &[T; N]
where T: Ord, [T; N]: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &Time<T>) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(AsRef::as_ref(self), rhs)
	}
}

impl<T, const N: usize> PartialEq<Time<T>> for &mut [T; N]
where T: Eq, [T; N]: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &Time<T>) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(AsRef::as_ref(self), rhs)
	}
}
impl<T, const N: usize> PartialOrd<Time<T>> for &mut [T; N]
where T: Ord, [T; N]: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &Time<T>) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(AsRef::as_ref(self), rhs)
	}
}

impl<T, const N: usize> PartialEq<&Time<T>> for [T; N]
where T: Eq, [T; N]: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &&Time<T>) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(AsRef::as_ref(self), rhs)
	}
}
impl<T, const N: usize> PartialOrd<&Time<T>> for [T; N]
where T: Ord, [T; N]: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &&Time<T>) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(AsRef::as_ref(self), rhs)
	}
}

impl<T, const N: usize> PartialEq<&mut Time<T>> for [T; N]
where T: Eq, [T; N]: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &&mut Time<T>) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(AsRef::as_ref(self), rhs)
	}
}
impl<T, const N: usize> PartialOrd<&mut Time<T>> for [T; N]
where T: Ord, [T; N]: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &&mut Time<T>) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(AsRef::as_ref(self), rhs)
	}
}
