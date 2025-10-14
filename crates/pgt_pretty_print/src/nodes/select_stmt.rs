use pgt_query::protobuf::SelectStmt;

use crate::TokenKind;
use crate::emitter::{EventEmitter, GroupKind, LineType};

use super::node_list::emit_comma_separated_list;

pub(super) fn emit_select_stmt(e: &mut EventEmitter, n: &SelectStmt) {
    e.group_start(GroupKind::SelectStmt);

    e.token(TokenKind::SELECT_KW);

    if !n.target_list.is_empty() {
        e.indent_start();
        e.line(LineType::SoftOrSpace);

        emit_comma_separated_list(e, &n.target_list);

        e.indent_end();
    }

    if !n.from_clause.is_empty() {
        e.line(LineType::SoftOrSpace);
        e.token(TokenKind::FROM_KW);
        e.line(LineType::SoftOrSpace);

        e.indent_start();

        emit_comma_separated_list(e, &n.from_clause);

        e.indent_end();
    }

    if let Some(ref where_clause) = n.where_clause {
        e.line(LineType::SoftOrSpace);
        e.token(TokenKind::WHERE_KW);
        e.space();
        super::emit_node(where_clause, e);
    }

    e.token(TokenKind::SEMICOLON);

    e.group_end();
}
