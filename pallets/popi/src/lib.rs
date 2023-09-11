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
	use frame_support::pallet_prelude::{DispatchResult, *};
	use frame_support::sp_runtime::ArithmeticError;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	///
	/// For additional information on BaseExperience, LevelDifficulty, and DifficultyMultiplier,
	/// check `fn calculate_exp_to_next_level`.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;

		#[pallet::constant]
		/// This is the amount of experience that a user needs to level up the first time
		/// In addition, this will impact the amount of experience required to level up in the
		/// future
		type BaseExperience: Get<u128>;

		#[pallet::constant]
		/// The multiplier for the amount of experience required to level up
		type DifficultyMultiplier: Get<u32>;

		// #[pallet::constant]
		// /// Maximum number of historical positive interactions per account
		// type MaxPositiveUserInteractions: Get<u32>;

		/*
		level 1: 100
		level 2: 400
		level 3: 900
		level 4: 1600
		level 5: 2500
		BaseExperience * (next_level ^ DifficultyMultiplier)
		 */
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	/// A positive interaction consists on a task state transition
	/// determined by a source AccountId on the work done by another dest accountId
	///
	/// TWOX-NOTE: Safe, as increasing integer keys are safe.
	//#[pallet::getter(fn positive_interaction_getter)]
	#[pallet::storage]
	pub type Interaction<T: Config> = StorageMap<_, Twox64Concat, InteractionIdentifier<T>, ()>;

	#[pallet::storage]
	#[pallet::getter(fn storage_getter)]
	pub type ExperienceStorage<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		(
			// This is the user's account id
			T::AccountId,
			// This is the type of experience that the user has
			ExperienceType,
		),
		// This is the user's experience struct, specific to the exact "ExperienceType"
		UserExperience<T>,
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
		UserExperienceDoesNotExist,
		/// User already has experience
		/// This error is thrown when a user tries to create a new experience when they already
		/// have one
		UserAlreadyHasExperience,
		/// an interaction is identified univoquely by (approver, worker, project_id, task_id,
		/// src_state, dst_state)
		InteractionExisting,
	}

	#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Debug, Clone, Copy)]
	/// This enum represents the different types of experience that a user can have
	/// Ideally, we want to this to be extensible so that we can add more types of experience
	pub enum ExperienceType {
		Frontend,
		Backend,
		Marketing,
		GraphicDesign,
	}

	#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Debug, Clone, Copy)]
	#[scale_info(skip_type_params(T))]
	/// Id that refer univoquely to an interaction between an approver and the owner
	/// of the increment, corresponding to a specific task of a specific board.
	/// Increment is referred to the portion of work executed in this task.
	pub struct InteractionIdentifier<T: Config> {
		/// a person with the right expertise to validate the increment proposed
		approver: T::AccountId,
		/// author of the increment
		worker: T::AccountId,
		/// we need to implement this storage, for now it is an abstract number
		/// this will be the specific board or project
		board_id: u32,
		/// id that identify the task inside of that board
		task_id: u32,
	}

	#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, Debug)]
	#[scale_info(skip_type_params(T))]
	/// This struct represents the a user's experience
	/// Due to the types of experience that a user can have
	pub struct UserExperience<T: Config> {
		/// The user's account id
		/// This allows for querying of user's with specific experience thresholds
		pub account_id: T::AccountId,
		/// The user's experience in the specific experience type
		pub experience: u128,
		/// The user's experience level
		/// This is calculated from the user's experience
		pub level: u32,
		/// Experience required to reach the next level
		/// This is calculated from the user's experience
		pub exp_to_next_lvl: u128,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		// #[pallet::weight(0)]
		// pub fn get_time(origin: OriginFor<T>) -> DispatchResult {
		// 	let _sender = ensure_signed(origin)?;
		// 	let _now = <timestamp::Pallet<T>>::get();
		// 	Ok(())
		// }
		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn interact(
			origin: OriginFor<T>,
			worker: T::AccountId,
			board_id: u32,
			task_id: u32,
		) -> DispatchResult {
			let approver = ensure_signed(origin)?;
			let upi = InteractionIdentifier::<T> { approver, worker, board_id, task_id };
			return Self::store_interaction(upi);
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
	/// For any function that needs to be accessible by the user, use the above implementation
	/// (under #[pallet::call] attribute)
	impl<T: Config> Pallet<T> {
		pub fn store_interaction(upi: InteractionIdentifier<T>) -> DispatchResult {
			if Interaction::<T>::contains_key(&upi) {
				return Err(Error::<T>::InteractionExisting.into());
			}
			Interaction::<T>::insert(&upi, ());
			Ok(())
		}
		/// Creates a new user experience, based on experience type and user id
		/// Returns an error if the user already has experience
		// May add "ensure_signed(origin)?" later on
		pub fn create_user_experience(
			user: T::AccountId,
			exp_type: ExperienceType,
		) -> DispatchResult {
			// Check if the user already has experience
			if ExperienceStorage::<T>::contains_key((&user, &exp_type)) {
				return Err(Error::<T>::UserAlreadyHasExperience.into());
			}

			// Create a new user experience
			let new_user_exp = UserExperience::<T> {
				account_id: user.clone(),
				experience: 0,
				level: 0,
				exp_to_next_lvl: T::BaseExperience::get(),
			};

			// Store the new user experience
			ExperienceStorage::<T>::set((user, &exp_type), Some(new_user_exp));
			Ok(())
		}

		/// Takes in a user id and returns the user's experience if it exists, otherwise returns an
		/// error
		pub fn get_user_experience(
			user: T::AccountId,
			exp_type: ExperienceType,
		) -> Result<UserExperience<T>, Error<T>> {
			ExperienceStorage::<T>::get((user, &exp_type))
				.ok_or(Error::<T>::UserExperienceDoesNotExist)
		}

		pub fn update_user_experience(
			user: T::AccountId,
			exp_type: ExperienceType,
			mut experience: UserExperience<T>,
		) -> DispatchResult {
			// check if the user has experience
			if !ExperienceStorage::<T>::contains_key((&user, &exp_type)) {
				return Err(Error::<T>::UserExperienceDoesNotExist.into());
			}

			experience.level = Self::calculate_exp_level(experience.experience, experience.level)?;
			let next_exp = Self::calc_exp_of_level(experience.level + 1)?;
			experience.exp_to_next_lvl = Self::remaining_exp(experience.experience, next_exp)?;

			// otherwise, update the user's experience
			ExperienceStorage::<T>::set((user, &exp_type), Some(experience));
			Ok(())
		}

		/// Uses our Config types to calculate the amount of experience required to level up
		/// BaseExperience * DifficultyMultiplier ^ (LevelDifficulty * (level - 1))
		pub fn calculate_exp_level(
			// Current experience of the user.
			experience: u128,
			// Current level of the user.
			level: u32,
		) -> Result<u32, DispatchError> {
			let mut new_level = level;
			let mut count: usize = 0;

			loop {
				// This will prevent an infinite loop
				// If the user has leveled up more than `count` times at once, then something is wrong
				if count > 10 {
					return Err(Error::<T>::StorageOverflow.into());
				}

				let next_level = new_level + 1;
				let exp_total = Self::calc_exp_of_level(next_level)?;

				// cover all cases
				if experience == exp_total {
					// can't level again if experience == exp_total
					new_level += 1;
					return Ok(new_level);
				} else if experience < exp_total {
					// if experience < exp_total, then we have found the level
					return Ok(new_level);
				} else {
					// if experience > exp_total, then we need to keep going
					new_level += 1;
					count += 1;
				}
			}
		}

		/// The level argument is the current level of the user.
		/// It's only purpose is to reduce computation by allowing us to start at the next level.
		/// The alternative would be to iterate through all levels, which would be more expensive.
		fn calc_exp_of_level(level: u32) -> Result<u128, DispatchError> {
			let exp_total = T::BaseExperience::get()
				.checked_mul(
					(level as u128)
						.checked_pow(T::DifficultyMultiplier::get())
						.ok_or(ArithmeticError::Overflow)?,
				)
				.ok_or(ArithmeticError::Overflow)?;
			Ok(exp_total)
		}

		fn remaining_exp(prev_exp: u128, next_exp: u128) -> Result<u128, DispatchError> {
			let result = next_exp.checked_sub(prev_exp).ok_or(ArithmeticError::Underflow)?;
			Ok(result)
		}
	}
}
