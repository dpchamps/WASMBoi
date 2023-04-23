use std::cell::RefCell;
use std::fs;

use ntest::timeout;
use std::time::Instant;
use wasmboi::spec;
use wasmboi::spec::gameboy::Peripheral;

mod util;
use util::{run_integration_test as run_test};

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
