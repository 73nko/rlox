use std::{
    fs::File,
    io::{self, Write},
};

#[derive(Debug)]
struct TreeType {
    base_class_name: String,
    class_name: String,
    fields: Vec<String>,
}

pub fn generate_ast(output_directory: &str) -> io::Result<()> {
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
    let path = format!("{output_directory}/{}.rs", base_name.to_lowercase());
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
    for t in &tree_types {
        writeln!(file, "\t{}({}),", t.base_class_name, t.class_name)?;
    }

    writeln!(file, "}}")?;

    for t in &tree_types {
        writeln!(file, "pub struct {} {{", t.class_name)?;
        for f in t.fields.iter() {
            writeln!(file, "\t{f},")?;
        }
        writeln!(file, "}}\n")?;
    }

    writeln!(file, "pub trait ExprVisitor<T> {{")?;

    for t in &tree_types {
        writeln!(
            file,
            "\t fn visit_{}_{}(&self, expr: &{}) -> Result<T, LoxError>;",
            t.base_class_name.to_lowercase(),
            base_name.to_lowercase(),
            t.class_name,
        )?;
    }
    writeln!(file, "}}\n")?;

    for t in &tree_types {
        writeln!(file, "impl {} {{", t.class_name)?;
        writeln!(
            file,
            "\tfn accept<T>(&self, visitor: &dyn {}Visitor<T>) -> Result<T, LoxError>{{",
            t.class_name
        )?;
        writeln!(
            file,
            "\t\tvisitor.visit_{}_{}(self)",
            t.base_class_name.to_lowercase(),
            base_name.to_lowercase()
        )?;
        writeln!(file, "\t}}")?;
        writeln!(file, "}}\n")?;
    }

    Ok(())
}
