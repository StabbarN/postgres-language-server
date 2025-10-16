use crate::{
    TokenKind,
    emitter::{EventEmitter, GroupKind},
    nodes::node_list::emit_comma_separated_list,
};
use pgt_query::{NodeEnum, protobuf::CreateSubscriptionStmt};

pub(super) fn emit_create_subscription_stmt(e: &mut EventEmitter, n: &CreateSubscriptionStmt) {
    e.group_start(GroupKind::CreateSubscriptionStmt);

    e.token(TokenKind::CREATE_KW);
    e.space();
    e.token(TokenKind::IDENT("SUBSCRIPTION".to_string()));
    e.space();
    e.token(TokenKind::IDENT(n.subname.clone()));

    e.space();
    e.token(TokenKind::IDENT("CONNECTION".to_string()));
    e.space();
    // Emit connection string as string literal
    e.token(TokenKind::IDENT(format!("'{}'", n.conninfo)));

    e.space();
    e.token(TokenKind::IDENT("PUBLICATION".to_string()));
    e.space();
    emit_comma_separated_list(e, &n.publication, |node, e| {
        if let Some(NodeEnum::String(s)) = &node.node {
            e.token(TokenKind::IDENT(s.sval.clone()));
        }
    });

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
