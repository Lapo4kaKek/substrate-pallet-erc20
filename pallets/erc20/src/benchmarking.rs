//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	// #[benchmark]
	// fn do_something() {
	// 	let value = 100u32.into();
	// 	let caller: T::AccountId = whitelisted_caller();
	// 	#[extrinsic_call]
	// 	do_something(RawOrigin::Signed(caller), value);

	// 	assert_eq!(Something::<T>::get(), Some(value));
	// }
	#[benchmark]
	fn transfer() {
		let caller: T::AccountId = whitelisted_caller();
		let _ = Erc20::<T>::mint(RawOrigin::Root.into(), caller.clone(), Balance::from(1000));
		let to: T::AccountId = account("to", 2, SEED);

		#[extrinsic_call]
		_(RawOrigin::Signed(caller), to.clone(), Balance::from(100));

		assert_eq!(Balances::<T>::get(&to), Balance::from(100));
	}
	// #[benchmark]
	// fn cause_error() {
	// 	Something::<T>::put(100u32);
	// 	let caller: T::AccountId = whitelisted_caller();
	// 	#[extrinsic_call]
	// 	cause_error(RawOrigin::Signed(caller));

	// 	assert_eq!(Something::<T>::get(), Some(101u32));
	// }

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
