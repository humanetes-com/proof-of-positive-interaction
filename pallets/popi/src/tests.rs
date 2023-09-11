use crate::{mock::*, Error, Event, ExperienceType};
use std::*;
use frame_support::{assert_noop, assert_ok};
#[test]
fn i_know_how_to_work_with_vectors() {}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(Popi::cause_error(RuntimeOrigin::signed(1)), Error::<Test>::NoneValue);
	});
}

#[test]
fn interactions_should_be_unique() {
	new_test_ext().execute_with(|| {
		assert_ok!(Popi::interact(RuntimeOrigin::signed(1), 2, 1, 1));
		assert_noop!(
			Popi::interact(RuntimeOrigin::signed(1), 2, 1, 1),
			Error::<Test>::InteractionExisting
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
#[test]
fn update_user_experiences() {
	new_test_ext().execute_with(|| {
		let account_id = 42;
		let exp_type = ExperienceType::Backend;

		let _ = Popi::create_user_experience(account_id, exp_type.clone());
		// Successfully get the user experience
		assert_ok!(Popi::get_user_experience(account_id, exp_type));
		
		let mut user_exp = Popi::get_user_experience(account_id, exp_type).unwrap();
		user_exp.experience = 100;

		// Successfully update the user experience
		assert_ok!(Popi::update_user_experience(account_id, exp_type, user_exp));
		// Successfully get the user experience
		let new_exp = Popi::get_user_experience(account_id, exp_type).unwrap();
		assert_eq!(new_exp.experience, 100);
	});
}

#[test]
fn level_up_user_twice() {
	new_test_ext().execute_with(|| {
		let account_id = 42;
		let exp_type = ExperienceType::Marketing;

		let _ = Popi::create_user_experience(account_id, exp_type.clone());
		// Successfully get the user experience
		assert_ok!(Popi::get_user_experience(account_id, exp_type));
		
		let mut user_exp = Popi::get_user_experience(account_id, exp_type).unwrap();
		user_exp.experience = 100;

		// Successfully update the user experience
		assert_ok!(Popi::update_user_experience(account_id, exp_type, user_exp));
		// Successfully get the user experience
		let mut new_exp = Popi::get_user_experience(account_id, exp_type).unwrap();
		assert_eq!(new_exp.experience, 100);
		assert_eq!(new_exp.level, 1);

		new_exp.experience = 400;
		let _ = Popi::update_user_experience(account_id, exp_type, new_exp);
		let updated_exp = Popi::get_user_experience(account_id, exp_type);
		assert_eq!(updated_exp.unwrap().level, 2);
	});
}

#[test]
fn level_up_three_times_at_once() {
	new_test_ext().execute_with(|| {
		let account_id = 42;
		let exp_type = ExperienceType::Marketing;

		let _ = Popi::create_user_experience(account_id, exp_type.clone());
		// Successfully get the user experience
		assert_ok!(Popi::get_user_experience(account_id, exp_type));
		
		let mut user_exp = Popi::get_user_experience(account_id, exp_type).unwrap();
		user_exp.experience = 1400;

		// Successfully update the user experience
		assert_ok!(Popi::update_user_experience(account_id, exp_type, user_exp));
		// Successfully get the user experience
		let new_exp = Popi::get_user_experience(account_id, exp_type).unwrap();
		println!("new_exp: {:?}", new_exp);
		assert_eq!(new_exp.experience, 1400);
		assert_eq!(new_exp.level, 3);
		assert_eq!(new_exp.exp_to_next_lvl, 200);
	});
}
