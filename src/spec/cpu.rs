use crate::spec::register::{Register};
use crate::dasm::{Disassembler, InstructionData};
use crate::spec::clock::Clock;
use crate::spec::mmu::MMU;

pub struct CPU {
    registers: Register,
    mmu: MMU
}

pub enum Error {
    Default(String),
    InitializationError
}


impl CPU {

    fn fetch(&self) -> u8 {
        unimplemented!()
    }

    fn execute(&mut self, instruction_data : &InstructionData) -> Clock {
        unimplemented!()
    }

    pub fn new(rom: &[u8]) -> Result<CPU, Error> {
        let mmu = match MMU::new(rom) {
            Ok(mmu) => mmu,
            _ => return Err(Error::InitializationError)
        };

        Ok(
            CPU {
                mmu,
                registers: Register::new()
            }
        )

    }

    pub fn tick(&mut self) -> Result<(), Error>{
        let buf: Vec<u8> = Vec::new();
        let opcode = self.fetch();

        let data = match Disassembler::get_instruction_data(
            &opcode,
            &self.registers.pc,
            &buf
        ) {
            Ok(data) => data,
            Err(e) => return Err(Error::Default(format!("{:?}", e)))
        };

        println!("{}", data);


        let timing_info = self.execute(&data);

        Ok(())
    }

}
