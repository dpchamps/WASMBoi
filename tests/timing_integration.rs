use ntest::timeout;
mod util;
use util::{run_integration_test as run_test};

#[test]
#[timeout(2000)]
fn blargg_instr_timing() -> Result<(), String> {
    run_test("instr_timing.gb")
}

#[test]
#[timeout(2000)]
fn blargg_mem_timing() -> Result<(), String> {
    run_test("mem_timing.gb")
}
