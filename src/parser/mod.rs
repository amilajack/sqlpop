//! A SQL parser.
//! Adapted from [SQLite parser](http://www.sqlite.org/src/artifact?ci=trunk&filename=src/parse.y)
extern crate wasm_bindgen;

use ast::Cmd;
use lalrpop_util;
use tok;

#[allow(dead_code)]
lalrpop_mod!(lrsql, "/parser/lrsql.rs");

#[cfg(test)]
mod test;

pub type ParseError<'input> = lalrpop_util::ParseError<usize, tok::Tok<'input>, tok::Error>;

#[wasm_bindgen]
pub fn parse_sql<'input>(input: &'input str) -> Result<Vec<Option<Cmd>>, ParseError<'input>> {
    use self::lrsql::CmdListParser;
    let tokenizer = tok::Tokenizer::new(input, 0);
    let sql = try!(CmdListParser::new().parse(input, tokenizer));

    Ok(sql)
}
