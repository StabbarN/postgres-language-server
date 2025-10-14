use pgt_query::protobuf::UpdateStmt;

use crate::TokenKind;
use crate::emitter::{EventEmitter, GroupKind};

use super::emit_node;
use super::node_list::emit_comma_separated_list;

pub(super) fn emit_update_stmt(e: &mut EventEmitter, n: &UpdateStmt) {
    e.group_start(GroupKind::UpdateStmt);

    e.token(TokenKind::UPDATE_KW);
    e.space();

    if let Some(ref range_var) = n.relation {
        super::emit_range_var(e, range_var)
    }

    if !n.target_list.is_empty() {
        e.space();
        e.token(TokenKind::SET_KW);
        e.space();
        emit_comma_separated_list(e, &n.target_list);
    }

    if let Some(ref where_clause) = n.where_clause {
        e.space();
        e.token(TokenKind::WHERE_KW);
        e.space();
        emit_node(where_clause, e);
    }

    e.token(TokenKind::SEMICOLON);

    e.group_end();
}
