//! 256-bit arithmetic through the zkVM SDK a zkVM-agnostic guest is linked against (`zkvm_u256_*`).
//!
//! Operands are the stack words themselves: `U256`'s four little-endian `u64` limbs, the layout
//! every zkVM's 256-bit precompile reads, so nothing is converted. Each function follows the EVM
//! opcode: wrapping modulo 2^256, and zero for a zero divisor or modulus. The SDK uses its zkVM's
//! precompile where there is one and plain RISC-V otherwise.
#![expect(unsafe_code, reason = "FFI to the zkVM SDK's `zkvm_u256_*` functions")]

use ethrex_common::U256;

type Limbs = [u64; 4];

unsafe extern "C" {
    fn zkvm_u256_mul(a: *const Limbs, b: *const Limbs, result: *mut Limbs) -> i32;
    fn zkvm_u256_div(a: *const Limbs, b: *const Limbs, result: *mut Limbs) -> i32;
    fn zkvm_u256_mod(a: *const Limbs, b: *const Limbs, result: *mut Limbs) -> i32;
    fn zkvm_u256_addmod(
        a: *const Limbs,
        b: *const Limbs,
        n: *const Limbs,
        result: *mut Limbs,
    ) -> i32;
    fn zkvm_u256_mulmod(
        a: *const Limbs,
        b: *const Limbs,
        n: *const Limbs,
        result: *mut Limbs,
    ) -> i32;
    fn zkvm_u256_exp(base: *const Limbs, exponent: *const Limbs, result: *mut Limbs) -> i32;
}

macro_rules! binary {
    ($name:ident, $sym:ident) => {
        #[inline(always)]
        pub fn $name(a: &U256, b: &U256) -> U256 {
            let mut result = U256::zero();
            unsafe { $sym(&a.0, &b.0, &mut result.0) };
            result
        }
    };
}

macro_rules! ternary {
    ($name:ident, $sym:ident) => {
        #[inline(always)]
        pub fn $name(a: &U256, b: &U256, n: &U256) -> U256 {
            let mut result = U256::zero();
            unsafe { $sym(&a.0, &b.0, &n.0, &mut result.0) };
            result
        }
    };
}

binary!(mul, zkvm_u256_mul);
binary!(div, zkvm_u256_div);
binary!(rem, zkvm_u256_mod);
binary!(exp, zkvm_u256_exp);
ternary!(add_mod, zkvm_u256_addmod);
ternary!(mul_mod, zkvm_u256_mulmod);
