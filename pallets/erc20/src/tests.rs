use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};


#[test]
fn should_fail_burn_if_insufficient_balance() {
    new_test_ext().execute_with(|| {
        let owner = 1;
        // owner has 0 tokens initially
        assert_noop!(
            ERC20Token::burn(RuntimeOrigin::signed(owner), 100),
            Error::<Test>::ERC20InsufficientBalance
        );
    });
}


#[test]
fn should_increase_total_supply_on_mint() {
	new_test_ext().execute_with(|| {
		assert_ok!(ERC20Token::mint(RuntimeOrigin::signed(1), 100));
		assert_ok!(ERC20Token::mint(RuntimeOrigin::signed(2), 200));

		assert_eq!(ERC20Token::total_supply(), 300);
	});
}

#[test]
fn should_fail_transfer_insufficient_balance() {
 new_test_ext().execute_with(|| {
  System::set_block_number(1);
  assert_noop!(
   ERC20Token::transfer(RuntimeOrigin::signed(1).into(), 1, 50),
   Error::<Test>::ERC20InsufficientBalance
  );
 });
}



#[test]
fn should_decrease_total_supply_on_burn() {
	new_test_ext().execute_with(|| {
		assert_ok!(ERC20Token::mint(RuntimeOrigin::signed(1), 100));
		assert_ok!(ERC20Token::burn(RuntimeOrigin::signed(1), 50));
		assert_eq!(ERC20Token::total_supply(), 50);
	});
}

#[test]
fn test_events() {
 new_test_ext().execute_with(|| {
  System::set_block_number(1);
  assert_ok!(ERC20Token::mint(RuntimeOrigin::signed(1).into(), 100));
  System::assert_last_event(Event::Mint { account: 1, value: 100 }.into());
  assert_ok!(ERC20Token::transfer(RuntimeOrigin::signed(1).into(), 2, 50));
  System::assert_last_event(Event::Transfer { from: 1, to: 2, value: 50 }.into());
  assert_ok!(ERC20Token::approve(RuntimeOrigin::signed(1), 2, 50));
  System::assert_last_event(Event::Approval { owner: 1, spender: 2, value: 50 }.into());
  assert_ok!(ERC20Token::transfer_from(RuntimeOrigin::signed(2), 1, 3, 40));
  System::assert_last_event(Event::Transfer { from: 1, to: 3, value: 40 }.into());
  assert_ok!(ERC20Token::burn(RuntimeOrigin::signed(1), 10));
  System::assert_last_event(Event::Burn { account: 1, value: 10 }.into());
 });
}


#[test]
fn should_transfer_tokens_between_accounts() {
	new_test_ext().execute_with(|| {
		assert_ok!(ERC20Token::mint(RuntimeOrigin::signed(1), 100));
		assert_ok!(ERC20Token::transfer(RuntimeOrigin::signed(1), 2, 50));
		assert_eq!(ERC20Token::total_supply(), 100);
		assert_eq!(ERC20Token::balances(1), 50);
		assert_eq!(ERC20Token::balances(2), 50);
	});
}


#[test]
fn should_transfer_from_tokens_between_accounts_after_approval() {
	new_test_ext().execute_with(|| {
		let owner = 1;
		let spender = 2;
		let target = 3;
		// owner
		assert_ok!(ERC20Token::mint(RuntimeOrigin::signed(owner), 500));
		assert_ok!(ERC20Token::approve(RuntimeOrigin::signed(owner), spender, 128));
		assert_eq!(ERC20Token::allowances(owner, spender), 128);

		assert_ok!(ERC20Token::transfer_from(RuntimeOrigin::signed(spender), owner, target, 50));
		assert_eq!(ERC20Token::allowances(owner, spender), 78);
		assert_eq!(ERC20Token::total_supply(), 500);
		assert_eq!(ERC20Token::balances(owner), 450);
		assert_eq!(ERC20Token::balances(target), 50);
		assert_eq!(ERC20Token::balances(spender), 0);
	});
}

#[test]
fn should_fail_transfer_if_insufficient_balance() {
    new_test_ext().execute_with(|| {
        let owner = 1;
        let spender = 2;
        // owner has 0 tokens initially
        assert_noop!(
            ERC20Token::transfer(RuntimeOrigin::signed(owner), spender, 100),
            Error::<Test>::ERC20InsufficientBalance
        );
    });
}
