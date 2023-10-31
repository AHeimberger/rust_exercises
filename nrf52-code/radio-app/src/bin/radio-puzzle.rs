#![deny(unused_must_use)]
#![no_main]
#![no_std]

use core::str;
use cortex_m_rt::entry;
use dk::ieee802154::{Channel, Packet};
use heapless::LinearMap;
// this imports `src/lib.rs`to retrieve our global logger + panicking-behavior
use radio_app as _;

const TEN_MS: u32 = 10_000;

#[entry]
fn main() -> ! {
    let board = dk::init().unwrap();
    let mut radio = board.radio;
    let mut timer = board.timer;

    radio.set_channel(Channel::_25); // <- must match the Dongle's listening channel

    let mut empty_response = Packet::new();
    empty_response.copy_from_slice(b"");
    radio.send(&mut empty_response);
    if radio.recv_timeout(&mut empty_response, &mut timer, TEN_MS).is_ok() {
        defmt::println!(" received: {}", str::from_utf8(&empty_response).expect("response was not valid UTF-8 data"));
    }

    let mut packet = Packet::new();
    let mut my_map = LinearMap::<u8, u8, 128>::new();
    for c in 0..127 {
        let msg = [c as u8];
        packet.copy_from_slice(&msg);
        radio.send(&mut packet);
        if radio.recv_timeout(&mut packet, &mut timer, TEN_MS).is_ok() {
            let letter_send = msg[0];
            let letter_received = packet[0];
            my_map.insert(letter_received, letter_send).expect("dictionary full");
            // defmt::println!("{} : {}",
            //     str::from_utf8(&[letter_received]).expect("response was not valid UTF-8 data"),
            //     str::from_utf8(&[letter_send]).expect("response was not valid UTF-8 data")
            // );
        }
    }
    
    for spot in empty_response.iter_mut() {
        if my_map.contains_key(spot) {
            let letter = my_map.get(&spot).unwrap().clone();
            *spot = letter;
        }
    }

    defmt::println!("plaintext:  {}",str::from_utf8(&empty_response).expect("buffer contains non-UTF-8 data"));
    radio.send(&mut empty_response);
    if radio.recv_timeout(&mut packet, &mut timer, TEN_MS).is_err() {
        defmt::error!("no response or response packet was corrupted");
        dk::exit()
    }

    defmt::println!("Dongle response: {}",str::from_utf8(&packet).expect("response was not UTF-8"));

    dk::exit()
}
