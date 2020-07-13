
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
	    writeln!(f, "//******************************************************************************")?;
	    writeln!(f, "//")?;
	    writeln!(f, "//      bin2src - https://github.com/gomiero/bin2src")?;
	    writeln!(f, "//")?;
	    writeln!(f, "//      File: {:63}", self.go.ifile_name)?;
	    writeln!(f, "//")?;
	    writeln!(f, "//  MIT License")?;
	    writeln!(f, "// ")?;
	    writeln!(f, "//  Copyright (c) 2020-2020 Alexandre Gomiero de Oliveira*")?;
	    writeln!(f, "// ")?;
	    writeln!(f, "//  Permission is hereby granted, free of charge, to any person obtaining a copy")?;
	    writeln!(f, "//  of this software and associated documentation files (the \"Software\"), to ")?;
	    writeln!(f, "//  deal in the Software without restriction, including without limitation the ")?;
	    writeln!(f, "//  rights to use, copy, modify, merge, publish, distribute, sublicense, and/or ")?;
	    writeln!(f, "//  sell copies of the Software, and to permit persons to whom the Software is")?;
	    writeln!(f, "//  furnished to do so, subject to the following conditions:")?;
	    writeln!(f, "// ")?;
	    writeln!(f, "//  The above copyright notice and this permission notice shall be included in ")?;
	    writeln!(f, "//   all copies or substantial portions of the Software.")?;
	    writeln!(f, "// ")?;
	    writeln!(f, "//  THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR")?;
	    writeln!(f, "//  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,")?;
	    writeln!(f, "//  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE")?;
	    writeln!(f, "//  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER")?;
	    writeln!(f, "//  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING ")?;
	    writeln!(f, "//  FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS ")?;
	    writeln!(f, "//  IN THE SOFTWARE.")?;
	    writeln!(f, "//")?;	    
	    writeln!(f, "//******************************************************************************")?;
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
