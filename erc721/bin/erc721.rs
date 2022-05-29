#![no_main]
#![no_std]

extern crate alloc;

use alloc::{boxed::Box, collections::BTreeSet, format, string::String, vec, vec::Vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::{Bytes, ToBytes},
    runtime_args, CLType, CLTyped, CLValue, ContractHash, ContractPackageHash, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
};
use contract_utils::{ContractContext, OnChainContractStorage};
use erc721_crate::ERC721;

#[derive(Default)]
struct Erc721(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for Erc721 {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl ERC721<OnChainContractStorage> for Erc721 {}
impl Erc721 {
    fn constructor(
        &mut self,
        name: String,
        symbol: String,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        ERC721::init(self, name, symbol, Key::from(contract_hash), package_hash);
    }
}

#[no_mangle]
fn constructor() {
    let name: String = runtime::get_named_arg("name");
    let symbol: String = runtime::get_named_arg("symbol");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    Erc721::default().constructor(name, symbol, contract_hash, package_hash);
}
#[no_mangle]
fn balance_of() {
    let owner: Key = runtime::get_named_arg("owner");
    let ret = Erc721::default().balance_of(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn owner_of() {
    let token_id: U256 = runtime::get_named_arg("token_id");
    Erc721::default().owner_of(token_id);
}
#[no_mangle]
fn name() {
    let ret: String = Erc721::default().name();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn symbol() {
    let ret: String = Erc721::default().symbol();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn token_uri() {
    let token_id: U256 = runtime::get_named_arg("token_id");
    let ret: String = Erc721::default().token_uri(token_id);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn approve() {
    let to: Key = runtime::get_named_arg("to");
    let token_id: U256 = runtime::get_named_arg("token_id");
    Erc721::default().approve(to, token_id);
}
#[no_mangle]
fn get_approved() {
    let token_id: U256 = runtime::get_named_arg("token_id");
    let ret = Erc721::default().get_approved(token_id);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn set_approval_for_all() {
    let operator: Key = runtime::get_named_arg("operator");
    let approved: bool = runtime::get_named_arg("approved");
    Erc721::default().set_approved_for_all(operator, approved);
}
#[no_mangle]
fn is_approved_for_all() {
    let owner: Key = runtime::get_named_arg("owner");
    let operator: Key = runtime::get_named_arg("operator");
    let ret = Erc721::default().is_approved_for_all(owner, operator);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn transfer_from() {
    let from: Key = runtime::get_named_arg("from");
    let to: Key = runtime::get_named_arg("to");
    let token_id: U256 = runtime::get_named_arg("token_id");
    Erc721::default().transfer_from(from, to, token_id);
}
#[no_mangle]
fn safe_transfer_from() {
    let from: Key = runtime::get_named_arg("from");
    let to: Key = runtime::get_named_arg("to");
    let token_id: U256 = runtime::get_named_arg("token_id");
    Erc721::default().safe_transfer_from(from, to, token_id);
}
#[no_mangle]
fn safe_transfer_from_() {
    let from: Key = runtime::get_named_arg("from");
    let to: Key = runtime::get_named_arg("to");
    let token_id: U256 = runtime::get_named_arg("token_id");
    let _data: Bytes = runtime::get_named_arg("_data");
    Erc721::default().safe_transfer_from_(from, to, token_id, _data);
}
#[no_mangle]
fn mint() {
    let to: Key = runtime::get_named_arg("to");
    let token_id: U256 = runtime::get_named_arg("token_id");
    Erc721::default().mint(to, token_id);
}
#[no_mangle]
fn burn() {
    let token_id: U256 = runtime::get_named_arg("token_id");
    Erc721::default().burn(token_id);
}
fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
            Parameter::new("name", String::cl_type()),
            Parameter::new("symbol", String::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "balance_of",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "owner_of",
        vec![Parameter::new("token_id", U256::cl_type())],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "name",
        vec![],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "symbol",
        vec![],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "token_uri",
        vec![Parameter::new("token_id", U256::cl_type())],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "approve",
        vec![
            Parameter::new("to", Key::cl_type()),
            Parameter::new("token_id", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_approved",
        vec![Parameter::new("token_id", U256::cl_type())],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_approval_for_all",
        vec![
            Parameter::new("operator", Key::cl_type()),
            Parameter::new("approved", bool::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "is_approved_for_all",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("operator", Key::cl_type()),
        ],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer_from",
        vec![
            Parameter::new("from", Key::cl_type()),
            Parameter::new("to", Key::cl_type()),
            Parameter::new("token_id", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "safe_transfer_from",
        vec![
            Parameter::new("from", Key::cl_type()),
            Parameter::new("to", Key::cl_type()),
            Parameter::new("token_id", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "safe_transfer_from_",
        vec![
            Parameter::new("from", Key::cl_type()),
            Parameter::new("to", Key::cl_type()),
            Parameter::new("token_id", U256::cl_type()),
            Parameter::new("_data", Bytes::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "mint",
        vec![
            Parameter::new("to", Key::cl_type()),
            Parameter::new("token_id", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "burn",
        vec![Parameter::new("token_id", U256::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}

#[no_mangle]
fn call() {
    // Contract name must be same for all new versions of the contracts
    let contract_name: alloc::string::String = runtime::get_named_arg("contract_name");

    // If this is the first deployment
    if !runtime::has_key(&format!("{}_package_hash", contract_name)) {
        // Build new package with initial a first version of the contract.
        let (package_hash, access_token) = storage::create_contract_package_at_hash();
        let (contract_hash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());
        let name: String = runtime::get_named_arg("name");
        let symbol: String = runtime::get_named_arg("symbol");
        // Prepare constructor args
        let constructor_args = runtime_args! {
            "name" => name,
            "symbol" => symbol,
            "contract_hash" => contract_hash,
            "package_hash"=> package_hash
        };

        // Add the constructor group to the package hash with a single URef.
        let constructor_access: URef =
            storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
                .unwrap_or_revert()
                .pop()
                .unwrap_or_revert();

        // Call the constructor entry point
        let _: () =
            runtime::call_versioned_contract(package_hash, None, "constructor", constructor_args);

        // Remove all URefs from the constructor group, so no one can call it for the second time.
        let mut urefs = BTreeSet::new();
        urefs.insert(constructor_access);
        storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
            .unwrap_or_revert();
        runtime::put_key(
            &format!("{}_package_hash", contract_name),
            package_hash.into(),
        );
        runtime::put_key(
            &format!("{}_package_hash_wrapped", contract_name),
            storage::new_uref(package_hash).into(),
        );
        runtime::put_key(
            &format!("{}_contract_hash", contract_name),
            contract_hash.into(),
        );
        runtime::put_key(
            &format!("{}_contract_hash_wrapped", contract_name),
            storage::new_uref(contract_hash).into(),
        );
        runtime::put_key(
            &format!("{}_package_access_token", contract_name),
            access_token.into(),
        );
    } else {
        // this is a contract upgrade
        let package_hash: ContractPackageHash =
            runtime::get_key(&format!("{}_package_hash", contract_name))
                .unwrap_or_revert()
                .into_hash()
                .unwrap()
                .into();

        let (contract_hash, _): (ContractHash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());

        // update contract hash
        runtime::put_key(
            &format!("{}_contract_hash", contract_name),
            contract_hash.into(),
        );
        runtime::put_key(
            &format!("{}_contract_hash_wrapped", contract_name),
            storage::new_uref(contract_hash).into(),
        );
    }
}
