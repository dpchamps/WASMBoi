use std::cell::RefCell;
use std::fs;
use std::time::Instant;
use wasmboi::spec;
use wasmboi::spec::gameboy::Peripheral;

pub fn run_integration_test(fixture_name: &str) -> Result<(), String> {
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
        "{} took {} cycles to complete. Expected time {} sec. Actual time to complete: {} sec",
        fixture_name, t_cycles, expected_seconds, actual_seconds
    );

    Ok(())
}