#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{pallet_prelude::*,  Blake2_128Concat};
	use frame_system::pallet_prelude::*;

	use scale_info::prelude::vec::Vec;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
		#[pallet::constant]
		type TokenName: Get<Vec<u8>>;
		#[pallet::constant]
		type TokenSymbol: Get<Vec<u8>>;
		#[pallet::constant]
		type TokenDecimals: Get<u8>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	// default total supply = 0
	#[pallet::type_value]
	pub(super) fn TotalSupplyDefault<T: Config>() -> u64 {
		0
	}

	#[pallet::storage]
	#[pallet::getter(fn total_supply)]
	type TotalSupply<T: Config> =
		StorageValue<Value = u64, QueryKind = ValueQuery, OnEmpty = TotalSupplyDefault<T>>;


	#[pallet::storage]
	#[pallet::getter(fn balances)]
	pub type Balances<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn allowances)]
	pub type Allowances<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		T::AccountId,
		u64,
		ValueQuery,
	>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Transfer { from: T::AccountId, to: T::AccountId, value: u64 },
		Approval { owner: T::AccountId, spender: T::AccountId, value: u64 },
		Mint { account: T::AccountId, value: u64 },
		Burn { account: T::AccountId, value: u64 },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		ERC20InsufficientBalance,
		ERC20InvalidSender,
		ERC20InvalidReceiver,
		ERC20InsufficientAllowance,
		ERC20InvalidApprover,
		ERC20InvalidSpender,
		AllowanceOverflow, // Maybe it's too much 
		AllowanceUnderflow, // Maybe it's too much 
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn approve(origin: OriginFor<T>, spender: T::AccountId, value: u64) -> DispatchResult {
			unimplemented!()
		}
	}
}
