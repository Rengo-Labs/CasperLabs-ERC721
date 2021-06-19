/**
 * @title ERC-721 Non-Fungible Token Standard, optional metadata extension
 * @dev See https://eips.ethereum.org/EIPS/eip-721
 */

#[allow(non_snake_case)]
#[allow(unused)]

pub trait IERC721Metadata {
    // function name() external view returns (string memory);
    fn name() -> String;

    // function symbol() external view returns (string memory);
    fn symbol() -> String;

    // function tokenURI(uint256 tokenId) external view returns (string memory);
    fn tokenURI() -> String;
}
