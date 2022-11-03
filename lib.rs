// Do not include this pallet in the std library until I explicitly say: include it in the std library of Rust
#![cfg_attr(not(feature = "std"), no_std)]

#![allow(clippy::unused_unit)]
//! A pallet to demonstrate usage of a simple storage map
//!
//! Storage maps map a key type to a value type. The hasher used to hash the key can be customized.
//! This pallet uses the `blake2_128_concat` hasher. This is a good default hasher.

// Want to include all the functions of the pallet
pub use pallet::*;

#[cfg(test)]
mod tests;

// When you compile the code this macro will include all the modules, traits, structs below
#[frame_support::pallet]
// construct runtime looks at this specific file
pub mod pallet {

	// frame is a framework which will provide us with a bunch of packages
	// provide you with lots of traits which provide you with types you will be including in your pallets
	// frame_support is a package, name of their module is dispatch, which has a trait DispathResultWithPostInfo
	//
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*, traits::{Currency, ReservableCurrency}};
	// pallet_prelude gives you all different types, i.e. when dealing with blockchain based code always see there is an accountid
	// every account id could be different on each chain -- it is a generic type which is bounded by various other types (i.e. should not be
	// greater than length of 16, 32 etc. all of these pre configurations -- if you want to utilize that you would bound your accountId from the 
	// pre decided account id types from the prelude)
	use frame_system::pallet_prelude::*;
	use sp_runtime::print;


	//AccountOf type coming from frame_system
	type AccountOf<T> = <T as frame_system::Config>::AccountId;
	// type BalanceOf from frame_system which provides the type urrency
	type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountOf<T>>>::Balance;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		// Have to include event type in our configuration trait\
		// From is a trait, self is my own event (the enum event below) and is also of the type frame_system::Config
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: ReservableCurrency<Self::AccountId>;


	}

	#[derive(Encode, Decode, Default, Debug)]
	pub struct AssetDetails<BalanceOf> {
		asset_name: Vec<u8>,
		asset_number: u32,
		asset_cost: BalanceOf,
	}
	// Telling runtime want to incude Event type so look for this type of parameter as well to include
	#[pallet::event]
	// When talking to the node via the pallet, need to use this syntax below	 
	// Telling code that i am calling this particular event (generate_deposit), which is a function which would call from super
	
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	// Event is going to be an enum type with all the variants below
	// this Config is the same as the config above 
	pub enum Event<T:Config> {
		// since we called this T:Config we can now call AccountId which comes from frame_system::Config 
		HelloValueStored(u32, T::AccountId),

	}
	// this needs to be included in the runtime
	#[pallet::storage]
	// fn get_value is the function we are writing below
	#[pallet::getter(fn get_value)]
	pub type GetValue<T> = StorageValue<_, u32, ValueQuery>;
	

	#[pallet::storage]
	// fn get_value is the function we are writing below
	#[pallet::getter(fn get_reserved_balance)]

	pub type GetReservedBalance<T> = StorageMap<_, Blake2_128Concat, AccountOf<T>, BalanceOf<T>, ValueQuery>;


	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// This used to be decl module! macro but now it is an attribute instead
	#[pallet::call] //function call being made to the chain
	impl<T: Config> Pallet<T> {
		/// Increase the value associated with a particular key
		/// This is the equivalent to gas fee
		#[pallet::weight(10_000)]

		//OriginFor<T> is the person calling this function and going to pay the gas fee
		// DispatchResultWithPostInfo: indicates with an ok or error that if function compiles return that ok value
		// if there is an error then throw and error
		pub fn set_value(origin: OriginFor<T>, value: u32) -> DispatchResultWithPostInfo {
			// Ensure that the caller is a regular keypair account
			// ensure_signed is a function coming from frame support, which ensures that this origin is of the type AccountId
			let caller = ensure_signed(origin)?;
			
			//T is Config and we are calling Currency type from it and then reserved_balance fn from that
			let reserve_balance_of_caller = T::Currency::reserved_balance(&caller.clone());

			// Print a message	
			print("Hello World");
			// Inspecting a variable as well
			debug::info!("Request sent by: {:?}", caller);

			// calling the event
			Self::deposit_event(Event::HelloValueStored(value.clone(), caller.clone()));
			// calling GetValue type with the value set by the user
			GetValue::<T>::put(value.clone());
			// Putting reservable_balance into the storage map
			GetReservedBalance::<T>::insert(caller.clone(), reserve_balance_of_caller.clone());

			// Indicate that this call succeeded
			Ok(().into())
		}
	}
}
