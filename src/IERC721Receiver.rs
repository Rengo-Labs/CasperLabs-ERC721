/**
 * @title ERC721 token receiver interface
 * @dev Interface for any contract that wants to support safeTransfers
 * from ERC721 asset contracts.
*/

use casper_types::
{
    account::AccountHash,
    bytesrepr::{Bytes},U256,
};

#[allow(non_snake_case)]
#[allow(unused)]

pub trait IERC721Receiver {
    //function onERC721Received( address operator, address from, uint256 tokenId, bytes calldata data) external returns (bytes4);

    // https://ethereum.stackexchange.com/questions/52989/what-is-calldata
    // http://attuneww.com/wp-content/uploads/2018/09/Getting-Started-with-Solidity.pdf
    fn onERC721Received(
        operator: AccountHash,
        from: AccountHash,
        tokenId: U256,
        data: Bytes,
    ) -> i32;
}
