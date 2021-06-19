#[allow(non_snake_case)]
#[allow(unused)]

use core::convert::TryInto;

use contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

use types::{
    bytesrepr::{FromBytes, ToBytes}, CLTyped, U256,
};

pub fn get_key<T: FromBytes + CLTyped + Default>(name: &str) -> T {
    match runtime::get_key(name) {
        None => Default::default(),
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            storage::read(key).unwrap_or_revert().unwrap_or_revert()
        }
    }
}

pub fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}

// Helper functions
/*
fn test(){
    let from: AccountHash = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap().to_account_hash();
    let flag:bool = false;
    let result = generate_nestedKey_differentType(from, flag);
}

pub fn generate_key_sameType<T: std::fmt::Display>(key: T, value: T) -> String {
    format!("{}_{}", key, value)
}

pub fn generate_key_differentType<T: std::fmt::Display, U: std::fmt::Display>(
    key: T,
    value: U,
) -> String {
    format!("{}_{}", key, value)
}
*/




pub fn parse_key(pair: String) -> (String, String) {
    let pair: Vec<&str> = pair.split("_").collect();

    if pair.len() == 2 {
        (String::from(pair[0]), String::from(pair[1]))
    } else {
        (String::from(""), String::from(""))
    }
}
