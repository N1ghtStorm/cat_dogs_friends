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
    traits::{
		Currency, OnUnbalanced, TryDrop, StoredMap,
		WithdrawReasons, LockIdentifier, LockableCurrency, ExistenceRequirement,
		Imbalance, SignedImbalance, ReservableCurrency, Get, ExistenceRequirement::KeepAlive,
		ExistenceRequirement::AllowDeath, BalanceStatus as Status,
	}
};
use frame_system::{
    ensure_signed,
};
// use sp_runtime::{
// 	RuntimeDebug, DispatchResult, DispatchError,
// 	traits::{
// 		Zero, AtLeast32BitUnsigned, StaticLookup, CheckedAdd, CheckedSub,
// 		MaybeSerializeDeserialize, Saturating, Bounded, StoredMapError,
// 	},
// };
use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::{
        RuntimeDebug,
        traits::{
            Zero, AtLeast32BitUnsigned, StaticLookup, CheckedAdd, CheckedSub,
            MaybeSerializeDeserialize, Saturating, Bounded, StoredMapError,
        },
    },
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

use pallet_balances;

pub trait Config: frame_system::Config + pallet_balances::Config {
    // type Balance = u32;
    // type Balance = u32;
    // // type DustRemoval: OnUnbalanced<NegativeImbalance<Self, I>>;

    // /// The overarching event type.
    // type Event: From<Event<Self, I>> + IsType<<Self as frame_system::Config>::Event>;

    // /// The minimum amount required to keep an account open.
    // // #[pallet::constant]
    // type ExistentialDeposit: Get<Self::Balance>;

    // /// The means of storing the balances of an account.
    // type AccountStore: StoredMap<Self::AccountId, AccountData<Self::Balance>>;

    // /// Weight information for extrinsics in this pallet.
    // type WeightInfo: WeightInfo;

    // /// The maximum number of locks that should exist on an account.
    // /// Not strictly enforced, but used for weight estimation.
    // type MaxLocks: Get<u64>;
}

// pub trait Config: frame_system::Config  {

// }

decl_storage! {
    trait Store for Module<T: Config> as CarbonCredits {
		DogById
			get(fn dog_by_id):
			map hasher(blake2_128_concat) u32 => Option<Dog<T::AccountId>>;

        FileById
			get(fn file_by_id):
			map hasher(blake2_128_concat) u32 => Vec<u8>;
        LastID: u32;
        LastFileID: u32;
    }
}

decl_error! {
    pub enum Error for Module<T: Config> {
        SSSSS
	}
        // CatErr
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;

		#[weight = 10_000]
        pub fn create_dog(origin, age: u8) -> DispatchResult {
            let caller = ensure_signed(origin)?;
			let new_id = LastID::get() + 1;
			let new_project = Dog::<<T as frame_system::Config>::AccountId>::new(caller, new_id, age);
			<DogById<T>>::insert(new_id, new_project);
			LastID::mutate(|x| *x = x.checked_add(1).unwrap());
            Ok(())
        }


        #[weight = 1]
        pub fn buy_dog(origin, dest: <T::Lookup as StaticLookup>::Source, value: T::Balance, file: Vec<u8>) -> DispatchResult {
            let caller = ensure_signed(origin.clone())?;
            if let Err(_) = pallet_balances::Module::<T>::transfer(origin, dest, value){
                ensure!(false,  Error::<T>::SSSSS);
            }
            let new_id = LastFileID::get() + 1;
            <FileById>::insert(new_id, file);
            // let a = std::collections::Vec::with_capacity(100000000000);
            Ok(())
        }
    }
}
