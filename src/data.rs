use std::io;

use crate::errs::ChainXResult;
use csv::ReaderBuilder;

#[derive(Debug)]
pub struct AddressInfo {
    pub pair: String,
    pub address: String,
    pub asset_name: String,
    pub asset_type: String,
    pub market_hours: String,
}

const ADDRESSE_INFOS: &'static str = include_str!("../data/addresses.csv");

pub fn get_addresses() -> ChainXResult<Vec<AddressInfo>> {
    let reader = io::BufReader::new(io::Cursor::new(ADDRESSE_INFOS));
    let mut reader = ReaderBuilder::new().from_reader(reader);
    let mut addresses: Vec<AddressInfo> = Vec::new();

    for result in reader.records() {
        let record = result?;
        let pair = record.get(0).unwrap().to_string();
        let address = record.get(1).unwrap().to_string();
        let asset_name = record.get(2).unwrap().to_string();
        let asset_type = record.get(3).unwrap().to_string();
        let market_hours = record.get(4).unwrap().to_string();

        let address_info = AddressInfo {
            pair,
            address,
            asset_name,
            asset_type,
            market_hours,
        };

        addresses.push(address_info);
    }

    Ok(addresses)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        //test load addresses
        let addrs = get_addresses().unwrap();
        println!("{:#?}", addrs);
    }
}
