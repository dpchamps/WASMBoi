use std::num::Wrapping;
use crate::spec::hardware_registers::Interrupt;
use crate::spec::mmu::{MMU, Error as MMUError};

const DIV_ADDR: u16 = 0xFF04;
const TIMA_ADDR: u16 = 0xFF05;
const TMA_ADDR: u16 = 0xFF06;
const TAC_ADDR: u16 = 0xFF07;

#[derive(Debug)]
pub enum TimerError {
    MMUError(MMUError)
}

pub enum SpeedMode {
    Single,
    Double
}

impl Default for SpeedMode {
    fn default() -> Self {
        SpeedMode::Single
    }
}

impl SpeedMode {
    pub fn oscillation(&self) -> usize {
        match self {
            SpeedMode::Single => 4194300,
            SpeedMode::Double => 8388600
        }
    }
}

impl From<MMUError> for TimerError{
    fn from(value: MMUError) -> Self {
        TimerError::MMUError(value)
    }
}

#[derive(Default)]
pub struct Clock {
    cycles: usize,
    speed_mode: SpeedMode,
    div_cycles: usize,
    tima_cycles: usize
}

#[derive(Debug)]
pub struct TimerControl {
    enabled: bool,
    clock_select: usize
}

impl From<u8> for TimerControl {
    fn from(value: u8) -> Self {
        let enabled = ((value & 0b100) >> 2) == 1;
        let clock_select = match value & 0b011 {
            00 => 1024,
            01 => 16,
            10 => 64,
            11 => 256,
            _ => unreachable!()
        };

        Self {
            enabled,
            clock_select
        }
    }
}

impl Clock {
    pub fn reset(&mut self) {
        self.cycles = 0;
    }

    pub fn add_cycles(&mut self, cycles: u8) {
        self.cycles += cycles as usize;
    }

    pub fn t_cycles(&self) -> usize {
        self.cycles * 4
    }

    pub fn finalize_cycle(&mut self, mmu: &mut MMU) -> Result<usize, TimerError> {
        let current_cpu_frequency = self.speed_mode.oscillation();

        self.update_tima(mmu)?;
        self.update_div(mmu)?;

        let final_cycles = self.cycles;
        self.cycles = 0;

        Ok(final_cycles)
    }

    fn update_tima(&mut self, mmu: &mut MMU) -> Result<(), TimerError> {
        let timer_control = TimerControl::from(mmu.read_byte(TAC_ADDR)?);
        let tac_frequency = timer_control.clock_select;

        // println!("{:?}: {}/{}", timer_control, self.tima_cycles, tac_frequency);

        if !timer_control.enabled {
            return Ok(())
        }

        self.tima_cycles += self.t_cycles();
        if self.tima_cycles >= tac_frequency {
            let next_tima = match mmu.read_byte(TIMA_ADDR)?.checked_add(1) {
                Some(next_tima) => next_tima,
                None => {
                    let tma = mmu.read_byte(TMA_ADDR)?;
                    // An overflow occurred, request interrupt
                    mmu.set_interrupt_bit(Interrupt::Timer, true)?;
                    tma
                }
            };

            mmu.write_byte(TIMA_ADDR, next_tima)?;

            self.tima_cycles = 0;
        }

        Ok(())
    }

    fn update_div(&mut self, mmu: &mut MMU) -> Result<(), TimerError>{
        let div = mmu.read_byte(DIV_ADDR)?;
        let div_frequency = 255;

        self.div_cycles += self.t_cycles();

        if self.div_cycles >= div_frequency {
            let next_div = Wrapping(mmu.read_byte(DIV_ADDR)?) + Wrapping(1);

            mmu.write_byte(DIV_ADDR, next_div.0)?;
            self.div_cycles = 0;
        }


        Ok(())
    }
}
