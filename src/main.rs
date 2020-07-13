
#![warn(anonymous_parameters)]
#![warn(bare_trait_objects)]
#![warn(elided_lifetimes_in_paths)]
#![warn(single_use_lifetimes)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
// #![warn(unreachable_pub)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]

mod lang;

use std::env;

const VERSION: &'static str = "0.0.36";
const AUTHOR: &'static str = "Alexandre Gomiero de Oliveira";

#[derive(Debug)]
pub enum Lang {
    C,
    Cshell,
    Pascal,
    Python,
    Rust,
    Undef
}

fn main() {
    let args: Vec<String> = match get_args_as_strings() {
	Ok(e) => e,
	Err(e) => {
	    println!("\n{}", e);
	    print_help();
	    return;
	}
    };

    if args.len() < 4 {
	print_help();
	return;
    };

    let mut parse_result: generator::GeneratorInput = match parse(args) {
	Ok(s) => s,
	Err(e) => {
	    println!("\nArgument parser error: {}", e);
	    print_help();
	    return;
	}
    };

    match parse_result.generate() {
	Err(e) => panic!(format!("Generator error: {}", e)),
	_ => ""
    };
}

fn get_args_as_strings() -> Result<Vec<String>, &'static str> {
    let mut ret: Vec<String> = Vec::new();
    let args = env::args_os();

    for cmd in args {
	ret.push(
	    match cmd.into_string() {
		Ok(c) => c,
		_  => return Err("Invalid unicode character found")
	    }
	);
    };
    Ok(ret)
}

fn parse(args: Vec<String>) -> Result<generator::GeneratorInput, String> {
    let mut parse_args = args.iter().skip(1); // Skip program name
    let mut inp_file: String = String::new();
    let mut out_lang: Lang = Lang::Undef;
    let mut out_dir: String = String::new();
    let mut out_file: String = String::new();
    let mut out_hex: bool = false;

    while let Some(cmd) = parse_args.next() {
	let cmd_name: &str;

	if cmd.starts_with("--") {
	    cmd_name = &cmd[2..];
	} else if cmd.starts_with("-") {
	    cmd_name = &cmd[1..];
	} else {
	    inp_file = String::from(&cmd[..]);
	    break;
	}

	match cmd_name {
	    "l" | "out-language" => {
		let value = match parse_args.next() {
		    Some(c) => c,
		    None => return Err(format!("Missing language"))
		};
		out_lang = match value.as_str() {
		    "c" => Lang::C,
		    "cshell" => Lang::Cshell,
		    "pascal" => Lang::Pascal,
		    "python" => Lang::Python,
		    "rust" => Lang::Rust,
		    l @ _ => return Err(format!("Language not implemented: {}", l))
		};
	    },
	    "d" | "out-dir" => {
		let value = match parse_args.next() {
		    Some(c) => c,
		    None => return Err(format!("Invalid directory"))
		};
		out_dir = String::from(value);
	    },
	    "f" | "out-file" => {
		let value = match parse_args.next() {
		    Some(c) => c,
		    None => return Err(format!("Invalid output file"))
		};
		out_file = String::from(value);
	    },
	    "h" | "hex" => {
		out_hex = true;
	    },
	    c @ _ => return Err(format!("Unknow command: {}", c))
	}
    }
    if inp_file.is_empty() {
	return Err(String::from("Invalid input file"));
    };
    if out_dir.is_empty() {
	out_dir = String::from("./");
    };
    Ok(generator::GeneratorInput {
	input_file: inp_file,
	output_file: out_file,
	output_dir: out_dir,
	lang: out_lang,
	hex: out_hex
    })
}

