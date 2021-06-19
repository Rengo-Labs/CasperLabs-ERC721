
use types::U256;

#[allow(non_snake_case)]
#[allow(unused)]

pub fn toString(value: U256) -> String {
    //  First need to check if the following line does the job. If not then the remaining code will do.
    //  let result: String = value.to_string();
    //  result

    if (value == 0.into()) {
        return String::from("0");
    }

    let mut temp: U256 = value;
    let mut digits: u8 = 0;
    while (temp > 0.into()) {
        digits += 1;
        temp = (temp / 10).into();
    }

    let mut buffer: String = String::from("");
    let mut index: usize = 0;
    let size: usize = digits.into();

    while (index < size) {
        let ch: u8 = 48 + value.byte(index); // get each bytes from first to end, convert it to character and push into string
        buffer.push(ch as char);

        index += 1;
    }

    buffer
}

// function toHexString(uint256 value) internal pure returns (string memory)
pub fn toHexString(value: U256) -> String {
    // LowerHex is implemented for U256 thus following calls will work.

    //format!("{:x}", value)                // a59
    format!("{:#x}", value) // 0xa59
}
