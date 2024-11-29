use crate::*;

/*
   Vec -> Time
   (Time -> Vec is not implimented.)
*/

impl<T> std::convert::AsRef<Time<T>> for Vec<T>
{
	#[inline]
	fn as_ref(&self) -> &Time<T>
	{
		Time::new(&self)
	}
}

impl<T> std::convert::AsMut<Time<T>> for Vec<T>
{
	#[inline]
	fn as_mut(&mut self) -> &mut Time<T>
	{
		Time::new_mut(self)
	}
}

/*
   Time == Vec
*/

impl<T> PartialEq<Vec<T>> for Time<T>
where T: Eq, Vec<T>: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &Vec<T>) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(self, AsRef::as_ref(rhs))
	}
}
impl<T> PartialOrd<Vec<T>> for Time<T>
where T: Ord, Vec<T>: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &Vec<T>) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(self, AsRef::as_ref(rhs))
	}
}

impl<T> PartialEq<Vec<T>> for &Time<T>
where T: Eq, Vec<T>: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &Vec<T>) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(self, AsRef::as_ref(rhs))
	}
}
impl<T> PartialOrd<Vec<T>> for &Time<T>
where T: Ord, Vec<T>: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &Vec<T>) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(self, AsRef::as_ref(rhs))
	}
}

impl<T> PartialEq<Vec<T>> for &mut Time<T>
where T: Eq, Vec<T>: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &Vec<T>) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(self, AsRef::as_ref(rhs))
	}
}
impl<T> PartialOrd<Vec<T>> for &mut Time<T>
where T: Ord, Vec<T>: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &Vec<T>) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(self, AsRef::as_ref(rhs))
	}
}

impl<T> PartialEq<&Vec<T>> for Time<T>
where T: Eq, Vec<T>: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &&Vec<T>) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(self, AsRef::as_ref(rhs))
	}
}
impl<T> PartialOrd<&Vec<T>> for Time<T>
where T: Ord, Vec<T>: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &&Vec<T>) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(self, AsRef::as_ref(rhs))
	}
}

impl<T> PartialEq<&mut Vec<T>> for Time<T>
where T: Eq, Vec<T>: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &&mut Vec<T>) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(self, AsRef::as_ref(rhs))
	}
}
impl<T> PartialOrd<&mut Vec<T>> for Time<T>
where T: Ord, Vec<T>: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &&mut Vec<T>) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(self, AsRef::as_ref(rhs))
	}
}

/*
   Vec == Time
*/

impl<T> PartialEq<Time<T>> for Vec<T>
where T: Eq, Vec<T>: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &Time<T>) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(AsRef::as_ref(self), rhs)
	}
}
impl<T> PartialOrd<Time<T>> for Vec<T>
where T: Ord, Vec<T>: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &Time<T>) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(AsRef::as_ref(self), rhs)
	}
}

impl<T> PartialEq<Time<T>> for &Vec<T>
where T: Eq, Vec<T>: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &Time<T>) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(AsRef::as_ref(self), rhs)
	}
}
impl<T> PartialOrd<Time<T>> for &Vec<T>
where T: Ord, Vec<T>: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &Time<T>) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(AsRef::as_ref(self), rhs)
	}
}

impl<T> PartialEq<Time<T>> for &mut Vec<T>
where T: Eq, Vec<T>: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &Time<T>) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(AsRef::as_ref(self), rhs)
	}
}
impl<T> PartialOrd<Time<T>> for &mut Vec<T>
where T: Ord, Vec<T>: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &Time<T>) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(AsRef::as_ref(self), rhs)
	}
}

impl<T> PartialEq<&Time<T>> for Vec<T>
where T: Eq, Vec<T>: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &&Time<T>) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(AsRef::as_ref(self), rhs)
	}
}
impl<T> PartialOrd<&Time<T>> for Vec<T>
where T: Ord, Vec<T>: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &&Time<T>) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(AsRef::as_ref(self), rhs)
	}
}

impl<T> PartialEq<&mut Time<T>> for Vec<T>
where T: Eq, Vec<T>: AsRef<Time<T>>, Time<T>: PartialEq,
{
	#[inline]
	fn eq(&self, rhs: &&mut Time<T>) -> bool
	{
		<Time<T> as PartialEq<Time<T>>>::eq(AsRef::as_ref(self), rhs)
	}
}
impl<T> PartialOrd<&mut Time<T>> for Vec<T>
where T: Ord, Vec<T>: AsRef<Time<T>>, Time<T>: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, rhs: &&mut Time<T>) -> Option<core::cmp::Ordering>
	{
		<Time<T> as PartialOrd<Time<T>>>::partial_cmp(AsRef::as_ref(self), rhs)
	}
}
