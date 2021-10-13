#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::*;
pub use pallet::*;

#[derive(Encode, Decode)]
pub struct Kitty([u8; 16]);

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    // use frame_support::pallet_prelude::*;
    // pub use pallet::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
    }
    
    #[pallet::storage]
    #[pallet::getter(fn kitties)]
    pub type Kitties<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat, T::AccountId,
        Blake2_128Concat, u32,
        Kitty, OptionQuery
    >;

    #[pallet::storage]
    #[pallet::getter(fn next_kitty_id)]
    pub type NextKittyId<T: Config> = StorageValue< _, u32, ValueQuery>;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);
}