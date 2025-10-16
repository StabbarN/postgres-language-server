use crate::{
    TokenKind,
    emitter::{EventEmitter, GroupKind},
};
use pgt_query::protobuf::DeleteStmt;

pub(super) fn emit_delete_stmt(e: &mut EventEmitter, n: &DeleteStmt) {
    emit_delete_stmt_impl(e, n, true);
}

pub(super) fn emit_delete_stmt_no_semicolon(e: &mut EventEmitter, n: &DeleteStmt) {
    emit_delete_stmt_impl(e, n, false);
}

fn emit_delete_stmt_impl(e: &mut EventEmitter, n: &DeleteStmt, with_semicolon: bool) {
    e.group_start(GroupKind::DeleteStmt);

    e.token(TokenKind::DELETE_KW);
    e.space();
    e.token(TokenKind::FROM_KW);
    e.space();

    // Emit table name
    if let Some(ref relation) = n.relation {
        super::emit_range_var(e, relation);
    }

    // Emit WHERE clause
    if let Some(ref where_clause) = n.where_clause {
        e.space();
        e.token(TokenKind::WHERE_KW);
        e.space();
        super::emit_node(where_clause, e);
    }

    // TODO: Handle USING clause
    // TODO: Handle RETURNING clause

    if with_semicolon {
        e.token(TokenKind::SEMICOLON);
    }

    e.group_end();
}
