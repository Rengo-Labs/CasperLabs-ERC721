// interface IERC721 is IERC165
// extends IERC165 - If a type implements IERC721, it also need to implement IERC165

// following are some of the Pending functions
//event Transfer(address indexed from, address indexed to, uint256 indexed tokenId);
//event Approval(address indexed owner, address indexed approved, uint256 indexed tokenId);
//event ApprovalForAll(address indexed owner, address indexed operator, bool approved);

use casper_types::
{
    account::AccountHash,
    bytesrepr::{Bytes},U256,
};

#[allow(non_snake_case)]
#[allow(unused)]

pub trait IERC721 {
    // function balanceOf(address owner) external view returns (uint256 balance);
    fn balanceOf(owner: AccountHash) -> U256;

    // function safeTransferFrom( address from, address to, uint256 tokenId) external;
    // function safeTransferFrom( address from, address to, uint256 tokenId, bytes calldata data) external;
    // Since rust doesn't supports method overloading, passing in additional parameter of type Option for 'calldata'

    fn safeTransferFrom(from: AccountHash, to: AccountHash, tokenId: U256, data: Option<Bytes>);

    // function transferFrom( address from, address to, uint256 tokenId) external;
    fn transferFrom(from: AccountHash, to: AccountHash, tokenId: U256);

    // function approve(address to, uint256 tokenId) external;
    fn approve(to: AccountHash, tokenId: U256);

    // function getApproved(uint256 tokenId) external view returns (address operator);
    fn getApproved(tokenId: U256) -> AccountHash;

    // function setApprovalForAll(address operator, bool _approved) external;
    fn setApprovalForAll(address: AccountHash, _approved: bool);

    // function isApprovedForAll(address owner, address operator) external view returns (bool);
    fn isApprovedForAll(owner: AccountHash, operator: AccountHash) -> bool;
}
