use crate::TokenKind;
use crate::emitter::{EventEmitter, GroupKind};
use pgt_query::protobuf::AlterEnumStmt;

use super::node_list::emit_dot_separated_list;

pub(super) fn emit_alter_enum_stmt(e: &mut EventEmitter, n: &AlterEnumStmt) {
    e.group_start(GroupKind::AlterEnumStmt);

    e.token(TokenKind::ALTER_KW);
    e.space();
    e.token(TokenKind::IDENT("TYPE".to_string()));
    e.space();

    // Enum type name (qualified)
    if !n.type_name.is_empty() {
        emit_dot_separated_list(e, &n.type_name);
    }

    e.space();

    // Check if this is ADD VALUE or RENAME VALUE
    if !n.old_val.is_empty() {
        // RENAME VALUE old TO new
        e.token(TokenKind::IDENT("RENAME".to_string()));
        e.space();
        e.token(TokenKind::IDENT("VALUE".to_string()));
        e.space();
        e.token(TokenKind::IDENT(format!("'{}'", n.old_val)));
        e.space();
        e.token(TokenKind::TO_KW);
        e.space();
        e.token(TokenKind::IDENT(format!("'{}'", n.new_val)));
    } else {
        // ADD VALUE [ IF NOT EXISTS ] new_value [ BEFORE old_value | AFTER old_value ]
        e.token(TokenKind::ADD_KW);
        e.space();
        e.token(TokenKind::IDENT("VALUE".to_string()));

        if n.skip_if_new_val_exists {
            e.space();
            e.token(TokenKind::IF_KW);
            e.space();
            e.token(TokenKind::NOT_KW);
            e.space();
            e.token(TokenKind::EXISTS_KW);
        }

        if !n.new_val.is_empty() {
            e.space();
            e.token(TokenKind::IDENT(format!("'{}'", n.new_val)));
        }

        // Optional BEFORE/AFTER clause
        if !n.new_val_neighbor.is_empty() {
            e.space();
            if n.new_val_is_after {
                e.token(TokenKind::IDENT("AFTER".to_string()));
            } else {
                e.token(TokenKind::IDENT("BEFORE".to_string()));
            }
            e.space();
            e.token(TokenKind::IDENT(format!("'{}'", n.new_val_neighbor)));
        }
    }

    e.token(TokenKind::SEMICOLON);

    e.group_end();
}
