// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # POPI Pallet
//!
//! Proof of positive interaction. A collectivity that contributes to build something.
//! There are different kind of things that may be built, each thing will go through
//! a set of lavoration steps. A lavoration step is registered onchain through an interaction.
//! The interaction is another user that verifies positively that your work contributed to move
//! a thing from the lavoration step n to the n+1.
//!
//! Yeah I know you didn't understand what I'm talking about. Lets make a concrete example:
//! a software company, may be an ecommerce software company, the website is built following a
//! kanban or scrum agile process. Think at the board typical columns:
//!
//! NEW	 | TODO | IN PROGRESS | CODE REVIEW | QA | READY TO DEPLOY | DEPLOYED | QA PRODUCTION | DONE
//!        Task1
//! Task2
//!                              Task3
//! ....
//!
//! We have tasks, task can be of 3 different kinds:
//! 1. UI = design the UI of a functionality
//! 2. FRONTEND DEVELOP = i.e. coding in React.js
//! 3. SERVER DEVELOP = i.e. implementing an API service consumed by the frontend
//! 4. BI = someone who analyses the user journey
//! 5. QA = Someone who validates something built
//! 6. PO = creator of tasks.
//! 7. PM = project manager
//!
//! Every task must follow all the steps from NEW to DONE. The progress can never be done by the
//! same person who worked on this specific task in this specific column.
//!
//! Lets make an example:
//! Task: change a color of a text "Click here to signin" from RED To Yellow.
//!
//! This task is created by UI. The task goes inmediately to the NEW column.
//! PO moves this to TODO.
//!
//! A developer PULLS this task, means: moves this task from TODO into IN PROGRESS>
//! Once she is done, set the task into "COMPLETED the current step". So small detail:
//! yes, someone else must move the task, but the person in this step must say "MY WORK IS READY TO
//! BE PULLED" So the board is simplified but actually it has the double of steps:
//! NEW| READY TO BE PULLED TO TODO | TODO| READY FOR INPROGRESS | INPROGRESS | READY FOR REVIEW |
//! REVIEW |
//!
//! three cases connected to the previous example:
//! 1. UI creates the task, this task must be well described, contain the color code corresponding
//!    to
//! Yellow. When she thinks the task is ready to be pulled by A PO, set the task as "READY TO BE
//! PULLED"
//!
//! 2. When a frontend developer takes a task from TODO to INPROGRESS. works on it, once she thinks
//!    everything is
//! completed and perhaps created a code review, sets this task as ready to be pulled
//!
//! 3. another developer will see the task as ready to be pulled and move it to Code review,
//! and so on.
//!
//! This Pallet is about the experience a person earns for working on something. We tried to figure
//! out the must fair way to earn experience, thinking also on how to resist to attacks by
//! introducing a few constraints:
//!
//! I already mentioned the first constraint, the most important: we work on a specific phase of a
//! task, set this as ready, but only another person may pull this task. Not anyone is capable of
//! pulling a task, but depending of the column, only an expert on that role may pull that task.
//! What makes you expert on something? We're talking about roles, roles are not preset. Like a QA
//! says " I am QA" or a root whitelist the user as QA. Tha'ts not what we want. Any person may work
//! on any kind of task, once someone else approves your task, we're setting a milestone. Someone is
//! saying "this person did a good job for this specific task in this specific phase". I'm setting a
//! STAMP. This stamp, makes the person which work on this task earn some experience points.
//!
//! That will be our ranking, and considering we may work on different kind of tasks, and our stamps
//! are given by different roles, we want to have a good granurality level keeping track of all
//! those aspects in a multidimensional matrix:
//!
//!  person given stamps by role | role1 | role 2 | role 3 | role 4
//! worked on kind of task:
//! kind 1
//! kind 2
//! kind 3
//! kind 4
//!
//! so for example, think on someone who is a bit QA, a bit developer.
//!
//! Alice worked on two tasks:
//! 1. as deveopler signup text color, changing from yellow to green.
//! 2. QA of a shopping cart functionality.
//!
//! Alice work on task 1, pull the task from TODO to INPROGRESS, once done set the task as ready for
//! review. A frontend expert pulls the task and move it to "Code review", finally approves that
//! task. Alice  work on task 2, pulling a task from CODE REVIEW DONE to QA, once done set QA to "QA
//! DONE"
//!
//! Many roles are involved in this process:
//! a. Task 1 pulled from TODO, was set as ready by a PO. Once Alice pulls it, is creating a proof
//! of positive interaction to the PO. Actually thinking on it, the proof is given just once Alice
//! finalize her task, and set it as READY to be pulled. cause the task creatd by the PO could have
//! low quality, not contain all the info needed and Alice never being able to finalize it. The next
//! role will make this point clearer: b. she sets the task as READY to be reviewed, cretes a code
//! review.A developer pulls it by movindg the task to CODE REVIEW, and the task could be low
//! quality, or not meet the espectations of the task. Only when the task will have an approved code
//! review, so the actual code review approval, will register a proof of positive interaction to
//! ALICE. c. once the code review is set as READY to be pulled, a QA will pull this task, work on
//! it, once the QA is ready, will set as QA done. Again, setting a positive proof of interaction
//! with the developer. d. a QA ready task, could be pulled perhaps by a PO(Product owner) or even
//! by a developer and moved to "READY TO BE RELEASED" e. when the task is deployed by a release
//! engineer or a developer, or a devops, this will increase the ranking of the PO.
//!
//! I hope now it is clear when I say that a person don't decides her role. Her role is an
//! experience depending on positive proof of interactiosn with other experts. Of course initially
//! in the project there will be no experts. Perhaps a ROOT could define initial roles, but I like
//! more the idea of initially every person has 0 as experience, and is allowed to approve
//! everything. We could define a formula that takes the norm of the experience on each role and
//! sets this as a minimum for approving a task. In this way the experience needed to approve will
//! be variable and grow with the overall experience of the community.
//!
//! Root(which could be for a specific project the PO), defines for a project two sets of values:
//! 1. kind of work(or in the board example, kind of tasks, for example a coding task,
//! 2. steps needed for a any work to be considered it DONE. In the board example, the columns of
//!    the board.
//! 3. The roles allowed to set a proof of positive interaction on some specific steps.
//! For example in our board example, only a PO is allowed to move a task from NEW to TO DO
//! Only a developer is allowed to allowed to pull and approve a code review. Or only a developer is
//! allowed to deploy the task.
//!
//! As I said before, what defines the a person to be a developer, is based on her experience, that
//! must be higher than the norm of all accounts in that specific role

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
	use frame_support::pallet_prelude::*;
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
}
