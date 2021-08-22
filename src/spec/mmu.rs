use crate::spec::cartridge_header::Cartridge;
use crate::spec::mmu::Error::CreateError;

pub struct MMU {
    memory: [u8; 0xFFFF],
    cartridge: Cartridge
}

pub enum Error {
    CreateError,
    ReadError,
    WriteError
}

impl MMU {
    pub fn new(game_data: &[u8]) -> Result<MMU, Error> {
        let mut memory = [0; 0xFFFF];
        // println!("Game Data Len: {:#x}", game_data.len());
        let cartridge = match Cartridge::new(game_data) {
            Ok(cart) => cart,
            Err(_) => return Err(CreateError)
        };
        println!("Cart: {}", cartridge);

        for n in 0..game_data.len() {
            memory[n+0xFF] = game_data[n];
        }

        Ok(
            MMU {
                memory,
                cartridge
            }
        )

    }

    pub fn read_byte(address: u16) -> Result<u8, Error>  {
        unimplemented!()
    }

    pub fn read_word(address: u16) -> Result<u16, Error> {
        unimplemented!()
    }

    pub fn write_byte(address: u16, value: u8) -> Result<(), Error> {
        unimplemented!()
    }

    pub fn write_word(address: u16, value: u8) -> Result<(), Error> {
        unimplemented!()
    }
}
