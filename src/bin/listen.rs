#![allow(unused)]
use espthen::EspNowPacket;
use espthen::MacPacket;

use std::env;

extern crate pnet;
use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::Packet;

extern crate radiotap;
use radiotap::Radiotap;

// Invoke as echo <interface name>
fn main() {
    let interface_name = env::args().nth(1).unwrap();
    let interface_names_match = |iface: &NetworkInterface| iface.name == interface_name;

    // Find the network interface with the provided name
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .filter(interface_names_match)
        .next()
        .unwrap();

    // Create a new channel, dealing with layer 2 packets
    let (mut tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!(
            "An error occurred when creating the datalink channel: {}",
            e
        ),
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                if let Ok(radio) = Radiotap::from_bytes(&packet) {
                    if let Some(mac) = MacPacket::new(&packet[radio.header.length..]) {
                        if let Some(esp) = EspNowPacket::new(mac.payload()) {
                            if (esp.get_catcode() == 127) {
                                println!("{:?}, {:?}, {:?} {:?}", radio, mac, esp, esp.payload());
                            }
                        }
                    }
                }
            }
            Err(e) => {
                // If an error occurs, we can handle it here
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}
