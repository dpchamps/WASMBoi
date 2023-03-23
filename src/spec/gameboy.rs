use crate::spec::cartridge_header::{Cartridge, CartridgeError};
use crate::spec::clock::Clock;
use crate::spec::cpu::{Error as CpuError, CPU, TCPU};
use crate::spec::mmu::{Error as MmuError, MMU};
use crate::spec::register::TRegister;

pub struct GameBoy {
    cartridge: Cartridge,
    clock: Clock,
    cpu: CPU,
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
        let cpu = CPU::new()?;
        println!("Initializing MMU");
        let mmu = MMU::new(rom, &cartridge.cartridge_type)?;
        println!("Initializing Clock");
        let clock = Clock::default();
        println!("OK");

        Ok(GameBoy {
            cpu,
            mmu,
            clock,
            cartridge,
        })
    }

    pub fn cycle(&mut self) -> Result<(), GameBoyError> {
        let cycles = self.cpu.tick(&mut self.mmu).map_err(GameBoyError::Cpu)?;
        self.clock.add_cycles(cycles);

        Ok(())
    }

    pub fn start(&mut self) -> Result<(), GameBoyError> {
        self.cpu
            .registers
            .pc
            .set_value(self.cartridge.start_address);
        self.cycle()?;
        self.cycle()?;
        self.cycle()?;
        self.cycle()?;
        Ok(())

        // loop {
        //     self.cycle()?
        // }
    }
}
