use std::fmt::{self, Write};

#[derive(Debug, Copy, Clone)]
pub struct PrintOpts {
    pub reg_names: bool,
    pub armips_cop0_names: bool,
}

impl Default for PrintOpts {
    fn default() -> Self {
        Self {
            reg_names: true,
            armips_cop0_names: true,
        }
    }
}

pub(crate) trait Print {
    fn print(&self, opts: PrintOpts, w: &mut impl Write) -> fmt::Result;
}
