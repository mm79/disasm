use getopt::Opt;
use std::process;

pub struct Config {
    pub filename: String,
    pub a_flag: bool,
    pub d_flag: bool,
    pub n_flag: bool,
    pub s_flag: bool,
    pub section: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, Box<dyn std::error::Error>> {
        let mut cfg = Config {
            a_flag: false,
            d_flag: false,
            n_flag: false,
            s_flag: false,
            filename: String::new(),
            section: String::new(),
        };

        let mut opts = getopt::Parser::new(&args, "adsf:nS:");

        loop {
            match opts.next().transpose()? {
                None => break,
                Some(opt) => match opt {
                    Opt('a', None) => cfg.a_flag = true,
                    Opt('d', None) => cfg.d_flag = true,
                    Opt('n', None) => cfg.n_flag = true,
                    Opt('s', None) => cfg.s_flag = true,
                    Opt('f', Some(string)) => cfg.filename = string.clone(),
                    Opt('S', Some(string)) => cfg.section = string.clone(),
                    _ => todo!(),
                },
            }
        }

        if cfg.filename.is_empty() {
            Self::usage();
            process::exit(0);
        }

        Ok(cfg)
    }

    fn usage() {
        println!(
            "Usage: disasm -f filename [-s] [-S sectioname] [-d] [-a]
		 -f filename -> file to open
		 -n -> show symbols
		 -s -> show section names
		 -S -> view this section name
		 -d disassemble
                 -a use AT&T syntax"
        );
    }
}
