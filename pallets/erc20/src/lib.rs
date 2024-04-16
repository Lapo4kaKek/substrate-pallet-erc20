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
			let owner = ensure_signed(origin)?;
			
			<Allowances<T>>::set(&owner, &spender, value);
			//Self::increase_allowance(origin, spender.clone(), value)?;

			Self::deposit_event(Event::Approval { owner, spender, value });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn transfer(origin: OriginFor<T>, to: T::AccountId, value: u64) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			
			let sender_balance = Balances::<T>::get(&sender);
			let receiver_balance = Balances::<T>::get(&to);

			// check balance
			ensure!(sender_balance >= value, Error::<T>::ERC20InsufficientBalance);

			<Balances<T>>::set(&sender, sender_balance - value);
			<Balances<T>>::set(&to, receiver_balance + value);

			Self::deposit_event(Event::Transfer {from: sender.clone(), to, value});

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn transfer_from(origin: OriginFor<T>, from: T::AccountId, to: T::AccountId, value: u64) -> DispatchResult {
			let spender = ensure_signed(origin)?;
			
			let sender_balance = Balances::<T>::get(&from);
			let receiver_balance = Balances::<T>::get(&to);
			
			let allowance = <Allowances<T>>::get(&from, &spender);

			// check allowance
			ensure!(allowance >= value, Error::<T>::ERC20InsufficientAllowance);
			// check balance
			ensure!(sender_balance >= value, Error::<T>::ERC20InsufficientBalance);

			<Allowances<T>>::set(&from, &spender, allowance - value);
			// Self::decrease_allowance(origin, spender.clone(), value)?;

			<Balances<T>>::set(&spender, sender_balance - value);
			<Balances<T>>::set(&to, receiver_balance + value);

			Self::deposit_event(Event::Transfer {from: spender, to, value});

			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn burn(origin: OriginFor<T>, value: u64) -> DispatchResult {
			let owner = ensure_signed(origin)?;
			let owner_balance = <Balances<T>>::get(&owner);
			ensure!(owner_balance >= value, Error::<T>::ERC20InsufficientBalance);
			let total_supply = <TotalSupply<T>>::get();
			<TotalSupply<T>>::put(total_supply - value);
			<Balances<T>>::set(&owner, owner_balance - value);
			Self::deposit_event(Event::Burn { account: owner.clone(), value });
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn mint(origin: OriginFor<T>, value: u64) -> DispatchResult {
			let owner = ensure_signed(origin)?;
			let total_supply = <TotalSupply<T>>::get();

			<TotalSupply<T>>::put(total_supply + value);
			<Balances<T>>::set(&owner, <Balances<T>>::get(&owner) + value);

			Self::deposit_event(Event::Mint { account: owner.clone(), value });
			Ok(())
		}	

		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn increase_allowance(origin: OriginFor<T>, spender: T::AccountId, value: u64) -> DispatchResult {
			let owner = ensure_signed(origin)?;
			let current_allowance = <Allowances<T>>::get(&owner, &spender);

			let updated_allowance = current_allowance.checked_add(value).ok_or(Error::<T>::AllowanceOverflow)?;

			<Allowances<T>>::insert(&owner, &spender, updated_allowance);

			Self::deposit_event(Event::Approval { owner, spender, value: updated_allowance });
			Ok(())
		}

		#[pallet::call_index(6)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn decrease_allowance(origin: OriginFor<T>, spender: T::AccountId, value: u64 ) -> DispatchResult {
			let owner = ensure_signed(origin)?;
            let current_allowance = <Allowances<T>>::get(&owner, &spender);

            let updated_allowance = current_allowance.checked_sub(value).ok_or(Error::<T>::AllowanceUnderflow)?;

            <Allowances<T>>::insert(&owner, &spender, updated_allowance);

            Self::deposit_event(Event::Approval { owner, spender, value: updated_allowance });
            Ok(())
        }

	}
}
