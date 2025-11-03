#[macro_export]
macro_rules! shim {
    ($executable:literal, $args:expr) => {
        fn main() -> std::process::ExitCode {
            let mut process = build_shim::ProcessBuilder::new($executable);
            let args = { $args };
            process.args(&args);

            for arg in std::env::args_os().skip(1) {
                process.arg(arg);
            }

            build_shim::exec_replace(process)
        }
    };
}

use std::process::ExitCode;
pub use cargo_util::ProcessBuilder;
use cargo_util::ProcessError;

pub fn exec_replace(process: ProcessBuilder) -> ExitCode {
    // only windows will return from exec_replace
    if let Err(e) = process.exec_replace() {
        match e.downcast::<ProcessError>() {
            Ok(e) => {
                let code = e.code.unwrap_or(127);
                if (code & 0xFF) == 0 {
                    eprintln!("original exit code: {}", code);
                    return std::process::ExitCode::from(255);
                }
                return ExitCode::from(code as u8);
            }
            Err(e) => {
                eprintln!("{:?}", e);
                return ExitCode::FAILURE;
            }
        }
    }

    ExitCode::SUCCESS
}
