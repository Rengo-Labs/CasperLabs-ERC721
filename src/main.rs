/*
#![cfg_attr(
    not(target_arch = "wasm32"),
    crate_type = "target arch should be wasm32"
)]
*/

#![no_main]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(non_snake_case)]

extern crate alloc;
use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::String,
};
use core::convert::TryInto;

use contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

use types::{
    account::AccountHash,
    bytesrepr::{ToBytes},
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys},
    CLType, CLTyped, CLValue, Parameter, U256,
};


// Bringing files in sub-folders into scope
//mod IERC721;
//mod IERC721Receiver;
mod utils;
//mod extensions;

// importing functions from the fules
use utils::Mappings::{get_key, set_key};
//use utils::introspection::ERC165::IERC165;                  // import IERC165 trait from utils>introspection
//use extensions::IERC721Metadata::IERC721Metadata;           // importing trait from extensions>IERC721Metadata


#[no_mangle]
pub extern "C" fn name() {
    let val: String = get_key("_name");
    ret(val)
}

#[no_mangle]
pub extern "C" fn symbol() {
    let val: String = get_key("_symbol");
    ret(val)
}

#[no_mangle]
pub extern "C" fn totalSupply() {
    let val: U256 = get_key("_totalSupply");
    ret(val)
}

#[no_mangle]
pub extern "C" fn balanceOf() {
    let account: AccountHash = runtime::get_named_arg("account");
    let val: U256 = get_key(&balance_key(&account));
    ret(val)
}

#[no_mangle]
pub extern "C" fn ownerOf() {
    let tokenId: U256 = runtime::get_named_arg("tokenId");
    let val: AccountHash = get_key(&owner_key(tokenId));
    ret(val)
}

#[no_mangle]
pub extern "C" fn approve() {

    let tokenId:U256 = runtime::get_named_arg("tokenId");
    let to:AccountHash = runtime::get_named_arg("to");
    let owner:AccountHash = get_key(&owner_key(tokenId));

    if owner == to {
        return
    }

    if(!(owner == runtime::get_caller())){       // if owner is not the owner of token id
        return
    }

    _approve(tokenId, to);
}

#[no_mangle]
pub extern "C" fn getApproved() 
{
    let tokenId:U256 = runtime::get_named_arg("tokenId");
    let val: AccountHash = get_key(&tokenApproval_key(tokenId));
    ret(val);
}

#[no_mangle]
pub extern "C" fn setApprovalForAll() 
{
    // Approve or remove `operator` as an operator for the caller.
    let caller:AccountHash = runtime::get_caller();
    let operator:AccountHash = runtime::get_named_arg("operator");
    let approve:bool = runtime::get_named_arg("approved");

    if(operator == caller){
        return
    }

    _setApprovalForAll(caller, operator, approve)
}


#[no_mangle]
pub extern "C" fn isApprovedForAll(){

    // Returns if the `operator` is allowed to manage all of the assets of `owner`.

    let owner:AccountHash = runtime::get_named_arg("owner");
    let operator:AccountHash = runtime::get_named_arg("operator");

    _isApprovedForAll(owner, operator);
}


// All session code must have a `call` entrypoint.
#[no_mangle]
pub extern "C" fn call() 
{
    let tokenName: String = runtime::get_named_arg("tokenName");
    let tokenSymbol: String = runtime::get_named_arg("tokenSymbol");
    let tokenTotalSupply: U256 = runtime::get_named_arg("tokenTotalSupply");
    let token_ids: Vec<U256> = runtime::get_named_arg("tokenIds");

    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        String::from("name"),
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        String::from("symbol"),
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        String::from("totalSupply"),
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        String::from("balanceOf"),
        vec![Parameter::new("account", AccountHash::cl_type())],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        String::from("ownerOf"),
        vec![Parameter::new("tokenId", CLType::U256)],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        String::from("approve"),
        vec![
            Parameter::new("to", AccountHash::cl_type()),
            Parameter::new("tokenId", CLType::U256)
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        String::from("getApproved"), 
        vec![Parameter::new("tokenId", CLType::U256)],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        String::from("setApprovalForAll"), 
        vec![
            Parameter::new("operator", AccountHash::cl_type()),
            Parameter::new("approved", bool::cl_type()),
            ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    
    let mut named_keys = NamedKeys::new();
    named_keys.insert("_name".to_string(), storage::new_uref(tokenName).into());
    named_keys.insert("_symbol".to_string(), storage::new_uref(tokenSymbol).into());
    named_keys.insert("_totalSupply".to_string(), storage::new_uref(tokenTotalSupply).into());
    //named_keys.insert("_tokenIds".to_string(), storage::new_uref(token_ids).into());
    

    // set initial token balance
    named_keys.insert(
        balance_key(&runtime::get_caller()),
        storage::new_uref(tokenTotalSupply).into(),
    );

    // set token ids
    {   
        let owner: AccountHash =  runtime::get_caller();
        //let ids:Vec<U256> = get_key("_tokenIds");
        for id in token_ids                                            // iterate by value
        {
            //set_key(&tokenApproved_key(id), owner);
            named_keys.insert(owner_key(id), storage::new_uref(owner).into());
        }
    }
    
    let (contract_hash, _) =
        storage::new_locked_contract(entry_points, Some(named_keys), None, None);
    runtime::put_key("ERC721", contract_hash.into());
    runtime::put_key("ERC721_hash", storage::new_uref(contract_hash).into());
}

fn _approve(tokenId: U256, to: AccountHash) {
    set_key(&tokenApproval_key(tokenId), to);
}

pub fn _setApprovalForAll(owner:AccountHash, operator:AccountHash , approve:bool) {
    set_key(&operatorApprovals_key(owner, operator), approve);
}

pub fn _isApprovedForAll(owner: AccountHash, operator: AccountHash) {
    let val:bool = get_key(&operatorApprovals_key(owner, operator));
    ret(val);
}

fn ret<T: CLTyped + ToBytes>(value: T) {
    runtime::ret(CLValue::from_t(value).unwrap_or_revert())
}

pub fn balance_key(account: &AccountHash) -> String {
    format!("_balances_{}", account)
}

pub fn owner_key(tokenId: U256) -> String {
    format!("_owner_{}", tokenId)
}

pub fn tokenApproval_key(tokenId: U256) -> String {
    format!("_tokenApprovals_{}", tokenId)
}

pub fn operatorApprovals_key(owner: AccountHash, operator: AccountHash) -> String {
    format!("operatorApprovals_{}_{}", owner, operator)
}

// Need to figure out how the key values pairs are set. For example, in balanceOf() method, where did we set the key???
