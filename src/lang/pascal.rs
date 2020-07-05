

use super::super::generator::{camel, GeneratorOutput};
use std::io::{Write, BufWriter};
use std::fs;
use std::error::Error;

/*
macro_rules! ssx {
    ($($s:expr),*) => ( vec![$($s.to_string()),*] );
}
*/

#[derive(Debug)]
pub struct Pascal {
    go: GeneratorOutput,
}

impl Pascal {
  
    pub fn new(g: GeneratorOutput) -> Self {
	Pascal {
	    go: g
	}
    }

    fn out_header(&mut self, f: &mut BufWriter<fs::File>) -> Result<(), &'static str>
    {
	let out_data_name = camel(&self.go.ofile_name);
	let mut doblock = move || -> Result<(), Box<dyn Error>> {
	    writeln!(f, "{{******************************************************************************}}")?;
	    writeln!(f, "{{                                                                              }}")?;
	    writeln!(f, "{{      bin2src - https://github.com/gomiero/bin2src                            }}")?;
	    writeln!(f, "{{                                                                              }}")?;
	    writeln!(f, "{{      File: {:63}   }}", self.go.ifile_name)?;
	    writeln!(f, "{{                                                                              }}")?;
	    writeln!(f, "{{******************************************************************************}}")?;
	    writeln!(f)?;
	    writeln!(f, "unit {};", out_data_name)?;
	    writeln!(f)?;
	    writeln!(f, "interface")?;
	    writeln!(f)?;
	    writeln!(f, "uses")?;
	    writeln!(f, "  Classes;")?;
	    writeln!(f)?;
	    writeln!(f, "const")?;
	    writeln!(f)?;
	    writeln!(f, "  {}_sz : Integer = {};", out_data_name, self.go.ifile_size)?;
	    writeln!(f)?;
	    writeln!(f, "  {}_data: array[0..{}] of byte = (", out_data_name, self.go.ifile_size-1)?;
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
	    write!(f, ");")?;
	    writeln!(f)?;
	    writeln!(f)?;
	    write!(f, "type")?;
	    writeln!(f)?;
	    writeln!(f)?;
	    let mut vname: String = self.go.ofile_name.clone();
	    vname.push_str("Stream");
	    let mut tname: String = String::from("T");
	    tname.push_str(camel(&vname).as_str());
	    writeln!(f, "  {} = class(TMemoryStream)", tname)?;
	    writeln!(f, "    function getStream: TMemoryStream;")?;
	    writeln!(f, "  end;")?;
	    writeln!(f)?;
	    writeln!(f, "var")?;
	    writeln!(f)?;
	    writeln!(f, "  {} : {};", vname, tname)?;
	    writeln!(f)?;
	    writeln!(f, "implementation")?;
	    writeln!(f)?;
	    writeln!(f, "function {}.getStream: TMemoryStream;", tname)?;
	    writeln!(f, "begin")?;
	    writeln!(f, "  self.SetPointer(@{n}_data, {n}_sz);", n=camel(&self.go.ofile_name))?;
	    writeln!(f, "  getStream := self;")?;
	    writeln!(f, "end;")?;
	    writeln!(f)?;
	    writeln!(f, "end.")?;
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
		    write!(f, "${:>02x}, ", expr)?;
		} else {
		    write!(f, "${:>02x}", expr)?;
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
	if self.go.ofile_name.is_empty() {
	    self.go.ofile_name = self.go.ifile_path
		.file_stem()
		.unwrap()
		.to_str()
		.unwrap()
		.to_string();
	};
	self.go.odir_path.push(&self.go.ofile_name);
	self.go.odir_path.set_extension("pas");

	let mut ofile: BufWriter<fs::File> = match fs::OpenOptions::new()
	    .write(true)
	    .create(true)
	    .truncate(true)
	    .open(&self.go.odir_path) {
		Ok(f) => BufWriter::with_capacity(32768, f),
		_ => return Err("Can't create output file")
	    };
	self.out_header(&mut ofile)?;
	self.go.write_data(&mut ofile, 16, writeif, String::from("\n"))?;
	self.out_footer(&mut ofile)?;
	Ok(())
    }
}
