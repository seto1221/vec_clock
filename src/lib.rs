mod clock;
mod time;
mod error;
mod new;
mod func;

pub use clock::Clock;
pub use time::Time;
pub use error::Error;
pub use error::Result;
pub use new::*;
pub use func::*;

#[cfg(test)]
pub mod test_func {
	use super::*;

	pub fn assert_out_of_range_index(e: &Error) {
		match e {
			Error::OutOfRangeIndex => (),
			_ => panic!("assertion failed: assert_out_of_range_index({:?})", e),
		}
	}

	pub fn assert_unmatch_time_size(e: &Error) {
		match e {
			Error::UnmatchTimeSize => (),
			_ => panic!("assertion failed: assert_unmatch_time_size({:?})", e),
		}
	}

	pub fn assert_invalid_time_value(e: &Error) {
		match e {
			Error::InvalidTimeValue => (),
			_ => panic!("assertion failed: assert_invalid_time_value({:?})", e),
		}
	}

	pub fn assert_try_from_int_error(e: &Error) {
		match e {
			Error::ExternalError(e) => {
				let e0 = format!("{:?}", e);
				let e1 = format!("{:?}", u8::try_from(-1).unwrap_err());
				assert_eq!(e0, e1)
			}
			_ => panic!("assertion failed: assert_try_from_int_error({:?})", e),
		}
	}

	pub fn assert_try_from_char_error(e: &Error) {
		match e {
			Error::ExternalError(e) => {
				let e0 = format!("{:?}", e);
				let e1 = format!("{:?}", u8::try_from(char::MAX).unwrap_err());
				assert_eq!(e0, e1)
			}
			_ => panic!("assertion failed: assert_try_from_char_error({:?})", e),
		}
	}
}
