#[derive(Debug)]
pub enum Error {
	OutOfRangeIndex,
	UnmatchTimeSize,
	InvalidTimeValue,
	ExternalError(Box<dyn ExternalError>),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error
{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		match self {
			Self::OutOfRangeIndex => write!(f, "out of range index."),
			Self::UnmatchTimeSize => write!(f, "unmatch time size."),
			Self::InvalidTimeValue => write!(f, "invalid time value."),
			Self::ExternalError(e) => write!(f, "external error.({})", e),
		}
	}
}

impl<E> From<E> for Error
where E: ExternalError
{
	fn from(e: E) -> Self
	{
		Self::ExternalError(Box::new(e))
	}
}

pub trait ExternalError:
	std::fmt::Display + std::fmt::Debug + Sync + Send + 'static
{}

impl ExternalError for std::num::TryFromIntError {}
impl ExternalError for std::char::TryFromCharError {}
impl ExternalError for std::convert::Infallible {}
