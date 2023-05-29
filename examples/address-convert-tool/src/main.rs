
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use hex;
fn main(){
    println!("Hello, world!");

    let Prefix = "OB";
    let evmAddress = "0x99ec891ff6602457efc2c5086c8926f4fe78cebc02a79a55485a6c56aca2b572";

    let result =  convertToOBAddress(Prefix, evmAddress);

    println!("the ob convert result is {}", result);


    ////===============

    let evmAddress = convertToEvmAddress(&result);
    println!("the evm convert result is {}", evmAddress);
}



fn sha256(input: &str) -> String {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    let result = hasher.finish();

    format!("{:x}", result)
}


//OB99ec891ff6602457efc2c5086c8926f4fe78cebc02a79a55485a6c56aca2b5723735
fn convertToOBAddress(prefix: &str, evm_address: &str) -> String {
    let mut address = evm_address.to_string();

    let result = sha256(address.as_str());
    let mut hex = hex::encode(result);
    hex.truncate(4);

    let mut address = prefix.to_string();
    address.push_str(&evm_address[2..]);
    address.push_str(hex.as_str());


    return address;
}

// 0x99ec891ff6602457efc2c5086c8926f4fe78cebc02a79a55485a6c56aca2b572
fn convertToEvmAddress(ob_address: &str) -> String {

    let mut address = ob_address[2..].to_string();
    let evmPrefix = String::from("0x");
    address.insert_str(0, evmPrefix.as_str());
    address.truncate(address.len()-4);

    let result = sha256(address.as_str());
    let mut hex = hex::encode(result);
    hex.truncate(4);


    let verifyCode = ob_address[ob_address.len()-4..].to_string();
    assert_eq!(verifyCode, hex);


    return address.to_string();
}
