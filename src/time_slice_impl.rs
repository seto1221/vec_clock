use crate::*;

/*
   Time -> Slice
   (Slice -> Time is not implimented.)
*/

impl<T> std::convert::AsRef<[T]> for Time<T>
{
	#[inline]
	fn as_ref(&self) -> &[T]
	{
		self.as_slice()
	}
}

impl<T> std::convert::AsMut<[T]> for Time<T>
{
	#[inline]
	fn as_mut(&mut self) -> &mut [T]
	{
		self.as_mut_slice()
	}
}

impl<T> std::convert::AsRef<Time<T>> for [T]
{
	#[inline]
	fn as_ref(&self) -> &Time<T>
	{
		Time::new(&self)
	}
}

impl<T> std::convert::AsMut<Time<T>> for [T]
{
	#[inline]
	fn as_mut(&mut self) -> &mut Time<T>
	{
		Time::new_mut(self)
	}
}

/*
   Time == Slice
*/

impl<T> PartialEq<[T]> for &Time<T>
where T: Eq, [T]: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &[T]) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(self, AsRef::as_ref(rhs))
	}
}
impl<T> PartialOrd<[T]> for &Time<T>
where T: Ord, [T]: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &[T]) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(self, AsRef::as_ref(rhs))
	}
}

impl<T> PartialEq<[T]> for &mut Time<T>
where T: Eq, [T]: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &[T]) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(self, AsRef::as_ref(rhs))
	}
}
impl<T> PartialOrd<[T]> for &mut Time<T>
where T: Ord, [T]: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &[T]) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(self, AsRef::as_ref(rhs))
	}
}

impl<T> PartialEq<&[T]> for Time<T>
where T: Eq, [T]: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &&[T]) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(self, AsRef::as_ref(rhs))
	}
}
impl<T> PartialOrd<&[T]> for Time<T>
where T: Ord, [T]: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &&[T]) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(self, AsRef::as_ref(rhs))
	}
}

impl<T> PartialEq<&[T]> for &Time<T>
where T: Eq, [T]: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &&[T]) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(self, AsRef::as_ref(rhs))
	}
}
impl<T> PartialOrd<&[T]> for &Time<T>
where T: Ord, [T]: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &&[T]) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(self, AsRef::as_ref(rhs))
	}
}

impl<T> PartialEq<&[T]> for &mut Time<T>
where T: Eq, [T]: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &&[T]) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(self, AsRef::as_ref(rhs))
	}
}
impl<T> PartialOrd<&[T]> for &mut Time<T>
where T: Ord, [T]: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &&[T]) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(self, AsRef::as_ref(rhs))
	}
}

impl<T> PartialEq<&mut [T]> for Time<T>
where T: Eq, [T]: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &&mut [T]) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(self, AsRef::as_ref(rhs))
	}
}
impl<T> PartialOrd<&mut [T]> for Time<T>
where T: Ord, [T]: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &&mut [T]) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(self, AsRef::as_ref(rhs))
	}
}

impl<T> PartialEq<&mut [T]> for &Time<T>
where T: Eq, [T]: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &&mut [T]) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(self, AsRef::as_ref(rhs))
	}
}
impl<T> PartialOrd<&mut [T]> for &Time<T>
where T: Ord, [T]: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &&mut [T]) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(self, AsRef::as_ref(rhs))
	}
}

impl<T> PartialEq<&mut [T]> for &mut Time<T>
where T: Eq, [T]: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &&mut [T]) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(self, AsRef::as_ref(rhs))
	}
}
impl<T> PartialOrd<&mut [T]> for &mut Time<T>
where T: Ord, [T]: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &&mut [T]) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(self, AsRef::as_ref(rhs))
	}
}

/*
   Slice == Time
*/

impl<T> PartialEq<Time<T>> for &[T]
where T: Eq, [T]: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &Time<T>) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(AsRef::as_ref(self), rhs)
	}
}
impl<T> PartialOrd<Time<T>> for &[T]
where T: Ord, [T]: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &Time<T>) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(AsRef::as_ref(self), rhs)
	}
}

impl<T> PartialEq<Time<T>> for &mut [T]
where T: Eq, [T]: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &Time<T>) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(AsRef::as_ref(self), rhs)
	}
}
impl<T> PartialOrd<Time<T>> for &mut [T]
where T: Ord, [T]: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &Time<T>) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(AsRef::as_ref(self), rhs)
	}
}

impl<T> PartialEq<&Time<T>> for [T]
where T: Eq, [T]: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &&Time<T>) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(AsRef::as_ref(self), rhs)
	}
}
impl<T> PartialOrd<&Time<T>> for [T]
where T: Ord, [T]: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &&Time<T>) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(AsRef::as_ref(self), rhs)
	}
}

impl<T> PartialEq<&Time<T>> for &[T]
where T: Eq, [T]: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &&Time<T>) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(AsRef::as_ref(self), rhs)
	}
}
impl<T> PartialOrd<&Time<T>> for &[T]
where T: Ord, [T]: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &&Time<T>) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(AsRef::as_ref(self), rhs)
	}
}

impl<T> PartialEq<&Time<T>> for &mut [T]
where T: Eq, [T]: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &&Time<T>) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(AsRef::as_ref(self), rhs)
	}
}
impl<T> PartialOrd<&Time<T>> for &mut [T]
where T: Ord, [T]: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &&Time<T>) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(AsRef::as_ref(self), rhs)
	}
}

impl<T> PartialEq<&mut Time<T>> for [T]
where T: Eq, [T]: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &&mut Time<T>) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(AsRef::as_ref(self), rhs)
	}
}
impl<T> PartialOrd<&mut Time<T>> for [T]
where T: Ord, [T]: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &&mut Time<T>) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(AsRef::as_ref(self), rhs)
	}
}

impl<T> PartialEq<&mut Time<T>> for &[T]
where T: Eq, [T]: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &&mut Time<T>) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(AsRef::as_ref(self), rhs)
	}
}
impl<T> PartialOrd<&mut Time<T>> for &[T]
where T: Ord, [T]: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &&mut Time<T>) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(AsRef::as_ref(self), rhs)
	}
}

impl<T> PartialEq<&mut Time<T>> for &mut [T]
where T: Eq, [T]: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &&mut Time<T>) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(AsRef::as_ref(self), rhs)
	}
}
impl<T> PartialOrd<&mut Time<T>> for &mut [T]
where T: Ord, [T]: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &&mut Time<T>) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(AsRef::as_ref(self), rhs)
	}
}
