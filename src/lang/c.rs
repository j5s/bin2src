use super::super::generator::GeneratorOutput;
use std::io::{Write, BufWriter};
use std::fs;
use std::error::Error;

pub struct C {
    go: GeneratorOutput
}

impl C {
    
    pub fn new(g: GeneratorOutput) -> Self {
	C {
	    go: g
	}
    }

    fn write_license(&mut self, f: &mut BufWriter<fs::File>) -> Result<(), Box<dyn Error>>  {
	writeln!(f, " * MIT License")?;
	writeln!(f, " *")?;
	writeln!(f, " * Copyright (c) 2020-2020 Alexandre Gomiero de Oliveira")?;
	writeln!(f, " *")?;
	writeln!(f, " * Permission is hereby granted, free of charge, to any person obtaining a copy")?;
	writeln!(f, " * of this software and associated documentation files (the \"Software\"), to deal")?;
	writeln!(f, " * in the Software without restriction, including without limitation the rights")?;
	writeln!(f, " * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell")?;
	writeln!(f, " * copies of the Software, and to permit persons to whom the Software is")?;
	writeln!(f, " * furnished to do so, subject to the following conditions:")?;
	writeln!(f, " *")?;
	writeln!(f, " * The above copyright notice and this permission notice shall be included in")?;
	writeln!(f, " * all copies or substantial portions of the Software.")?;
	writeln!(f, " *")?;
	writeln!(f, " * THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR")?;
	writeln!(f, " * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,")?;
	writeln!(f, " * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE")?;
	writeln!(f, " * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER")?;
	writeln!(f, " * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,")?;
	writeln!(f, " * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE")?;
	writeln!(f, " * SOFTWARE.")?;	    
	Ok(())
    }
    
