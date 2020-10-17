mod ast;
mod infer;
mod types;

use ast::*;
use infer::*;

fn main() {
    for expression in &[
        num(42),
        let_("x", num(42), var("x")),
        let_("f", lambda("x", num(42)), app(var("f"), num(42))),
        let_("f", lambda("x", var("x")), app(var("f"), num(42))),
        let_(
            "f",
            lambda("x", num(42)),
            let_("y", app(var("f"), num(42)), var("f")),
        ),
        let_(
            "f",
            lambda("x", lambda("x", num(42))),
            app(app(var("f"), num(42)), num(42)),
        ),
        let_("f", lambda("x", var("x")), var("f")),
        let_(
            "f",
            lambda("x", var("x")),
            let_("y", app(var("f"), num(42)), var("f")),
        ),
        let_("f", lambda("x", lambda("x", var("x"))), var("f")),
        let_("f", lambda("x", lambda("y", var("x"))), var("f")),
        let_("f", lambda("x", app(var("f"), var("x"))), var("f")),
    ] {
        let type_scheme = infer_type_scheme(expression).unwrap();

        println!("{} : {}", expression, type_scheme);
    }
}