fn print_help() {
    print!(
"
bin2src - version {}
Copyright (C) 2020  {}
This program comes with ABSOLUTELY NO WARRANTY; for details type `show w'.
This is free software, and you are welcome to redistribute it
under certain conditions; for details access LICENSE file at:
https://github.com/gomiero/bin2src/

bin2src - Converts a binary file to an array of bytes, defined at a source of another language, so you can embed it into your program.

Usage: bin2src < -l LANG | --out-lang LANG > [ OPTIONS ] < FILE >

LANG and FILE are required.

Options:

	-l, --out-language LANG		specify the language, where LANG={{c|cshell|pascal|js|python}}
	-d, --out-dir PATH		specify where to output source(s) file(s)
					if not specified, generate in current directory
	-f, --out-file OUTFILE		specify the output file(s) name (* without extension *)
					if not specified, output file(s) will have the same name of input file
	-h, --hex			output bytes in hexadecimal

Currently supported languages:

  - C
  - C for shellcode
  - Pascal
  - Python
  - Rust


", VERSION, AUTHOR);
}

mod generator {

    use std::path::PathBuf;
    use std::fs;
    use std::io::{ErrorKind, Write, Read, BufWriter, BufReader};
    use std::error::Error;
    use super::Lang;
    use super::lang::c;
    use super::lang::cshell;
    use super::lang::pascal;
    use super::lang::python;
    use super::lang::rust;

    #[inline]
    pub fn camel(s: &String) -> String {
	let mut ss = s.clone().to_lowercase();
	let mut first = ss.remove(0).to_uppercase().to_string();
	first.push_str(ss.as_str());
	first
    }

    #[derive(Debug)]
    pub struct GeneratorOutput {
	pub ifile_name: String,
	pub ifile_path: PathBuf,
	pub ifile_size: u64,
	pub odir_path: PathBuf,
	pub ofile_name: String,
	pub hex: bool
    }

    impl GeneratorOutput {

	pub fn open_inp_file(&mut self) -> Result<BufReader<fs::File>, &'static str> {
	    let inp_file: BufReader<fs::File> = match fs::OpenOptions::new()
		.read(true)
		.open(&self.ifile_path) {
		    Ok(f) => BufReader::with_capacity(32768, f),
		    Err(e) => return match e.kind() {
			ErrorKind::PermissionDenied => Err("Permission"),
			ErrorKind::NotFound => Err("Not found"),
			_ => Err("Can't open file")
		    }
		};
	    Ok(inp_file)
	}

	pub fn write_data(&mut self, f: &mut BufWriter<fs::File>, numbytes: u64, write_if: fn(bool, bool, &mut BufWriter<fs::File>, u8) -> Result<(), Box<dyn Error>>, sep: String) -> Result<(), &'static str> {
	    let mut ifile = self.open_inp_file()?;
	    let mut doblock = || -> Result<(), Box<dyn Error>> {
		let mut buf = [0u8; 4096];
		let mut count = 0;
		'outter: loop {
		    let sz = ifile.read(&mut buf[..])?;
		    if sz == 0 {
			f.flush()?;
			break;
		    } else if sz <= 4096 {
			for b in 0..sz {
			    if count == self.ifile_size-1 {
				write_if(self.hex, false, f, buf[b])?;
				break 'outter;
			    };
			    write_if(self.hex, true, f, buf[b])?;
			    count += 1;
			    if count % numbytes == 0 {
				write!(f, "{}", sep)?;
			    };
			};
		    };
		};
		Ok(())
	    };
	    if let Err(_err) = doblock() {
		Err("Error when writing data block")
	    } else {
		Ok(())
	    }
	}

	pub fn set_output_fname(&mut self) {
	    if self.ofile_name.is_empty() {
		self.ofile_name = self.ifile_path
		    .file_stem()
		    .unwrap()
		    .to_str()
		    .unwrap()
		    .to_string();
		if let Some(pos) = self.ofile_name.find(".") {
		    self.ofile_name.truncate(pos);
		}
	    };
	}
    }

    #[derive(Debug)]
    pub struct GeneratorInput {
	pub input_file: String,
	pub output_file: String,
	pub output_dir: String,
	pub lang: Lang,
	pub hex: bool
    }

    impl GeneratorInput {

	fn input_file_test(&mut self) -> Result<(String, PathBuf, u64), &'static str> {
	    let ifpath: PathBuf = PathBuf::from(&self.input_file);

	    if !(ifpath.exists() || ifpath.is_file()) {
		Err("Input file does not exists or is not a file")
	    } else {
		let ifname: String = String::from(ifpath.file_name().unwrap().to_str().unwrap());
		let ifsize = ifpath.metadata().unwrap().len();
		Ok((ifname, ifpath, ifsize))
	    }
	}

	fn output_dir_test(&mut self) -> Result<PathBuf, &'static str> {
	    let ofpath: PathBuf = PathBuf::from(&self.output_dir);

	    // Test for output dir
	    if !(ofpath.exists() || ofpath.is_dir()) {
		Err("Output folder does not exists or is inacessible")
	    } else {
		Ok(ofpath)
	    }
	}

	pub fn generate(&mut self) -> Result<(), &'static str> {
	    // Test for input file
	    let (ifname, ifpath, ifsize) = self.input_file_test()?;

	    // Test for output dir
	    let ofpath: PathBuf = self.output_dir_test()?;

	    let go = GeneratorOutput {
		ifile_name: ifname,
		ifile_path: ifpath,
		ifile_size: ifsize,
		odir_path: ofpath,
		ofile_name: String::from(&self.output_file),
		hex: self.hex
	    };
	    match
		match &self.lang {
		    Lang::C => c::C::new(go).generate_files(),
		    Lang::Cshell => cshell::Cshell::new(go).generate_files(),
		    Lang::Pascal => pascal::Pascal::new(go).generate_files(),
		    Lang::Python => python::Python::new(go).generate_files(),
		    Lang::Rust => rust::Rust::new(go).generate_files(),
		    _ => Err("Language not implemented yet")
		} {
		    Ok(_) => {
			println!("Source(s) created.");
			Ok(())
		    },
		    Err(e) => Err(e)
		}
	}
    }
}
