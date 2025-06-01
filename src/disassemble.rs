use capstone::prelude::*;
use object::{ObjectSymbol, Symbol};

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Architectures {
    I386,
    X86_64,
    ARM64,
}

pub struct Disasm {
    architecture: Architectures,
    cap: Capstone,
}

impl Disasm {
    pub fn new(arch: Architectures) -> Disasm {
        Disasm {
            architecture: arch.clone(),
            cap: Self::get_capstone_object(arch),
        }
    }

    pub fn disasm(&self, data: &[u8], addr: u64, symbols: &HashMap<u64, Symbol>) {
        println!(" [Arch: {:?}]", self.architecture);
        let insns = self.cap.disasm_all(data, addr).unwrap();
        for insn in insns.iter() {
            let address = insn.address();

            if let Some(sym) = symbols.get(&address) {
                println!("\n{:#010x} <{}>:", address, sym.name().unwrap_or("unknown"));
            }

            println!(
                "{:#010x}:\t{}\t{}",
                address,
                insn.mnemonic().unwrap_or(""),
                insn.op_str().unwrap_or("")
            );
        }
    }

    pub fn set_att_syntax(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.cap.set_syntax(capstone::Syntax::Att)?;

        Ok(())
    }

    fn get_capstone_object(arch: Architectures) -> Capstone {
        let cap = match arch {
            Architectures::I386 => Capstone::new()
                .x86()
                .mode(arch::x86::ArchMode::Mode32)
                .build()
                .expect("Failed to create capstone for x86"),
            Architectures::X86_64 => Capstone::new()
                .x86()
                .mode(arch::x86::ArchMode::Mode64)
                .build()
                .expect("Failed to create capstone for Mode64"),
            Architectures::ARM64 => Capstone::new()
                .arm64()
                .mode(arch::arm64::ArchMode::Arm)
                .build()
                .expect("Failed to create capstone for arm64"),
        };

        cap
    }
}
