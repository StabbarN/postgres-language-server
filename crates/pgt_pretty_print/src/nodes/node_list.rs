use pgt_query::Node;

use crate::TokenKind;
use crate::emitter::{EventEmitter, LineType};

pub(super) fn emit_comma_separated_list(e: &mut EventEmitter, nodes: &[Node]) {
    for (i, n) in nodes.iter().enumerate() {
        if i > 0 {
            e.token(TokenKind::COMMA);
            e.line(LineType::SoftOrSpace);
        }
        super::emit_node(n, e);
    }
}

pub(super) fn emit_dot_separated_list(e: &mut EventEmitter, nodes: &[Node]) {
    for (i, n) in nodes.iter().enumerate() {
        if i > 0 {
            e.token(TokenKind::DOT);
        }
        super::emit_node(n, e);
    }
}

pub(super) fn emit_keyword_separated_list(
    e: &mut EventEmitter,
    nodes: &[Node],
    keyword: TokenKind,
) {
    for (i, n) in nodes.iter().enumerate() {
        if i > 0 {
            e.space();
            e.token(keyword.clone());
            e.line(LineType::SoftOrSpace);
        }
        super::emit_node(n, e);
    }
}
