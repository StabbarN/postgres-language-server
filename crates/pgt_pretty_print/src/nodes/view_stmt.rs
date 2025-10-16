use pgt_query::protobuf::{ViewCheckOption, ViewStmt};

use crate::{
    TokenKind,
    emitter::{EventEmitter, GroupKind, LineType},
};

use super::node_list::emit_comma_separated_list;

pub(super) fn emit_view_stmt(e: &mut EventEmitter, n: &ViewStmt) {
    e.group_start(GroupKind::ViewStmt);

    e.token(TokenKind::CREATE_KW);

    if n.replace {
        e.space();
        e.token(TokenKind::OR_KW);
        e.space();
        e.token(TokenKind::REPLACE_KW);
    }

    e.space();
    e.token(TokenKind::VIEW_KW);

    if let Some(ref view) = n.view {
        e.space();
        super::emit_range_var(e, view);
    }

    // Column aliases
    if !n.aliases.is_empty() {
        e.space();
        e.token(TokenKind::L_PAREN);
        emit_comma_separated_list(e, &n.aliases, super::emit_node);
        e.token(TokenKind::R_PAREN);
    }

    // Query
    if let Some(ref query) = n.query {
        e.space();
        e.token(TokenKind::AS_KW);
        e.line(LineType::SoftOrSpace);
        super::emit_node(query, e);
    }

    // WITH CHECK OPTION
    match n.with_check_option() {
        ViewCheckOption::LocalCheckOption => {
            e.space();
            e.token(TokenKind::WITH_KW);
            e.space();
            e.token(TokenKind::LOCAL_KW);
            e.space();
            e.token(TokenKind::CHECK_KW);
            e.space();
            e.token(TokenKind::OPTION_KW);
        }
        ViewCheckOption::CascadedCheckOption => {
            e.space();
            e.token(TokenKind::WITH_KW);
            e.space();
            e.token(TokenKind::CASCADED_KW);
            e.space();
            e.token(TokenKind::CHECK_KW);
            e.space();
            e.token(TokenKind::OPTION_KW);
        }
        ViewCheckOption::NoCheckOption => {
            // No check option
        }
        ViewCheckOption::Undefined => {
            // Undefined, don't emit
        }
    }

    e.token(TokenKind::SEMICOLON);

    e.group_end();
}
