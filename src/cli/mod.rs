use std::path::Path;

pub mod b64;
pub mod csv;
pub mod gen_pass;
pub mod opts;
pub mod text;

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    // FQDN格式的方法调用 xx::yy::zz
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("Input file does not exist")
    }
}

// 编译宏，test表示只有在测试时才编译
#[cfg(test)]
// mod可以随意命名，一般规约为tests
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("Input file does not exist"));
        assert_eq!(
            verify_input_file("not-exist"),
            Err("Input file does not exist")
        );
    }
}
