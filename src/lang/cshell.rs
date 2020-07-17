
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
	    writeln!(f, " *")?;
	    writeln!(f, " ******************************************************************************")?;
	    writeln!(f, "*/")?;
	    writeln!(f)?;
	    writeln!(f, "#include <stdio.h>")?;
	    writeln!(f)?;
	    if self.go.hex {
		writeln!(f, "#ifdef __GNUC__")?;
		writeln!(f, "#pragma GCC diagnostic push")?;
		writeln!(f, "#pragma GCC diagnostic ignored \"-Woverlength-strings\"")?;
		writeln!(f, "#endif")?;
		writeln!(f)?;
	    };
	    writeln!(f, "unsigned int {}_sz = {};", self.go.ofile_name.to_lowercase(), self.go.ifile_size)?;
	    writeln!(f)?;
	    write!(f, "{}{}_code{}",
		   if self.go.hex { "char *" } else { "unsigned char " },
		   self.go.ofile_name.to_lowercase(),
		   if self.go.hex { " = \n\"" } else { "[] = {\n" })?;
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
	    writeln!(f, "{};", if self.go.hex { "\"" } else { "}"})?;
	    writeln!(f)?;
	    if self.go.hex {		
		writeln!(f, "#ifdef __GNUC__")?;
		writeln!(f, "#pragma GCC diagnostic pop")?;
		writeln!(f, "#endif")?;
		writeln!(f)?;
	    };
	    writeln!(f, "typedef void (*t_shell_func)(void);")?;
	    writeln!(f)?;
	    writeln!(f, "int main(void)")?;
	    writeln!(f, "{{")?;
	    writeln!(f, "    int ret = 0;")?;
	    writeln!(f, "    t_shell_func shell_func;")?;
	    writeln!(f)?;
	    writeln!(f, "    *(void **)(&shell_func) = {}_code;", self.go.ofile_name.to_lowercase())?;
	    writeln!(f, "    shell_func();")?;
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
	let sep = if self.go.hex {
	    String::from("\"\n\"")
	} else {
	    String::from("\n")
	};
	let writeif = |_hex: bool, _comma: bool, f: &mut BufWriter<fs::File>, expr:u8| -> Result<(), Box<dyn Error>>  {
	    if _hex {
		write!(f, "\\x{:02x}", expr)?;
	    } else {
		write!(f, "{:3}{} ", expr, if _comma {","} else {""})?;		    
	    };
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
	self.go.write_data(&mut ofile, if self.go.hex { 16 } else { 15 }, writeif, sep)?;
	self.out_footer(&mut ofile)?;
	Ok(())
    }
}
