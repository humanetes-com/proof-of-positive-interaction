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
	use frame_support::pallet_prelude::{*, DispatchResult};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn storage_getter)]
	pub type ExperienceStorage<T: Config> = StorageMap<
		_,
		Blake2_128Concat, //hashing algorithm
		T::AccountId,     //key
		(),               //value
	>;

	//pub type ListOfThings<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, ()>;
	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored { something: u32, who: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// Desired value does not exist in ExdperienceStorage
		ValueDoesNotExist,
		/// User already has experience
		/// This error is thrown when a user tries to create a new experience when they already have one
		UserAlreadyHasExperience,
	}


	/// This enum represents the different types of experience that a user can have
	/// Ideally, we want to this to be extensible so that we can add more types of experience
	pub enum ExperienceType {
		Frontend,
		Backend,
		Marketing,
		GraphicDesign,
	}

	/// This struct represents the a user's experience
	/// Due to the types of experience that a user can have
	pub struct UserExperience<T: Config> {
		/// The user's account id
		/// This allows for querying of user's with specific experience thresholds
		account_id: T::AccountId,
		/// The user's experience
		experience: u128,
		/// The user's experience level
		/// This is calculated from the user's experience
		level: u32,
		/// Experience required to reach the next level
		/// This is calculated from the user's experience
		experience_to_next_level: u128,
		/// Type of experience that this is referring to
		/// Eg. Frontend, Backend, Marketing, Graphic Design, etc.
		type_of_experience: ExperienceType,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored { something, who });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}

	/// The following impl and functions should not be accessible by the user
	/// For any function that needs to be accessible by the user, use the above implementation (under #[pallet::call] attribute)
	impl<T: Config> Pallet<T> {
		/// Creates a new user experience, based on experience type and user id
		/// Returns an error if the user already has experience
		// fn create_user_experience(user: T::AccountId) -> DispatchResult {
		// 	// Check if the user already has experience
		// 	ExperienceStorage::<T>::get(user).ok_or(Error::<T>::UserAlreadyHasExperience.into())?;

		// 	// Create a new user experience
		// 	let new_user_experience = UserExperience {
		// 		account_id: user,
		// 		experience: 0,
		// 		level: 0,
		// 		experience_to_next_level: 100,
		// 		type_of_experience: ExperienceType::Frontend,
		// 	};
		// }

		/// Takes in a user id and returns the user's experience if it exists, otherwise returns an error
		fn get_user_experience(user: T::AccountId) -> DispatchResult {
			ExperienceStorage::<T>::get(user).ok_or(Error::<T>::ValueDoesNotExist.into())
		}
	}
}
