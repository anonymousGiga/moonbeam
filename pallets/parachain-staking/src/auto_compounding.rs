use crate::pallet::{
	AutoCompoundingInfo, CandidateInfo, Config, DelegatorState, Error, Event, Pallet,
};
use frame_support::ensure;
use frame_support::{dispatch::DispatchResultWithPostInfo, RuntimeDebug};
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::Percent;
use sp_std::{vec, vec::Vec};

/// Represents the auto-compounding
#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo, PartialOrd, Ord)]
pub struct AutoCompoundingDelegation<AccountId> {
	pub delegator: AccountId,
	pub value: Percent,
}

impl<AccountId> AutoCompoundingDelegation<AccountId>
where
	AccountId: Eq,
{
	fn new(delegator: AccountId) -> Self {
		AutoCompoundingDelegation {
			delegator,
			value: Percent::zero(),
		}
	}
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo, PartialOrd, Ord)]
pub struct AutoCompounding<AccountId> {
	pub candidate: AccountId,
	pub value: Percent,
	pub delegations: Vec<AutoCompoundingDelegation<AccountId>>,
}

impl<AccountId> AutoCompounding<AccountId>
where
	AccountId: Eq,
{
	pub fn new(candidate: AccountId) -> Self {
		AutoCompounding {
			candidate,
			value: Percent::zero(),
			delegations: vec![],
		}
	}

	pub fn set_delegation_value(&mut self, delegator: AccountId, value: Percent) {
		let maybe_delegation = self
			.delegations
			.iter_mut()
			.find(|entry| entry.delegator == delegator);

		let mut delegation = if let Some(delegation) = maybe_delegation {
			delegation
		} else {
			let new_entry = AutoCompoundingDelegation::new(delegator);
			self.delegations.push(new_entry);
			self.delegations.last_mut().expect("cannot fail; qed")
		};

		delegation.value = value;
	}

	pub fn remove_delegation_value(&mut self, delegator: &AccountId) {
		if let Some(index) = self
			.delegations
			.iter()
			.position(|entry| &entry.delegator == delegator)
		{
			self.delegations.remove(index);
		}
	}
}

impl<T: Config> Pallet<T> {
	pub(crate) fn candidate_set_auto_compounding(
		candidate: T::AccountId,
		value: Percent,
	) -> DispatchResultWithPostInfo {
		ensure!(
			<CandidateInfo<T>>::get(&candidate).is_some(),
			<Error<T>>::CandidateDNE,
		);

		let mut state = <AutoCompoundingInfo<T>>::get(&candidate)
			.unwrap_or_else(|| AutoCompounding::new(candidate.clone()));
		state.value = value;

		<AutoCompoundingInfo<T>>::insert(candidate.clone(), state);
		Self::deposit_event(Event::CandidateAutoCompoundingSet { candidate, value });

		Ok(().into())
	}

	pub(crate) fn delegation_set_auto_compounding(
		candidate: T::AccountId,
		delegator: T::AccountId,
		value: Percent,
	) -> DispatchResultWithPostInfo {
		ensure!(
			<DelegatorState<T>>::get(&delegator)
				.ok_or(<Error<T>>::DelegatorDNE)?
				.delegations
				.0
				.iter()
				.any(|b| b.owner == candidate),
			<Error<T>>::DelegationDNE,
		);

		let mut state = <AutoCompoundingInfo<T>>::get(&candidate)
			.unwrap_or_else(|| AutoCompounding::new(candidate.clone()));

		state.set_delegation_value(delegator.clone(), value);
		<AutoCompoundingInfo<T>>::insert(candidate.clone(), state);
		Self::deposit_event(Event::DelegationAutoCompoundingSet {
			candidate,
			delegator,
			value,
		});

		Ok(().into())
	}

	pub(crate) fn delegation_remove_auto_compounding(
		candidate: &T::AccountId,
		delegator: &T::AccountId,
	) {
		if let Some(mut state) = <AutoCompoundingInfo<T>>::get(candidate) {
			state.remove_delegation_value(delegator);
			<AutoCompoundingInfo<T>>::insert(candidate, state);
		}
	}
}
