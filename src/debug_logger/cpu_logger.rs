use crate::debug_logger::{DebugLogger, FromEnvList};


lazy_static! {
    pub static ref CPU_LOGGER: CpuDebug = CpuDebug::env("CPU_DEBUG").build();
}

#[derive(Default)]
pub struct CpuDebug {
    env_var: String,
    current_instruction: bool,
    current_register_values: bool,
    alu: bool,
    bitwise: bool,
    control: bool,
    ld: bool,
    stack: bool,
    interrupts: bool,
    gb_doc: bool,
}

impl FromEnvList for CpuDebug {
    fn from_env_list(debug_list: &[String]) -> Self {
        CpuDebug {
            env_var: String::new(),
            current_instruction: debug_list.contains(&"PC".into()),
            current_register_values: debug_list.contains(&"REG".into()),
            alu: debug_list.contains(&"ALU".into()),
            bitwise: debug_list.contains(&"BIT".into()),
            control: debug_list.contains(&"CONTROL".into()),
            ld: debug_list.contains(&"LD".into()),
            stack: debug_list.contains(&"STACK".into()),
            interrupts: debug_list.contains(&"INTS".into()),
            gb_doc: debug_list.contains(&"GB_DOC".into()),
        }
    }
}

impl DebugLogger for CpuDebug {
    #[cfg(debug_assertions)]
    fn log<F>(&self, t: &str, f: F)
        where
            F: Fn(),
    {
        let should_log = match t {
            "PC" => self.current_instruction,
            "REG" => self.current_register_values,
            "ALU" => self.alu,
            "BIT" => self.bitwise,
            "CONTROL" => self.control,
            "LD" => self.ld,
            "STACK" => self.stack,
            "INTS" => self.interrupts,
            "GB_DOC" => self.gb_doc,
            _ => false,
        };

        if should_log {
            f()
        }
    }

    #[cfg(not(debug_assertions))]
    fn log<F>(&self, t: &str, f: F)
        where
            F: Fn() -> (),
    {
    }
}