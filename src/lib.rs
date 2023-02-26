#![doc = include_str!("../README.md")]

/// # Examples
/// `ESPNOW.parse` takes an u8 array and returns a Result-Tupel of the espnow header and the
/// remaining data
///
/// ```rust
/// assert_eq!( 2 + 2, 4)
/// ```

#[derive(Debug)]
pub struct ESPNOW {
    // category code, set to 127 (vendorspecific)
    pub catcode: u8,

    // organisation identifier, set to 0x18FE34
    pub orga: [u8; 3],

    // padding to prevent replay/relay attacks
    pub padding: [u8; 4],

    // element id, set to 221 (vendorspecific)
    pub element_id: u8,

    // length of orgid, ptype, version and following data
    pub length: u8,

    // second orga code (equal to first)
    pub orga2: [u8; 3],

    // packet type, set to 0x04
    pub ptype: u8,

    // version of esp-now
    pub version: u8,
}

impl ESPNOW {
    /// Returns a result of an espnow packet
    ///
    /// # Arguments
    /// * `data` - Binary data
    pub fn parse(data: &[u8]) -> Result<(ESPNOW, &[u8]), &str> {
        if data.len() > 16 {
            let packet = ESPNOW {
                catcode: data[0],
                orga: data[1..4].try_into().expect("valid slice size"),
                padding: data[4..8].try_into().expect("valid slice size"),
                element_id: data[8],
                length: data[9],
                orga2: data[10..13].try_into().expect("valid slice size"),
                ptype: data[13],
                version: data[14],
            };

            if packet.is_valid() {
                return Ok({
                    // -5 bytes for orgid (3), ptype (1) and version (1)
                    let end: usize = usize::from(10 + packet.length);

                    (packet, &data[16..end])
                });
            }
        }
        Err("not an espnow packet")
    }

    /// Returns true if the data is a valid ESP-now packet
    fn is_valid(&self) -> bool {
        self.ptype == 0x04
            && self.orga == self.orga2
            && self.orga == [0x18, 0xfe, 0x34]
            && self.catcode == 127
            && self.element_id == 221
    }
}

#[cfg(test)]
mod tests {
    use super::ESPNOW;

    // test if example packet can be parsed
    #[test]
    fn happy_path() -> Result<(), String> {
        let data: [u8; 83] = [
            0xD0, 0x0, 0x0, 0x0, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x10, 0x52, 0x1C, 0x67, 0xD9,
            0xC4, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xC0, 0x1, 0x7F, 0x18, 0xFE, 0x34, 0xD, 0x5F,
            0x70, 0x4D, 0xDD, 0x31, 0x18, 0xFE, 0x34, 0x4, 0x1, 0x54, 0x48, 0x49, 0x53, 0x20, 0x49,
            0x53, 0x20, 0x41, 0x20, 0x43, 0x48, 0x41, 0x52, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x3, 0x0, 0x0, 0x0, 0x9A, 0x99, 0x99,
            0x3F, 0x0, 0x0, 0x0, 0x0,
        ];
        ESPNOW::parse(&data)?;
        Ok(())
    }
}
