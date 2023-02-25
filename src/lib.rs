/// # Examples
/// `ESPNOW.parse` takes an u8 array and returns a Result-Tupel of the espnow header and the
/// remaining data
///
/// ```rust
/// assert_eq!( 2 + 2, 4)
/// ```

#[derive(Debug)]
pub struct ESPNOW {
    // MAC header
    mac: [u8; 24],

    // category code, set to 127 (vendorspecific)
    catcode: u8,

    // organisation identifier, set to 0x18FE34
    orga: [u8; 3],

    // padding to prevent replay/relay attacks
    padding: [u8; 4],

    // element id, set to 221 (vendorspecific)
    element_id: u8,

    // length of orgid, ptype, version and following data
    length: u8,

    // second orga code (equal to first)
    orga2: [u8; 3],

    // packet type, set to 0x04
    ptype: u8,

    // version of esp-now
    version: u8,
}

impl ESPNOW {
    pub fn parse(data: &[u8]) -> Result<(ESPNOW, &[u8]), &str> {
        if data.len() > 38 {
            let packet = ESPNOW {
                mac: data[..24].try_into().expect("valid slice size"),
                catcode: data[24],
                orga: data[25..28].try_into().expect("valid slice size"),
                padding: data[28..32].try_into().expect("valid slice size"),
                element_id: data[32],
                length: data[33],
                orga2: data[34..37].try_into().expect("valid slice size"),
                ptype: data[37],
                version: data[38],
            };

            if packet.isValid() {
                return Ok({
                    // -5 bytes for orgid (3), ptype (1) and version (1)
                    let end: usize = usize::from(39 - 5 + packet.length);

                    (packet, &data[38..end])
                });
            }
        }
        Err("not an espnow packet")
    }

    fn isValid(&self) -> bool {
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
