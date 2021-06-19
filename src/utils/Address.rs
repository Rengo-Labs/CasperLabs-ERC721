use crate::Mapping::{balance_key, get_key, set_key};
use casper_types::account::AccountHash;
#[allow(non_snake_case)]
#[allow(unused)]
use casper_types::{bytesrepr::Bytes, U256};

// function isContract(address account) internal view returns (bool)

// function sendValue(address payable recipient, uint256 amount)
fn sendValue(sender: AccountHash, recipient: AccountHash, amount: U256) {
    let sender_key = balance_key(&sender);
    let recipient_key = balance_key(&recipient);

    if (get_key::<U256>(&sender_key) < amount) {
        // insufficient balance
        return;
    }

    let new_sender_balance: U256 = (get_key::<U256>(&sender_key) - amount);
    set_key(&sender_key, new_sender_balance);
    let new_recipient_balance: U256 = (get_key::<U256>(&recipient_key) + amount);
    set_key(&recipient_key, new_recipient_balance);
}

//function functionCall(address target, bytes memory data) internal returns (bytes memory) {
//    return functionCall(target, data, "Address: low-level call failed");
//}
fn functionCall(target: AccountHash, data: Bytes) -> Bytes {
    functionCallWithErrorMsg(target, data, String::from("Address: low-level call failed"))
}

fn functionCallWithErrorMsg(target: AccountHash, data: Bytes, errorMessage: String) -> Bytes {
    functionCallWithValue(target, data, U256::from(0), errorMessage)
}

fn functionCallWithValue(
    target: AccountHash,
    data: Bytes,
    value: U256,
    errorMessage: String,
) -> Bytes {
    Bytes::new() // dummy return to avoid error - need to work on this function
}
