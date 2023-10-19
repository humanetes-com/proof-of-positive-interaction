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
		/// Represents the overall difficulty of leveling up
		type LevelDifficulty: Get<u32>;

		#[pallet::constant]
		/// The multiplier for the amount of experience required to level up
		type DifficultyMultiplier: Get<u32>;

		/// maximum number of chars used to identify a state
		type MaxBuildingStateNameLength: Get<u32>;
		/// maximum number of roles allowed to transition a specific state
		type MaxRolesAllowance: Get<u8>;

		type MaxBuildingStates: Get<u8>;

		type MaxBuildingStateLevel: Get<u8>;
		// key of a building state definition
		// #[pallet::constant]
		// /// Maximum number of historical positive interactions per account
		// type MaxPositiveUserInteractions: Get<u32>;

		/*
		level 1: 100
		level 2: 200
		level 3: 400
		level 4: 800
		level 5: 1600
		BaseExperience * DifficultyMultiplier ^ (LevelDifficulty * (level - 1))
		 */
	}

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
		InteractionAlreadyExisting,
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

	/// Product lifecycle states
	/// |STATE NAME
	///	| STATE LEVEL (predefined states 0=NON_READY, 1=TO BE PULLED, 2=IN PROGRESS, 03=APPROVED,
	/// 04=NON APPROVED,                state_level=5..255 remaining free for custom cases)
	/// | ROLES
	/// Please note: if a ticket must be done by server and client for example,
	/// there should be a ticket only for client, and a ticket only for server
	/// Example of states to populate on a typical software development board:
	///	todo = build(ready) which means ready to be built
	/// in progress = build(inprogress)
	/// ready to review = build(step completed)
	/// or build(ste_failed)
	/// review=review(in progress)
	/// for more example states and a better explanation you may visit:
	/// https://miro.com/app/board/uXjVMqnQG9M=/?moveToWidget=3458764562536777565&cot=14
	#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone)]
	#[scale_info(skip_type_params(T))]
	pub struct BuildingState<T: Config> {
		/// key. currently we're autoincrementing it, is there in frame some unique id random
		/// generation? is it needed?
		id: T::MaxBuildingStates,
		/// A state could be build, review, qa
		name: BoundedVec<u8, <T as Config>::MaxBuildingStateNameLength>,
		///0==non ready to be pulled, 1=ready to be pulled, 2=pulled, so in progress
		///3=work done, validation requested 4=validated and approved 5=non approved
		///remaining free levels could be used for custom states. I'm using u8 because
		///those values are encoded to scale, and the minimum is a byte.
		///I find it more optimized a byte than an enum where every option I think will
		///be encoded to a byte
		level: T::MaxBuildingStateLevel,
		///experiences needed to work on this specific task, a requirement engineer
		///perhaps a project manager, po, could fill these values during the definition
		///of the board, or when setting the task as ready to be worked
		work_roles_allowance: BoundedVec<ExperienceType, <T as Config>::MaxRolesAllowance>,
		///most of the times the validator contains the same experience as the worker
		///for example: a developer cretes a pull request, the validator should be a developer
		///but in some cases like in qa: sometimes it is a po, or even a dev who does qa
		validate_roles_allowance: BoundedVec<ExperienceType, <T as Config>::MaxRolesAllowance>,
	}

	/// https://miro.com/app/board/uXjVMqnQG9M=/?moveToWidget=3458764562536777565&cot=14
	#[derive(Encode, Decode, MaxEncodedLen, TypeInfo)]
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
		/// organization that owns the board id and task id
		org_id: u32,
		/// a transition takes place from an initial state
		initial_state_id: u8,
		/// to a final state
		final_state_id: u8,
	}

	#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Debug)]
	#[scale_info(skip_type_params(T))]
	/// This struct represents the a user's experience
	/// Due to the types of experience that a user can have
	pub struct UserExperience<T: Config> {
		/// The user's account id
		/// This allows for querying of user's with specific experience thresholds
		pub account_id: T::AccountId,
		/// The user's experience
		pub experience: u128,
		/// The user's experience level
		/// This is calculated from the user's experience
		pub level: u32,
		/// Experience required to reach the next level
		/// This is calculated from the user's experience
		pub experience_to_next_level: u128,
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
			org_id: u32,
			initial_state_id: u8,
			final_state_id: u8,
		) -> DispatchResult {
			let approver = ensure_signed(origin)?;
			let upi = InteractionIdentifier::<T> {
				approver,
				worker,
				board_id,
				task_id,
				org_id,
				initial_state_id,
				final_state_id,
			};
			return Self::store_interaction(upi)
		}
	}

	/// ------------------------------------------------------------------------------------
	/// Perhaps we could move the implementations and functions to another file
	/// ------------------------------------------------------------------------------------
	/// The following impl and functions should not be accessible by the user
	/// For any function that needs to be accessible by the user, use the above implementation
	/// (under #[pallet::call] attribute)
	impl<T: Config> Pallet<T> {
		pub fn store_interaction(upi: InteractionIdentifier<T>) -> DispatchResult {
			if Interaction::<T>::contains_key(&upi) {
				return Err(Error::<T>::InteractionAlreadyExisting.into())
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
				return Err(Error::<T>::UserAlreadyHasExperience.into())
			}

			// Create a new user experience
			let new_user_exp = UserExperience::<T> {
				account_id: user.clone(),
				experience: 0,
				level: 0,
				experience_to_next_level: T::BaseExperience::get(),
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

		fn update_user_experience(
			user: T::AccountId,
			experience: UserExperience<T>,
		) -> DispatchResult {
			unimplemented!()
		}

		/// Usees our Config types to calculate the amount of experience required to level up
		fn calculate_exp_to_next_level(
			experience: UserExperience<T>,
			level: u32,
		) -> DispatchResult {
			unimplemented!()
		}
	}
}
