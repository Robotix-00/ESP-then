extern crate pnet;
extern crate radiotap;
use radiotap::{Radiotap};

use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::{Packet, MutablePacket};
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket};

use std::env;

#[derive(Debug)]
struct ESPNOW {
        mac: [u8; 24],
        catcode: u8,
        orga: [u8; 3],
        padding: [u8; 4],
        element_id: u8,
        length: u8,
        orga2: [u8; 3],
        ptype: u8,
        version: u8,
}

impl ESPNOW {
    pub fn parse(data: &[u8]) -> Option<(ESPNOW, &[u8])> {
        if(data.len() > 38) {
            Some((ESPNOW {
               mac: data[..24].try_into().unwrap(),
               catcode: data[24],
               orga: data[25..28].try_into().unwrap(),
               padding: data[28..32].try_into().unwrap(),
               element_id: data[32],
               length: data[33],
               orga2: data[34..37].try_into().unwrap(),
               ptype: data[37],
               version: data[38],
            }, &data[38..]))
        } else {
            None
        }
    }
}

fn main() {
    let data = [ 0x0, 0x0, 0x29, 0x0, 0xee, 0x48, 0x0, 0xa0, 0xa0, 0x8, 0x0, 0xa0, 0xa0, 0x8, 0x0, 0x0, 0x10, 0x2, 0x6c, 0x9, 0xa0, 0x0, 0xd0, 0x0, 0x64, 0x0, 0x0, 0x0, 0x0, 0x34, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x2, 0xd0, 0x0, 0x0, 0x0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x10, 0x52, 0x1c, 0x67, 0xd9, 0xc4, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xc0, 0x1, 0x7f, 0x18, 0xfe, 0x34, 0xd, 0x5f, 0x70, 0x4d, 0xdd, 0x31, 0x18, 0xfe, 0x34, 0x4, 0x1, 0x54, 0x48, 0x49, 0x53, 0x20, 0x49, 0x53, 0x20, 0x41, 0x20, 0x43, 0x48, 0x41, 0x52, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x3, 0x0, 0x0, 0x0, 0x9a, 0x99, 0x99, 0x3f, 0x0, 0x0, 0x0, 0x0, 0xcb, 0xe5, 0xca, 0x52];
    let (packet, ether) = Radiotap::parse(&data).unwrap();

    let (esp, data) = ESPNOW::parse(ether).unwrap();
    println!("{:?}", esp);
    println!("{:?}", data);

}

// Invoke as echo <interface name>
fn listen() {
    let interface_name = env::args().nth(1).unwrap();
    let interface_names_match =
        |iface: &NetworkInterface| iface.name == interface_name;

    // Find the network interface with the provided name
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter()
                              .filter(interface_names_match)
                              .next()
                              .unwrap();

    // Create a new channel, dealing with layer 2 packets
    let (mut tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("An error occurred when creating the datalink channel: {}", e)
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                // let packet = Radiotap::from_bytes(&packet).unwrap();
                println!("{:X?}", packet);
            },
            Err(e) => {
                // If an error occurs, we can handle it here
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}
