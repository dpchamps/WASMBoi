use std::cell::RefCell;
use std::fs;

use ntest::timeout;
use std::time::Instant;
use wasmboi::spec;
use wasmboi::spec::gameboy::Peripheral;
fn run_test(fixture_name: &str) -> Result<(), String> {
    let fixture_location = format!("./tests/fixtures/{}", fixture_name);
    let rom = fs::read(&fixture_location)
        .map_err(|_| format!("Failed to read fixture from location: {}", fixture_location))?;

    let serial_port_out = RefCell::new(String::new());
    let mut gameboy = spec::gameboy::GameBoy::new(&rom)
        .map_err(|e| format!("Failed to initialize gameboy with {:?}", e))?;

    gameboy.attach_peripheral(Peripheral::SerialPort(Box::new(|c| {
        if let Some(c) = c {
            serial_port_out.borrow_mut().push(c);
        }
    })));
    let mut cycles = 0;
    let now = Instant::now();
    while !serial_port_out.borrow().contains("Passed") {
        cycles += gameboy
            .cycle()
            .map_err(|e| format!("Failed to execute gameboy cycle with error {:?}", e))?;

        if serial_port_out.borrow().to_lowercase().contains("failed") {
            return Err(format!("{} received fail code from ROM", fixture_name));
        }
    }

    let t_cycles = cycles * 4;
    let expected_seconds = t_cycles as f64 / 4194300.0;
    let actual_seconds = now.elapsed().as_secs();
    assert!(actual_seconds as f64 <= expected_seconds);
    println!(
        "{} took {} cycles to complete. Expected time {} sec. Actual time to complete: {}",
        fixture_name, t_cycles, expected_seconds, actual_seconds
    );

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

#[test]
#[timeout(2000)]
fn blargg_08_misc_instrs() -> Result<(), String> {
    run_test("08_misc_instrs.gb")
}

#[test]
#[timeout(5000)]
fn blargg_09_op_r_r() -> Result<(), String> {
    run_test("09_op_r_r.gb")
}

#[ignore]
#[timeout(2000)]
fn blargg_10_bit_ops() -> Result<(), String> {
    run_test("10_bit_ops.gb")
}

#[test]
#[timeout(7000)]
fn blargg_11_op_a_hl() -> Result<(), String> {
    run_test("11_op_a_hl_.gb")
}
