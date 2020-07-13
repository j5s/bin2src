
use super::super::generator::GeneratorOutput;
use std::io::{Write, BufWriter};
use std::fs;
use std::error::Error;

pub struct Cshell {
    go: GeneratorOutput
}

impl Cshell {
    
    pub fn new(g: GeneratorOutput) -> Self {
	Cshell {
	    go: g
	}
    }

    fn out_header(&mut self, f: &mut BufWriter<fs::File>) -> Result<(), &'static str>  {
	let mut doblock = move || -> Result<(), Box<dyn Error>> {
	    writeln!(f, "/*")?;
	    writeln!(f, " *****************************************************************************")?;
	    writeln!(f, " *")?;
	    writeln!(f, " *")?;
	    writeln!(f, " *      bin2src - https://github.com/gomiero/bin2src")?;
	    writeln!(f, " *")?;
	    writeln!(f, " *      File: {:63}", self.go.ifile_name)?;
	    writeln!(f, " *")?;
	    writeln!(f, " ******************************************************************************")?;
	    writeln!(f, "*/")?;
	    writeln!(f)?;
	    writeln!(f, "#include <stdlib.h>")?;
	    writeln!(f)?;
	    write!(f, "char *{}_code = \"", self.go.ofile_name.to_lowercase())?;
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
	    writeln!(f, "\";")?;
	    writeln!(f)?;
	    writeln!(f, "int main()")?;
	    writeln!(f, "{{")?;
	    writeln!(f, "    int ret = 0;")?;
	    writeln!(f, "    int (*myfunc)();")?;
	    writeln!(f)?;
	    writeln!(f, "    myfunc = (int (*)()) {}_code;", self.go.ofile_name.to_lowercase())?;
	    writeln!(f, "    ret = (*myfunc)();")?;
	    writeln!(f, "    return ret;")?;
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
	let writeif = |_hex: bool, _comma: bool, f: &mut BufWriter<fs::File>, expr:u8| -> Result<(), Box<dyn Error>>  {
	    write!(f, "\\x{:02x}", expr)?;
	    Ok(())
	};
	self.go.set_output_fname();
	self.go.odir_path.push(&self.go.ofile_name);
	self.go.odir_path.set_file_name(self.go.ofile_name.clone());
	self.go.odir_path.set_extension("c");
	

	let mut ofile: BufWriter<fs::File> = match fs::OpenOptions::new()
	    .write(true)
	    .create(true)
	    .truncate(true)
	    .open(&self.go.odir_path) {
		Ok(f) => BufWriter::new(f),
		_ => return Err("Can't create output .c file")
	    };
	self.out_header(&mut ofile)?;
	self.go.write_data(&mut ofile, self.go.ifile_size + 1, writeif, String::new())?;
	self.out_footer(&mut ofile)?;
	Ok(())
    }
}
