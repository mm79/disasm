use object::{Object, ObjectSection, ObjectSymbol, SectionKind};
use std::env;

mod config;
mod disassemble;
mod file;
mod hexdump;

use config::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let conf = Config::new(&args).unwrap();

    let binary_path = &conf.filename;
    let binary_data = file::read(binary_path).expect("unable to read the file");

    if file::is_text(&binary_data) {
        hexdump::dump(&binary_data);
        return;
    }

    let obj_file = object::File::parse(&*binary_data).expect("Unable to parse the file");

    let arch = obj_file.architecture();

    if conf.n_flag == true {
        for sym in obj_file.symbols() {
            if sym.is_undefined() {
                print!("undef ");
            } else {
                print!("{:016x} ", sym.address());
            }

            println!("{}", sym.name().unwrap_or("unknown"));
        }
        return;
    }

    for section in obj_file.sections() {
        let section_name = section.name().unwrap();

        if conf.s_flag == true {
            println!("section {}", section_name);
            continue;
        }

        if !conf.section.is_empty() && conf.section != section_name {
            continue;
        }

        if conf.d_flag == true {
            // disassemble only executable sections
            if conf.section.is_empty() && section.kind() != SectionKind::Text {
                continue;
            }

            print!("\nDisassembling '{}' section", section_name);

            let dis_arch = match arch {
                object::Architecture::I386 => disassemble::Architectures::I386,
                object::Architecture::X86_64 => disassemble::Architectures::X86_64,
                object::Architecture::Aarch64 => disassemble::Architectures::ARM64,
                _ => todo!(),
            };

            let syms = obj_file
                .symbols()
                .filter(|s| !s.is_undefined())
                .map(|s| (s.address(), s))
                .collect();

            let mut disasm = disassemble::Disasm::new(dis_arch);

            if conf.a_flag == true {
                disasm.set_att_syntax().unwrap();
            }

            disasm.disasm(&section.data().unwrap(), section.address(), &syms);
        } else {
            println!("Dumping '{}' section", section_name);

            hexdump::dump(&section.data().unwrap());
        }
    }
}
