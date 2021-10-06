#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    ensure,
    decl_error, 
    decl_module, 
    decl_storage,
    dispatch::{
        DispatchResult, 
        DispatchError, 
        Vec,
    },
};
use frame_system::{
    ensure_signed,
};

use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
};

#[derive(Encode, Decode, Clone, Default, RuntimeDebug)]
pub struct Dog<AccountId> {
	pub owner: AccountId,
	pub id: u32,
	pub age: u8,
}

impl<AccountId> Dog<AccountId> {
	pub fn new(owner: AccountId, id: u32, age: u8) -> Self {
		Dog {
			owner,
			id,
			age,
		}
	}
}

pub trait Config: frame_system::Config {}

decl_storage! {
    trait Store for Module<T: Config> as CarbonCredits {
		DogById
			get(fn dog_by_id):
			map hasher(blake2_128_concat) u32 => Option<Dog<T::AccountId>>;
        LastID: u32;
    }
}

decl_error! {
    pub enum Error for Module<T: Config> {

	}
        // CatErr
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
		#[weight = 10_000]
        pub fn create_dog(origin, age: u8) -> DispatchResult {
            let caller = ensure_signed(origin)?;
			let new_id = LastID::get() + 1;
			let new_project = Dog::<<T as frame_system::Config>::AccountId>::new(caller, new_id, age);
			<DogById<T>>::insert(new_id, new_project);
			LastID::mutate(|x| *x = x.checked_add(1).unwrap());
            Ok(())
        }
    }
}
