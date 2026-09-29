//! Ethrex stateless validator guest program, for every zkVM. It uses `std`: built for
//! `riscv64im-unknown-zkvm-elf`, where `ere-platform-zkvm` provides what `std` needs.

use ere_platform_zkvm::{ZkvmPlatform, run};
use ethrex_stateless_validator::platform::entrypoint;

#[unsafe(no_mangle)]
extern "C" fn main() -> i32 {
    run(entrypoint::<ZkvmPlatform>)
}
