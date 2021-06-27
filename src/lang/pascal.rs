// bin2src - convert a binary file to source code in various languages
//
//  Copyright (C) 2020  Alexandre Gomiero de Oliveira
//
// MIT License
// 
// Copyright (c) 2020-2021 Alexandre Gomiero de Oliveira
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use super::super::generator::{camel, GeneratorOutput};
use std::error::Error;
use std::fs;
use std::io::{BufWriter, Write};

#[derive(Debug)]
pub struct Pascal {
    go: GeneratorOutput,
}

impl Pascal {
    pub fn new(g: GeneratorOutput) -> Self {
        Pascal { go: g }
    }

    fn out_header(&mut self, f: &mut BufWriter<fs::File>) -> Result<(), &'static str> {
        let out_data_name = camel(&self.go.ofile_name);
        let mut doblock = move || -> Result<(), Box<dyn Error>> {
            writeln!(f, "{{******************************************************************************}}")?;
            writeln!(f, "{{                                                                              }}")?;
            writeln!(f, "{{      bin2src - https://github.com/gomiero/bin2src                            }}")?;
            writeln!(f, "{{                                                                              }}")?;
            writeln!(f, "{{      File: {:63}   }}", self.go.ifile_name)?;
            writeln!(f, "{{                                                                              }}")?;
            writeln!(f, "{{  MIT License                                                                 }}")?;
            writeln!(f, "{{                                                                              }}")?;
            writeln!(f, "{{  Copyright (c) 2020-2020 Alexandre Gomiero de Oliveira                       }}")?;
            writeln!(f, "{{                                                                              }}")?;
            writeln!(f, "{{  Permission is hereby granted, free of charge, to any person obtaining a copy}}")?;
            writeln!(f, "{{  of this software and associated documentation files (the \"Software\"), to    }}")?;
            writeln!(f, "{{  deal in the Software without restriction, including without limitation the  }}")?;
            writeln!(f, "{{  rights to use, copy, modify, merge, publish, distribute, sublicense, and/or }}")?;
            writeln!(f, "{{  sell copies of the Software, and to permit persons to whom the Software is  }}")?;
            writeln!(f, "{{  furnished to do so, subject to the following conditions:                    }}")?;
            writeln!(f, "{{                                                                              }}")?;
            writeln!(f, "{{  The above copyright notice and this permission notice shall be included in  }}")?;
            writeln!(f, "{{  all copies or substantial portions of the Software.                         }}")?;
            writeln!(f, "{{                                                                              }}")?;
            writeln!(f, "{{  THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR  }}")?;
            writeln!(f, "{{  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,    }}")?;
            writeln!(f, "{{  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE }}")?;
            writeln!(f, "{{  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER      }}")?;
            writeln!(f, "{{  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING     }}")?;
            writeln!(f, "{{  FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER         }}")?;
            writeln!(f, "{{  DEALINGS IN THE SOFTWARE.                                                   }}")?;
            writeln!(f, "{{                                                                              }}")?;
            writeln!(f, "{{******************************************************************************}}")?;
            writeln!(f)?;
            writeln!(f, "unit {};", out_data_name)?;
            writeln!(f)?;
            writeln!(f, "{{$mode ObjFPC}}")?;
            writeln!(f)?;
            writeln!(f, "interface")?;
            writeln!(f)?;
            writeln!(f, "uses")?;
            writeln!(f, "  Classes;")?;
            writeln!(f)?;
            writeln!(f, "const")?;
            writeln!(f)?;
            writeln!(
                f,
                "  {}_sz : Integer = {};",
                out_data_name, self.go.ifile_size
            )?;
            writeln!(f)?;
            writeln!(
                f,
                "  {}_data: array[0..{}] of byte = (",
                out_data_name,
                self.go.ifile_size - 1
            )?;
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
            writeln!(
                f,
                "  self.SetPointer(@{n}_data, {n}_sz);",
                n = camel(&self.go.ofile_name)
            )?;
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
        let writeif = |hex: bool,
                       comma: bool,
                       f: &mut BufWriter<fs::File>,
                       expr: u8|
         -> Result<(), Box<dyn Error>> {
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
        self.go.set_output_fname();
        self.go.odir_path.push(&self.go.ofile_name);
        self.go.odir_path.set_extension("pas");

        let mut ofile: BufWriter<fs::File> = match fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.go.odir_path)
        {
            Ok(f) => BufWriter::with_capacity(32768, f),
            _ => return Err("Can't create output file"),
        };
        self.out_header(&mut ofile)?;
        self.go
            .write_data(&mut ofile, 16, writeif, String::from("\n"))?;
        self.out_footer(&mut ofile)?;
        Ok(())
    }
}
