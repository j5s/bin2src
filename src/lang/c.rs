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

    fn out_header(&mut self, fh: &mut BufWriter<fs::File>, fc: &mut BufWriter<fs::File>) -> Result<(), &'static str>  {
	let mut doblock = move || -> Result<(), Box<dyn Error>> {
	    // Header
	    writeln!(fh, "/*")?;
	    writeln!(fh, " *****************************************************************************")?;
	    writeln!(fh, " *")?;
	    writeln!(fh, " *")?;
	    writeln!(fh, " *      bin2src - https://github.com/gomiero/bin2src")?;
	    writeln!(fh, " *")?;
	    writeln!(fh, " *      File: {:63}", self.go.ifile_name)?;
	    writeln!(fh, " *")?;
	    writeln!(fh, " ******************************************************************************")?;
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
	    writeln!(fc, " *****************************************************************************")?;
	    writeln!(fc, " *")?;
	    writeln!(fc, " *")?;
	    writeln!(fc, " *      bin2src - https://github.com/gomiero/bin2src")?;
	    writeln!(fc, " *")?;
	    writeln!(fc, " *      File: {:63}", self.go.ifile_name)?;
	    writeln!(fc, " *")?;
	    writeln!(fc, " ******************************************************************************")?;
	    writeln!(fc, "*/")?;
	    writeln!(fc)?;
	    writeln!(fc, "#include <stdlib.h>")?;
	    writeln!(fc)?;
	    writeln!(fc, "unsigned int me_sz = {};", self.go.ifile_size)?;
	    writeln!(fc, "unsigned char me_data[] = {{")?;
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
	    writeln!(f, "  return &me_data[0];")?;
	    writeln!(f, "}}")?;
	    writeln!(f)?;
	    writeln!(f, "unsigned char* next()")?;
	    writeln!(f, "{{")?;
	    writeln!(f, "  return (iter_idx >= me_sz) ? NULL : &me_data[iter_idx++];")?;
	    writeln!(f, "}}")?;
	    writeln!(f)?;
	    writeln!(f, "unsigned char* end()")?;
	    writeln!(f, "{{")?;
	    writeln!(f, "  return &me_data[me_sz - 1];")?;
	    writeln!(f, "}}")?;
	    writeln!(f)?;
	    writeln!(f, "int set_pos(unsigned int p)")?;
	    writeln!(f, "{{")?;
	    writeln!(f, "  if (p < me_sz)")?;
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
	    writeln!(f, "  return (p >= me_sz) ? NULL: &me_data[p];")?;
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
	if self.go.ofile_name.is_empty() {
	    let fstem = self.go.ifile_path
		.file_stem()
		.unwrap()
		.to_str()
		.unwrap()
		.to_string();
	    self.go.ofile_name = fstem;
	};
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
	self.go.write_data(&mut ofile_c, 13, writeif, String::new())?;
	self.out_footer(&mut ofile_c)?;
	Ok(())
    }
}
