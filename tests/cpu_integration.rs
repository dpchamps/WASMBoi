use std::cell::RefCell;
use std::fs;
use std::sync::Arc;
use WASMBoi::spec;
use WASMBoi::spec::gameboy::Peripheral;

#[test]
fn test_cpu() {
    let rom = fs::read("./tests/fixtures/02_interrupts.gb").unwrap();

    let mut stdout = RefCell::new(String::new());
    let mut gameboy = spec::gameboy::GameBoy::new(&rom).unwrap_or_else(|e| {
        panic!(
            "Failed to initialize GameBoy with the following error: {:?}",
            e
        )
    });

    let peripheral = Peripheral::SerialPort(Box::new(|c| {
        if let Some(c) = c {
            stdout.borrow_mut().push(c);
        }
    }));

    gameboy.attach_peripheral(peripheral);

    while !stdout.borrow().contains("Passed") {
        gameboy.cycle().unwrap();
    }
}
