use super::node_list::emit_comma_separated_list;
use crate::{
    TokenKind,
    emitter::{EventEmitter, GroupKind},
};
use pgt_query::protobuf::AlterSubscriptionStmt;

pub(super) fn emit_alter_subscription_stmt(e: &mut EventEmitter, n: &AlterSubscriptionStmt) {
    e.group_start(GroupKind::AlterSubscriptionStmt);

    e.token(TokenKind::ALTER_KW);
    e.space();
    e.token(TokenKind::IDENT("SUBSCRIPTION".to_string()));
    e.space();
    e.token(TokenKind::IDENT(n.subname.clone()));

    e.space();

    // Kind enum: 0=Undefined, 1=OPTIONS, 2=CONNECTION, 3=SET_PUBLICATION, 4=ADD_PUBLICATION, 5=DROP_PUBLICATION, 6=REFRESH, 7=ENABLED, 8=SKIP
    match n.kind {
        1 => {
            // OPTIONS - handled via options field below
        }
        2 => {
            e.token(TokenKind::IDENT("CONNECTION".to_string()));
            e.space();
            e.token(TokenKind::IDENT(format!("'{}'", n.conninfo)));
        }
        3 => {
            e.token(TokenKind::SET_KW);
            e.space();
            e.token(TokenKind::IDENT("PUBLICATION".to_string()));
            e.space();
            emit_comma_separated_list(e, &n.publication, super::emit_node);
        }
        4 => {
            e.token(TokenKind::IDENT("ADD".to_string()));
            e.space();
            e.token(TokenKind::IDENT("PUBLICATION".to_string()));
            e.space();
            emit_comma_separated_list(e, &n.publication, super::emit_node);
        }
        5 => {
            e.token(TokenKind::DROP_KW);
            e.space();
            e.token(TokenKind::IDENT("PUBLICATION".to_string()));
            e.space();
            emit_comma_separated_list(e, &n.publication, super::emit_node);
        }
        6 => {
            e.token(TokenKind::IDENT("REFRESH".to_string()));
            e.space();
            e.token(TokenKind::IDENT("PUBLICATION".to_string()));
        }
        7 => {
            e.token(TokenKind::IDENT("ENABLE".to_string()));
        }
        8 => {
            e.token(TokenKind::IDENT("SKIP".to_string()));
        }
        _ => {}
    }

    // Options
    if !n.options.is_empty() {
        e.space();
        e.token(TokenKind::WITH_KW);
        e.space();
        e.token(TokenKind::L_PAREN);
        emit_comma_separated_list(e, &n.options, super::emit_node);
        e.token(TokenKind::R_PAREN);
    }

    e.token(TokenKind::SEMICOLON);

    e.group_end();
}
