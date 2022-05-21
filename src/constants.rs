// Directories
pub const INSTRUCTIONS_DIR: &str = "instructions";
pub const ANCHOR_DIR: &str = "anchor";
pub const IDL_DIR: &str = "idl";

// Program defaults
pub const PROGRAM_BEGINNING: &str = r#"// This file is autogenerated with https://github.com/acheroncrypto/native-to-anchor

use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod program_name {
use super::*;

"#;
pub const SKIP_LINE: [&str; 8] = [
    "///",
    "#[allow(dead_code)]",
    "#[deprecated",
    "#[serde(",
    "since =",
    "note =",
    ")]",
    "Reserved",
];
pub const DEFAULT_ARGS: [&str; 5] = ["u16", "u32", "u64", "bool", "[u64; 8]"];
pub const ACCOUNT_SPLIT: [&str; 4] = ["_pubkey", "_key", "_pk", "_id"];
