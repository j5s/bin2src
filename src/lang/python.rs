

use super::super::generator::GeneratorOutput;
use std::io::{Write, BufWriter};
use std::fs;
use std::error::Error;


pub struct Python {
    go: GeneratorOutput
}

impl Python {

    pub fn new(g: GeneratorOutput) -> Self {
	Python {
	    go: g
	}
    }

    fn out_header(&mut self, f: &mut BufWriter<fs::File>) -> Result<(), &'static str>  {
	let mut doblock = move || -> Result<(), Box<dyn Error>> {
	    writeln!(f, "#*******************************************************************************")?;
	    writeln!(f, "#")?;
	    writeln!(f, "#      bin2src - https://github.com/gomiero/bin2src")?;
	    writeln!(f, "#")?;
	    writeln!(f, "#      File: {:63}", self.go.ifile_name)?;
	    writeln!(f, "#")?;
	    writeln!(f, "#*******************************************************************************")?;
	    writeln!(f)?;
	    if self.go.hex {
		write!(f, "{}_DATA  = b\"", self.go.ofile_name.to_uppercase())?;
	    } else {
		writeln!(f, "{}_DATA = bytes([", self.go.ofile_name.to_uppercase())?;
	    };
	    Ok(())
	};

	if let Err(_err) = doblock() {
	    Err("Error when writing header")
	} else {
	    Ok(())
	}
    }

    fn out_footer(&mut self, f: &mut BufWriter<fs::File>) -> Result<(), &'static str> {
	let mut doblock = move || -> Result<(), Box<dyn Error>> {
	    if self.go.hex {
		write!(f, "\"")?;
	    } else {
		writeln!(f, "])")?;
	    };
	    writeln!(f)?;
	    Ok(())
	};
	if let Err(_err) = doblock() {
	    Err("Error when writing data block")
	} else {
	    Ok(())
	}
    }

    pub fn generate_files(&mut self) -> Result<(), &'static str> {
	let writeif = |hex: bool, comma: bool, f: &mut BufWriter<fs::File>, expr:u8| -> Result<(), Box<dyn Error>>  {
	    if hex {
		    write!(f, "\\x{:>02x}", expr)?;
	    } else {
		if comma {
		    write!(f, "{:3}, ", expr)?;
		} else {
		    write!(f, "{:3}", expr)?;
		};
	    }
	    Ok(())
	};
	let sep = if self.go.hex {
	    format!("\"\n{}_DATA += b\"", self.go.ofile_name.to_uppercase())
	} else {
	    String::from("\n")
	};
	if self.go.ofile_name.is_empty() {
	    self.go.ofile_name = self.go.ifile_path
		.file_stem()
		.unwrap()
		.to_str()
		.unwrap()
		.to_string();
	};
	self.go.odir_path.push(&self.go.ofile_name);
	self.go.odir_path.set_extension("py");

	let mut ofile: BufWriter<fs::File> = match fs::OpenOptions::new()
	    .write(true)
	    .create(true)
	    .truncate(true)
	    .open(&self.go.odir_path) {
		Ok(f) => BufWriter::with_capacity(32768, f),
		_ => return Err("Can't create output file")
	    };
	self.out_header(&mut ofile)?;
	self.go.write_data(&mut ofile, 16, writeif, sep)?;
	self.out_footer(&mut ofile)?;
	Ok(())
    }
}
