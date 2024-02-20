#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::storage]
	#[pallet::getter(fn number)]
	pub type Number<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn set_number(origin: OriginFor<T>, value: u32) -> DispatchResult {
			ensure_root(origin)?;

			Number::<T>::put(value);
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn increment_number(origin: OriginFor<T>, value: u32) -> DispatchResult {
			ensure_signed(origin)?;

			Number::<T>::mutate(|num| *num = num.saturating_add(value));
			Self::deposit_event(Event::IncrementNumber(value));
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn decrement_number(origin: OriginFor<T>) -> DispatchResult {
			ensure_signed(origin)?;

			Number::<T>::mutate(|val| *val -= 1);
			Ok(())
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		IncrementNumber(u32),
	}
}
