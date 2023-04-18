use std::cell::RefCell;
use std::fs;
use std::sync::Arc;
use WASMBoi::spec;
use WASMBoi::spec::gameboy::{Peripheral};
use ntest::{timeout};


fn run_test(fixture_name: &str) -> Result<(), String> {
    let fixture_location = format!("./tests/fixtures/{}", fixture_name);
    let rom = fs::read(&fixture_location).map_err(|_| format!("Failed to read fixture from location: {}", fixture_location))?;

    let mut serial_port_out = RefCell::new(String::new());
    let mut gameboy = spec::gameboy::GameBoy::new(&rom).map_err(|e| format!("Failed to initialize gameboy with {:?}", e))?;

    gameboy.attach_peripheral(Peripheral::SerialPort(Box::new(|c| {
        if let Some(c) = c {
            serial_port_out.borrow_mut().push(c);
        }
    })));

    while !serial_port_out.borrow().contains("Passed") {
        gameboy.cycle().map_err(|e| format!("Failed to execute gameboy cycle with error {:?}", e))?;

        if serial_port_out.borrow().to_lowercase().contains("failed") {
            return Err(format!("{} received fail code from ROM", fixture_name))
        }
    }

    Ok(())
}

#[test]
#[timeout(2000)]
fn blargg_01_special() -> Result<(), String> {
    run_test("01_special.gb")
}

#[test]
#[timeout(2000)]
fn blargg_02_interrupts() -> Result<(), String> {
    run_test("02_interrupts.gb")
}

#[test]
#[timeout(2000)]
fn blargg_03_op_sp_hl() -> Result<(), String> {
    run_test("03_op_sp_hl.gb")
}

#[ignore]
#[test]
#[timeout(2000)]
fn blargg_04_op_r_imm() -> Result<(), String> {
    run_test("04_op_r_imm.gb")
}

#[test]
#[timeout(2000)]
fn blargg_05_op_rp() -> Result<(), String> {
    run_test("05_op_rp.gb")
}

#[test]
#[timeout(2000)]
fn blargg_06_ld_r_r() -> Result<(), String> {
    run_test("06_ld_r_r.gb")
}

#[test]
#[timeout(2000)]
fn blargg_07_jr_jp_call_ret_rst() -> Result<(), String> {
    run_test("07_jr_jp_call_ret_rst.gb")
}

#[ignore]
#[test]
#[timeout(2000)]
fn blargg_08_misc_instrs() -> Result<(), String> {
    run_test("08_misc_instrs.gb")
}

#[ignore]
#[test]
#[timeout(2000)]
fn blargg_09_op_r_r() -> Result<(), String> {
    run_test("09_op_r_r.gb")
}

#[ignore]
#[test]
#[timeout(2000)]
fn blargg_10_bit_ops() -> Result<(), String> {
    run_test("10_bit_ops.gb")
}

#[ignore]
#[test]
#[timeout(2000)]
fn blargg_11_op_a_hl() -> Result<(), String> {
    run_test("11_op_a_hl_.gb")
}