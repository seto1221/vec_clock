//! # vec_clock
//!
//! A library for vector clock.
//!
//! # Examples
//!
//! ```
//! use vec_clock as vc;
//!
//! let mut clock = vc::new(vec![0u64; 3], 0).unwrap();
//! assert_eq!(clock.len(), 3);
//! assert_eq!(clock.self_index(), 0);
//! assert_eq!(clock.compare(&[0, 0, 0]).unwrap(), vc::CompareState::Same);
//! assert_eq!(format!("{:?}", clock), "VecClock { time: VecTime([0, 0, 0]), self_index: 0 }");
//! assert_eq!(format!("{:?}", clock.as_slice()), "[0, 0, 0]");
//!
//! let mut time = clock.time();
//! assert_eq!(time.len(), 3);
//! assert_eq!(time.compare(&[1, 0, 0]).unwrap(), vc::CompareState::Same);
//! assert_eq!(time.compare(&[1, 1, 0]).unwrap(), vc::CompareState::Before);
//! assert_eq!(time.compare(vc::convert(&[0u32, 1, 0])).unwrap(), vc::CompareState::Concurrent);
//! assert_eq!(time.compare(vc::try_convert(&[0i32, 0, 0]).unwrap()).unwrap(), vc::CompareState::After);
//! assert_eq!(format!("{:?}", time), "VecTime([1, 0, 0])");
//! assert_eq!(format!("{:?}", time.as_slice()), "[1, 0, 0]");
//!
//! time = clock.time();
//! assert_eq!(time.compare(&[2, 0, 0]).unwrap(), vc::CompareState::Same);
//!
//! time = clock.time();
//! assert_eq!(time.compare(&[3, 0, 0]).unwrap(), vc::CompareState::Same);
//!
//! time = clock.time_by(&[0, 1, 2]).unwrap();
//! assert_eq!(time.compare(&[4, 1, 2]).unwrap(), vc::CompareState::Same);
//!
//! time = clock.time_by(vc::try_convert(&[0i32, 1, 2]).unwrap()).unwrap();
//! assert_eq!(time.compare(&[5, 1, 2]).unwrap(), vc::CompareState::Same);
//!
//! assert_eq!(clock.compare(&[5, 1, 2]).unwrap(), vc::CompareState::Same);
//! ```

pub mod clock;
pub mod time;
pub mod error;
mod func;

pub use clock::VecClock;
pub use time::VecTime;
pub use error::Error;
pub use func::*;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum CompareState {
	Same,
	Before,
	After,
	Concurrent,
}

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
