use std::iter::Peekable;

use crate::lexer::Lexer;
use crate::parser::Parser;

use crate::error_handler::parse_error::ParseError;

use crate::types::node::Node;
use crate::types::node::Node::*;
use crate::types::tokenize::OperatorKind::*;
use crate::types::tokenize::TokenKind::*;

use super::equality::equality;

//assign = equality ( "=" assign )?
pub(super) fn assign(parser: &mut Parser, lexer: &mut Peekable<Lexer>) -> Result<Node, ParseError> {
    log::info!("Parsing is entered 'assign' !");

    let node = equality(parser, lexer)?;

    if parser.choice(lexer, Opr(Assign)) {
        Ok(NdAssign(Box::new(node), Box::new(assign(parser, lexer)?)))
    } else {
        Ok(node)
    }
}
