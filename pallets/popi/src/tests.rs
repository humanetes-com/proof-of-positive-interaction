use crate::{mock::*, Error, Event, ExperienceType};
use frame_support::{assert_noop, assert_ok};
#[test]
fn i_know_how_to_work_with_vectors() {}

#[test]
fn interactions_should_be_unique() {
	new_test_ext().execute_with(|| {
		assert_ok!(Popi::interact(RuntimeOrigin::signed(1), 2, 1, 1, 1, 2, 3));
		assert_noop!(
			Popi::interact(RuntimeOrigin::signed(1), 2, 1, 1, 1, 2, 3),
			Error::<Test>::InteractionAlreadyExisting
		);
	});
}

#[test]
fn create_user_experiences() {
	new_test_ext().execute_with(|| {
		let account_id = 42;
		let exp_type = ExperienceType::Backend;

		// User exp has not been set yet, so it should return an error
		assert!(Popi::get_user_experience(account_id, exp_type).is_err());
		// Successfully create a new user experience
		assert_ok!(Popi::create_user_experience(account_id, exp_type.clone()));
		// Should not allow the duplicate creation of user experience
		assert_noop!(
			Popi::create_user_experience(account_id, exp_type),
			Error::<Test>::UserAlreadyHasExperience
		);
		// Successfully get the user experience
		assert_ok!(Popi::get_user_experience(account_id, exp_type));
		// Should not allow the creation of user experience for a different type that does not exist
		assert!(Popi::get_user_experience(account_id, ExperienceType::Frontend).is_err());
	});
}
