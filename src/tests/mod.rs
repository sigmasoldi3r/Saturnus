use crate::{
    code::{Builder, Visitor},
    lua::LuaEmitter,
    parser::Script,
};

mod assignment;
mod collections;
mod conditions;
mod functions;
mod numbers;

/// Fixture that streamlines the compilation, only for tests.
pub fn compile(input: &str) -> String {
    LuaEmitter
        .visit_script(Builder::new("  "), &Script::parse(input).unwrap())
        .unwrap()
        .collect()
}

pub fn compile_expr(input: &str) -> String {
    LuaEmitter
        .visit_expression(
            Builder::new("  "),
            &Script::parse_expression(input).unwrap(),
        )
        .unwrap()
        .collect()
}