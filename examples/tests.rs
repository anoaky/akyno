use std::{
    fs::File,
    io::{BufRead, BufReader, Result, Write},
    path::{Path, PathBuf},
};
use toyc::{
    lexer::{Category, Token, Tokeniser},
    util::CompilerPass,
};
use xmlwriter::*;

const LEXER_FAIL: u32 = 250;
const PASS: u32 = 0;

pub fn main() -> Result<()> {
    let test_cases = collect_test_cases()?;
    let mut w = XmlWriter::new(Options::default());
    let mut lexer_passed = 0;
    let mut lexer_total = 0;
    let mut parser_passed = 0;
    let mut parser_total = 0;
    let mut sem_passed = 0;
    let mut sem_total = 0;
    let mut codegen_passed = 0;
    let mut codegen_total = 0;
    let mut regalloc_passed = 0;
    let mut regalloc_total = 0;
    w.write_declaration();
    w.start_element("tests");
    for tc in test_cases {
        println!("Found test case: {}", tc.name);
        if tc.lexer_exit_code == LEXER_FAIL {
            lexer_total += 1;
            if tc.test_lexer(&mut w)? == LEXER_FAIL {
                lexer_passed += 1;
            }
        } else {
            lexer_total += 1;
            if tc.test_lexer(&mut w)? == PASS {
                lexer_passed += 1;
            }
        }
    }
    write_overview(&mut w, "lexer", lexer_passed, lexer_total);
    write_overview(&mut w, "parser", parser_passed, parser_total);
    write_overview(&mut w, "sem", sem_passed, sem_total);
    write_overview(&mut w, "codegen", codegen_passed, codegen_total);
    write_overview(&mut w, "regalloc", regalloc_passed, regalloc_total);

    let mut report_path = File::create("tests/reports/report.xml")?;
    write!(&mut report_path, "{}", w.end_document())?;
    Ok(())
}

fn write_overview(w: &mut XmlWriter, component: &str, passed: u32, total: u32) {
    w.start_element("overview");
    w.write_attribute("component", component);
    w.write_attribute("passed", &passed.to_string());
    w.write_attribute("total", &total.to_string());
    w.end_element();
}

struct TestCase {
    pub name: String,
    pub path: PathBuf,
    pub lexer_exit_code: u32,
}

impl TestCase {
    pub fn from_path(fp: PathBuf) -> Result<Self> {
        let path = fp.clone();
        let name = fp
            .file_stem()
            .expect("Failed to get test file name")
            .to_string_lossy()
            .to_string();
        let mut reader = BufReader::new(File::open(fp)?);
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let expected_exit_code = if line.starts_with("/*") {
            // extract exit code
            line.clear();
            reader.read_line(&mut line)?;
            line.trim().parse::<u32>().unwrap_or(1)
        } else if line.starts_with("//") {
            line.split_off(2).trim().parse::<u32>().unwrap_or(1)
        } else {
            0
        };
        let lexer_exit_code = if expected_exit_code == LEXER_FAIL {
            LEXER_FAIL
        } else {
            0
        };
        Ok(Self {
            name,
            path,
            lexer_exit_code,
        })
    }

    pub fn test_lexer(&self, w: &mut XmlWriter) -> Result<u32> {
        w.start_element("test");
        w.write_attribute("name", &self.name);
        w.write_attribute("expected", &self.lexer_exit_code.to_string());
        w.write_attribute("component", "lexer");
        let mut tokeniser = Tokeniser::from_path(self.path.as_path())?;
        while tokeniser.next_token()?.category() != Category::Eof {}
        let exit_code = if tokeniser.has_error() {
            LEXER_FAIL
        } else {
            PASS
        };
        w.write_attribute("actual", &exit_code.to_string());
        w.end_element();
        Ok(exit_code)
    }
}

fn collect_test_cases() -> Result<Vec<TestCase>> {
    let mut v: Vec<TestCase> = Vec::new();
    let test_paths = std::fs::read_dir("tests/resources/source")?;
    for path in test_paths {
        v.push(TestCase::from_path(path?.path().to_path_buf())?);
    }
    Ok(v)
}
