# Vector Clock implementation for Rust

A library for vector clock.

# Examples

``` rust
use vec_clock as vc;

let mut clock = vc::new(vec![0u64; 3], 0).unwrap();
assert_eq!(clock.len(), 3);
assert_eq!(clock.self_index(), 0);
assert_eq!(clock.compare(&[0, 0, 0]).unwrap(), vc::CompareState::Same);
assert_eq!(format!("{:?}", clock), "VecClock { time: [0, 0, 0], self_index: 0 }");
assert_eq!(format!("{:?}", clock.as_slice()), "[0, 0, 0]");

let mut time = clock.time();
assert_eq!(time.len(), 3);
assert_eq!(time.compare(&[1, 0, 0]).unwrap(), vc::CompareState::Same);
assert_eq!(time.compare(&[1, 1, 0]).unwrap(), vc::CompareState::Before);
assert_eq!(time.compare(&[0, 1, 0]).unwrap(), vc::CompareState::Concurrent);
assert_eq!(time.compare(&[0, 0, 0]).unwrap(), vc::CompareState::After);
assert_eq!(format!("{:?}", time), "VecTime([1, 0, 0])");
assert_eq!(format!("{:?}", time.as_slice()), "[1, 0, 0]");

time = clock.time();
assert_eq!(time.compare(&[2, 0, 0]).unwrap(), vc::CompareState::Same);

time = clock.time();
assert_eq!(time.compare(&[3, 0, 0]).unwrap(), vc::CompareState::Same);

time = clock.time_by(vc::convert(&[0u32, 1, 2])).unwrap();
assert_eq!(time.compare(&[4, 1, 2]).unwrap(), vc::CompareState::Same);

time = clock.time_by(vc::try_convert(&[0i32, 1, 2]).unwrap()).unwrap();
assert_eq!(time.compare(&[5, 1, 2]).unwrap(), vc::CompareState::Same);

assert_eq!(clock.compare(&[5, 1, 2]).unwrap(), vc::CompareState::Same);
```
