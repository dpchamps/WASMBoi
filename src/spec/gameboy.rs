use crate::spec::cartridge_header::{Cartridge, CartridgeError};
use crate::spec::cpu::{CPU, Error as CpuError};
use crate::spec::mmu::{MMU, Error as MmuError};

pub struct GameBoy {
    cpu: CPU,
    mmu: MMU
}

#[derive(Debug)]
pub enum GameBoyError {
    Unknown,
    Cpu(CpuError),
    Mmu(MmuError),
    Cartridge(CartridgeError)
}

impl Default for GameBoyError {
    fn default() -> Self {
        GameBoyError::Unknown
    }
}

impl From<CpuError> for GameBoyError {
    fn from(e: CpuError) -> Self {
        GameBoyError::Cpu(e)
    }
}

impl From<MmuError> for GameBoyError {
    fn from(e: MmuError) -> Self {
        GameBoyError::Mmu(e)
    }
}

impl From<CartridgeError> for GameBoyError {
    fn from(e: CartridgeError) -> Self {
        GameBoyError::Cartridge(e)
    }
}

impl GameBoy {
    pub fn new(rom: &[u8]) -> Result<GameBoy, GameBoyError> {
        println!("Loading Cartridge Header");
        let cartridge = Cartridge::new(rom)?;
        println!("---\n{}\n---", cartridge);
        println!("Initializing Z80 CPU");
        let cpu = CPU::new()?;
        println!("Initializing MMU");
        let mmu = MMU::new(rom, &cartridge.cartridge_type)?;
        println!("OK");

        Ok(GameBoy {
            cpu,
            mmu
        })
    }

    pub fn cycle() -> Result<(), GameBoyError> {
        Ok(())
    }
}