use pgt_query::protobuf::RowExpr;

use crate::{
    TokenKind,
    emitter::{EventEmitter, GroupKind},
};

use super::node_list::emit_comma_separated_list;

pub(super) fn emit_row_expr(e: &mut EventEmitter, n: &RowExpr) {
    e.group_start(GroupKind::RowExpr);

    // ROW constructor can be explicit ROW(...) or implicit (...)
    // row_format: CoerceExplicitCall = explicit ROW keyword
    // Always use explicit ROW(...) for clarity, especially when used with field access
    e.token(TokenKind::ROW_KW);
    e.token(TokenKind::L_PAREN);

    if !n.args.is_empty() {
        emit_comma_separated_list(e, &n.args, super::emit_node);
    }

    e.token(TokenKind::R_PAREN);

    e.group_end();
}
