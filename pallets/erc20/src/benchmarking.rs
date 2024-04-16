//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;
use crate as pallet_erc20;

pub use pallet::*;
//mod mock;

use crate::{mock::*};


#[allow(unused)]
use crate::mock::ERC20Token;
//use crate::Pallet as ERC20Token;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use frame_support;

#[benchmarks]
mod benchmarks {
	use super::*;
	const SEED: u32 = 0;

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
		let _ = ERC20Token::<T>::mint(RawOrigin::Root.into(), 1000);
		let to: T::AccountId = account("to", 100, SEED);

		#[extrinsic_call]
		ERC20Token::<T>::transfer(RawOrigin::Signed(caller), to, 100);

		assert_eq!(Balances::<T>::get(&to), ERC20Token::balances(1000));
	}
	// #[benchmark]
	// fn cause_error() {
	// 	Something::<T>::put(100u32);
	// 	let caller: T::AccountId = whitelisted_caller();
	// 	#[extrinsic_call]
	// 	cause_error(RawOrigin::Signed(caller));

	// 	assert_eq!(Something::<T>::get(), Some(101u32));
	// }

	impl_benchmark_test_suite!(ERC20Token, crate::mock::new_test_ext(), crate::mock::Test);
}
