use std::fs;
use std::path::Path;
use std::io::prelude::*;
use std::io::{self, BufWriter};

use crate::ast::{Expr, Compile};

pub struct Compiler {
    ast: Expr
}

impl Compiler {
    pub fn new(ast: Expr) -> Compiler {
        Compiler { ast }
    }

    pub fn compile<T: AsRef<Path>> (&mut self, path: T) -> io::Result<()> {
        fs::remove_file(&path);
        let mut stream = BufWriter::new(fs::File::create(path)?);
        stream.write(b"@echo off\n")?;
        stream.write(b"REM AUTO-GENERATED FILE. DO NOT MODIFY.\n")?;
        stream.write(b"REM This file was automatically generated by the ski compiler.\n")?;

        stream.write(self.ast.compile().as_bytes())?;

        stream.write(b"@echo on")?;
        Ok(())
    }

    fn compile_var_decl(&mut self) {

    }

    pub fn compile_for(&mut self) -> String {
        unimplemented!()
        // "FOR %%{} IN {} DO {}", self.item, self.container, self.body
    }
}