    fn out_header(&mut self, fh: &mut BufWriter<fs::File>, fc: &mut BufWriter<fs::File>) -> Result<(), &'static str>  {
	let mut doblock = move || -> Result<(), Box<dyn Error>> {
	    // Header
	    writeln!(fh, "/*")?;
	    writeln!(fh, " *******************************************************************************")?;
	    writeln!(fh, " *")?;
	    writeln!(fh, " *")?;
	    writeln!(fh, " *      bin2src - https://github.com/gomiero/bin2src")?;
	    writeln!(fh, " *")?;
	    writeln!(fh, " *      File: {:63}", self.go.ifile_name)?;
	    writeln!(fh, " *")?;
	    self.write_license(fh)?;
	    writeln!(fh, " *")?;
	    writeln!(fh, " *******************************************************************************")?;
	    writeln!(fh, "*/")?;
	    writeln!(fh)?;
	    writeln!(fh, "#ifndef __{}__HEADER", self.go.ofile_name.to_uppercase())?;
	    writeln!(fh, "#define __{}__HEADER", self.go.ofile_name.to_uppercase())?;
	    writeln!(fh)?;
	    writeln!(fh, "// Very simple iterator implementation")?;
	    writeln!(fh, "void restart();")?;
	    writeln!(fh, "unsigned char* begin();")?;
	    writeln!(fh, "unsigned char* next();")?;
	    writeln!(fh, "unsigned char* end();")?;
	    writeln!(fh, "int set_pos(unsigned int p);")?;
	    writeln!(fh, "unsigned char* get_pos(unsigned int p);")?;
	    writeln!(fh)?;
	    writeln!(fh, "extern unsigned int {}_sz;", self.go.ofile_name.to_lowercase())?;
	    writeln!(fh, "extern unsigned char {}_data[];", self.go.ofile_name.to_lowercase())?;
	    writeln!(fh)?;
	    writeln!(fh, "#endif")?;
	    writeln!(fh)?;
	    // Header end

	    // Body c	    
	    writeln!(fc, "/*")?;
	    writeln!(fc, " *******************************************************************************")?;
	    writeln!(fc, " *")?;
	    writeln!(fc, " *")?;
	    writeln!(fc, " *      bin2src - https://github.com/gomiero/bin2src")?;
	    writeln!(fc, " *")?;
	    writeln!(fc, " *      File: {:63}", self.go.ifile_name)?;
	    writeln!(fc, " *")?;
	    self.write_license(fc)?;
	    writeln!(fc, " *")?;
	    writeln!(fc, " *******************************************************************************")?;
	    writeln!(fc, "*/")?;
	    writeln!(fc)?;
	    writeln!(fc, "#include <stdlib.h>")?;
	    writeln!(fc)?;
	    writeln!(fc, "unsigned int {}_sz = {};", self.go.ofile_name.to_lowercase(), self.go.ifile_size)?;
	    writeln!(fc, "unsigned char {}_data[] = {{", self.go.ofile_name.to_lowercase())?;
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
	    let varname = self.go.ofile_name.to_lowercase();
	    
	    writeln!(f, "}};")?;
	    writeln!(f)?;
	    writeln!(f, "static unsigned int iter_idx = 0;")?;
	    writeln!(f)?;
	    writeln!(f, "void restart()")?;
	    writeln!(f, "{{")?;
	    writeln!(f, "  iter_idx = 0;")?;
	    writeln!(f, "}}")?;
	    writeln!(f)?;
	    writeln!(f, "unsigned char* begin()")?;
	    writeln!(f, "{{")?;
	    writeln!(f, "  iter_idx = 0;")?;
	    writeln!(f, "  return &{}_data[0];", varname)?;
	    writeln!(f, "}}")?;
	    writeln!(f)?;
	    writeln!(f, "unsigned char* next()")?;
	    writeln!(f, "{{")?;
	    writeln!(f, "  return (iter_idx >= {me}_sz) ? NULL : &{me}_data[iter_idx++];", me = varname)?;
	    writeln!(f, "}}")?;
	    writeln!(f)?;
	    writeln!(f, "unsigned char* end()")?;
	    writeln!(f, "{{")?;
	    writeln!(f, "  return &{me}_data[{me}_sz - 1];", me = varname)?;
	    writeln!(f, "}}")?;
	    writeln!(f)?;
	    writeln!(f, "int set_pos(unsigned int p)")?;
	    writeln!(f, "{{")?;
	    writeln!(f, "  if (p < {}_sz)", varname)?;
	    writeln!(f, "  {{")?;
	    writeln!(f, "    iter_idx = p;")?;
	    writeln!(f, "    return 1;")?;
	    writeln!(f, "  }}")?;
	    writeln!(f, "  else {{")?;
	    writeln!(f, "    return -1;")?;
	    writeln!(f, "  }}")?;
	    writeln!(f, "}}")?;
	    writeln!(f)?;
	    writeln!(f, "unsigned char* get_pos(unsigned int p)")?;
	    writeln!(f, "{{")?;
	    writeln!(f, "  return (p >= {me}_sz) ? NULL: &{me}_data[p];", me = varname)?;
	    writeln!(f, "}}")?;
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
	let cols = if self.go.hex {
	    13
	} else {
	    16
	};
	let writeif = |hex: bool, comma: bool, f: &mut BufWriter<fs::File>, expr:u8| -> Result<(), Box<dyn Error>>  {
	    if hex {
		if comma {
		    write!(f, "{:#04x}, ", expr)?;
		} else {
		    write!(f, "{:#04x}", expr)?;
		};
	    } else {
		if comma {
		    write!(f, "{:3}, ", expr)?;
		} else {
		    write!(f, "{:3}", expr)?;
		};
	    }
	    Ok(())
	};
	let mut ifile_h_path = self.go.odir_path.clone();
	self.go.set_output_fname();

	// Header file
	ifile_h_path.push(&self.go.ofile_name);
	ifile_h_path.set_file_name(&self.go.ofile_name);
	ifile_h_path.set_extension("h");
	
	// Implementation file
	self.go.odir_path.push(&self.go.ofile_name);
	self.go.odir_path.set_file_name(self.go.ofile_name.clone());
	self.go.odir_path.set_extension("c");
	

	let mut ofile_c: BufWriter<fs::File> = match fs::OpenOptions::new()
	    .write(true)
	    .create(true)
	    .truncate(true)
	    .open(&self.go.odir_path) {
		Ok(f) => BufWriter::new(f),
		_ => return Err("Can't create output .c file")
	    };
	let mut ofile_h: BufWriter<fs::File> = match fs::OpenOptions::new()
	    .write(true)
	    .create(true)
	    .truncate(true)
	    .open(&ifile_h_path) {
		Ok(f) => BufWriter::with_capacity(32768, f),
		_ => return Err("Can't create output .h file")
	    };
	self.out_header(&mut ofile_h, &mut ofile_c)?;
	self.go.write_data(&mut ofile_c, cols, writeif, String::from("\n"))?;
	self.out_footer(&mut ofile_c)?;
	Ok(())
    }
}
