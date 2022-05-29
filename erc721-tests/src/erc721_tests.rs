use crate::erc721_instance::ERC721Instance;
use casper_types::{
    account::AccountHash, bytesrepr::ToBytes, gens::contract_arb, runtime_args,system::auction::RuntimeProvider, ContractPackageHash, Key, RuntimeArgs, URef, U256, U512,
};
use test_env::{TestContract, TestEnv};

fn deploy() -> (TestEnv, AccountHash, TestContract, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let instance = ERC721Instance::new(&env, "ERC721", owner, "sa".to_string(), "s".to_string());
    let proxy = ERC721Instance::proxy(
        &env,
        "ERC721PROXY",
        owner,
        Key::Hash(instance.contract_hash()),
    );
    (env, owner, instance, proxy)
}

#[test]
fn test_deploy() {
    let (_, _, _, _) = deploy();
}
#[test]
fn test_mint() {
    let (_, owner, instance, _) = deploy();
    let instance = ERC721Instance::contract_instance(instance);
    let token_id: U256 = 1.into();
    let to: Key = Key::Account(owner);
    instance.mint(owner, to, token_id)
}
#[test]
fn test_burn() {
    let (_, owner, instance, _) = deploy();
    let instance = ERC721Instance::contract_instance(instance);
    let to: Key = Key::Account(owner);
    let token_id: U256 = 1.into();
    instance.mint(owner, to, token_id);
    instance.burn(owner, token_id)
}
#[test]
fn test_approve() {
    let (_, owner, instance, _) = deploy();
    let instance = ERC721Instance::contract_instance(instance);
    let to: Key = Key::Account(owner);
    let token_id: U256 = 1.into();
    instance.mint(owner, to, token_id);
    instance.approve(
        owner,
        Key::from_formatted_str(
            "hash-0000000000000000000000000000000000000000000000020000000000000000".into(),
        )
        .unwrap(),
        token_id,
    );
}
#[test]
fn test_transfer_from() {
    let (_, owner, instance, _) = deploy();
    let instance = ERC721Instance::contract_instance(instance);
    let from: Key = Key::Account(owner);
    let to: Key = Key::Account(owner);
    let token_id: U256 = 1.into();
    instance.mint(owner, to, token_id);
    instance.transfer_from(
        owner,
        from,
        Key::from_formatted_str(
            "hash-0000000000000000000000000000000000000000000000020000000000000000".into(),
        )
        .unwrap(),
        token_id,
    );
}
#[test]
fn test_safe_transfer_from() {
    let (_, owner, instance, _) = deploy();
    let instance = ERC721Instance::contract_instance(instance);
    let from: Key = Key::Account(owner);
    let to: Key = Key::Account(owner);
    let token_id: U256 = 1.into();
    instance.mint(owner, to, token_id);
    instance.safe_transfer_from(
        owner,
        from,
        Key::from_formatted_str(
            "hash-0000000000000000000000000000000000000000000000020000000000000000".into(),
        )
        .unwrap(),
        token_id,
    );
}
#[test]
fn test_balance_of() {
    let (_, owner, _, proxy) = deploy();
    let from: Key = Key::Account(owner);
    let proxy = ERC721Instance::contract_instance(proxy);
    proxy.balance_of(owner, from);
}
#[test]
fn test_name() {
    let (_, owner, _, proxy) = deploy();
    let proxy = ERC721Instance::contract_instance(proxy);
    proxy.name(owner);
    let result: String = proxy.result();
    println!("{:}", result);
}
#[test]
fn test_symbol() {
    let (_, owner, _, proxy) = deploy();
    let proxy = ERC721Instance::contract_instance(proxy);
    proxy.symbol(owner);
    let result: String = proxy.result();
    println!("{:}", result);
}
#[test]
fn test_token_uri() {
    let (_, owner, instance, proxy) = deploy();
    let instance = ERC721Instance::contract_instance(instance);
    let proxy = ERC721Instance::contract_instance(proxy);
    let token_id: U256 = 1.into();
    instance.mint(owner, Key::Account(owner), token_id);
    proxy.token_uri(owner, token_id);
    let result: String = proxy.result();
    println!("{:}", result);
}
