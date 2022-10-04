use std::{
    env::args,
    fs::File,
    io::{self, Write},
};

#[derive(Debug)]
struct TreeType {
    base_class_name: String,
    class_name: String,
    fields: Vec<String>,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        eprintln!("Usage: generate-ast <output-directory>");
    }

    let output_directory = args.get(1).unwrap();

    define_ast(
        output_directory,
        "Expr",
        &[
            "Binary   : Box<Expr> left, Token Operator, Box<Expr> right".to_string(),
            "Grouping : Box<Expr> expression".to_string(),
            "Literal  : Object value".to_string(),
            "Unary    : Token operator, Box<Expr> right".to_string(),
        ],
    )?;

    Ok(())
}

fn define_ast(output_directory: &str, base_name: &str, types: &[String]) -> io::Result<()> {
    let path = format!("{output_directory}/{}", base_name.to_lowercase());
    let mut file = File::create(&path)?;

    let mut tree_types = Vec::new();

    writeln!(file, "use crate::error::*;")?;
    writeln!(file, "use crate::token::*;\n")?;

    for ttype in types {
        let (base_class_name, args) = ttype.split_once(':').unwrap();
        let class_name = format!("{}{}", base_class_name.trim(), base_name);
        let arg_split = args.split(',');
        let mut fields = Vec::new();

        for arg in arg_split {
            let (t2type, name) = arg.trim().split_once(' ').unwrap();
            fields.push(format!("{} : {}", name, t2type));
        }

        tree_types.push(TreeType {
            base_class_name: base_class_name.trim().to_string(),
            class_name,
            fields,
        });
    }

    writeln!(file, "pub enum {base_name} {{")?;
    for t in tree_types {
        writeln!(file, "\t{}({}),", t.base_class_name, t.class_name)?;
    }

    writeln!(file, "}}")?;

    Ok(())
}
