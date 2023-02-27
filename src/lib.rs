#![allow(rustdoc::bare_urls)]
#![doc = include_str!("./../README.md")]

// use pnet_macros::packet;
// use pnet_macros_support::types::*;
use pnet_macros::packet;
use pnet_macros_support::types::u32le;
use pnet_macros_support::packet::PrimitiveValues;


/// # Examples
/// `ESPNOW` is a representation of an espnow-header packet
///
/// ```rust
/// assert_eq!( 2 + 2, 4)
/// ```

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
pub struct OrganisationCode(u8, u8, u8);

impl OrganisationCode {
    pub fn new(val1: u8, val2: u8, val3: u8) -> OrganisationCode {
        OrganisationCode(val1, val2, val3)
    }
}

impl PrimitiveValues for OrganisationCode {
    type T = (u8,u8,u8);
    fn to_primitive_values(&self) -> (u8,u8,u8) {
        (self.0,self.1, self.2)
    }
}



#[packet]
pub struct EspNow {
    // category code, set to 127 (vendorspecific)
    pub catcode: u8,

    // organisation identifier, set to 0x18FE34
    #[construct_with(u8, u8, u8)]
    pub orga: OrganisationCode,

    // padding to prevent replay/relay attacks
    // #[construct_with(u8, u8, u8, u8)]
    pub padding: u32le,

    // element id, set to 221 (vendorspecific)
    pub element_id: u8,

    // length of orgid, ptype, version and following data
    pub length: u8,

    // second orga code (equal to first)
    #[construct_with(u8, u8, u8)]
    pub orga2: OrganisationCode,

    // packet type, set to 0x04
    pub ptype: u8,

    // version of esp-now
    pub version: u8,

    #[length = "length - 5"]
    #[payload]
    pub data: Vec<u8>,


    pub mac: u32le,
}

impl EspNow {
    /// Returns true if the data is a valid ESP-now packet
    pub fn is_valid(&self) -> bool {
        self.ptype == 0x04
            && self.orga == self.orga2
            // && self.orga == 0x18fe34
            && self.catcode == 127
            && self.element_id == 221
    }
}

#[cfg(test)]
mod tests {
    use super::EspNowPacket;

    // test if example packet can be parsed
    #[test]
    fn happy_path() {
        let data: &[u8] = &[
            // 0xD0, 0x0, 0x0, 0x0, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x10, 0x52, 0x1C, 0x67, 0xD9,
            // 0xC4, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xC0, 0x1,
            0x7F, 0x18, 0xFE, 0x34, 0xD, 0x5F,
            0x70, 0x4D, 0xDD, 0x31, 0x18, 0xFE, 0x34, 0x4, 0x1, 0x54, 0x48, 0x49, 0x53, 0x20, 0x49,
            0x53, 0x20, 0x41, 0x20, 0x43, 0x48, 0x41, 0x52, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x3, 0x0, 0x0, 0x0, 0x9A, 0x99, 0x99,
            0x3F, 0x0, 0x0, 0x0, 0x0,
        ];
        EspNowPacket::new(data).unwrap();
    }
}
