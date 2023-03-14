use crate::spec::cartridge_header::{Cartridge, CartridgeError};
use crate::spec::cpu::{CPUImpl, Error as CpuError, CPU};
use crate::spec::mmu::{Error as MmuError, MMU};

pub struct GameBoy {
    cpu: CPUImpl,
    mmu: MMU,
}

#[derive(Debug)]
pub enum GameBoyError {
    Unknown,
    Cpu(CpuError),
    Mmu(MmuError),
    Cartridge(CartridgeError),
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
        let cpu = CPUImpl::new()?;
        println!("Initializing MMU");
        let mmu = MMU::new(rom, &cartridge.cartridge_type)?;
        println!("OK");

        Ok(GameBoy { cpu, mmu })
    }

    pub fn cycle(&mut self) -> Result<(), GameBoyError> {
        self.cpu
            .tick(&mut self.mmu)
            .map_err(|x| GameBoyError::Cpu(x))
    }

    pub fn start(&mut self) -> Result<(), GameBoyError> {
        loop {
            self.cycle()?
        }
    }
}
