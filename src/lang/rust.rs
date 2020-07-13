
use super::super::generator::GeneratorOutput;
use std::io::{Write, BufWriter};
use std::fs;
use std::error::Error;


#[derive(Debug)]
pub struct Rust {
    go: GeneratorOutput
}

impl Rust {
  
    pub fn new(g: GeneratorOutput) -> Self {
	Rust {
	    go: g
	}
    }

    fn out_header(&mut self, f: &mut BufWriter<fs::File>) -> Result<(), &'static str>
    {
	let mut doblock = move || -> Result<(), Box<dyn Error>> {
	    writeln!(f, "//*****************************************************************************")?;
	    writeln!(f, "//")?;
	    writeln!(f, "//      bin2src - https://github.com/gomiero/bin2src")?;
	    writeln!(f, "//")?;
	    writeln!(f, "//      File: {:63}", self.go.ifile_name)?;
	    writeln!(f, "//")?;
	    writeln!(f, "//*****************************************************************************")?;
	    writeln!(f)?;
	    writeln!(f, "#![allow(dead_code)]")?;
	    writeln!(f, "pub static {}_SIZE: u64 = {};", self.go.ofile_name.to_uppercase(), self.go.ifile_size)?;
	    writeln!(f)?;
	    writeln!(f, "pub static {}_DATA: &'static [u8; {}] = &[", self.go.ofile_name.to_uppercase(), self.go.ifile_size)?;
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
	    writeln!(f, "];")?;
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
		if comma {
		    write!(f, "{:#04x}u8, ", expr)?;
		} else {
		    write!(f, "{:#04x}u8", expr)?;
		};
	    } else {
		if comma {
		    write!(f, "{:3}u8, ", expr)?;
		} else {
		    write!(f, "{:3}u8", expr)?;
		};
	    }
	    Ok(())
	};
	self.go.set_output_fname();
	self.go.odir_path.push(&self.go.ofile_name);
	self.go.odir_path.set_extension("rs");

	let mut ofile: BufWriter<fs::File> = match fs::OpenOptions::new()
	    .write(true)
	    .create(true)
	    .truncate(true)
	    .open(&self.go.odir_path) {
		Ok(f) => BufWriter::with_capacity(32768, f),
		_ => return Err("Can't create output file")
	    };
	self.out_header(&mut ofile)?;
	self.go.write_data(&mut ofile, 10, writeif, "\n".to_string())?;
	self.out_footer(&mut ofile)?;
	Ok(())
    }
}
