use crate::TokenKind;
use crate::emitter::{EventEmitter, GroupKind, LineType};
use pgt_query::protobuf::AlterForeignServerStmt;

use super::node_list::emit_comma_separated_list;

pub(super) fn emit_alter_foreign_server_stmt(e: &mut EventEmitter, n: &AlterForeignServerStmt) {
    e.group_start(GroupKind::AlterForeignServerStmt);

    e.token(TokenKind::ALTER_KW);
    e.space();
    e.token(TokenKind::IDENT("SERVER".to_string()));
    e.space();

    if !n.servername.is_empty() {
        e.token(TokenKind::IDENT(n.servername.clone()));
    }

    if n.has_version && !n.version.is_empty() {
        e.line(LineType::SoftOrSpace);
        e.indent_start();
        e.token(TokenKind::IDENT("VERSION".to_string()));
        e.space();
        e.token(TokenKind::IDENT(format!("'{}'", n.version)));
        e.indent_end();
    }

    if !n.options.is_empty() {
        e.line(LineType::SoftOrSpace);
        e.indent_start();
        e.token(TokenKind::IDENT("OPTIONS".to_string()));
        e.space();
        e.token(TokenKind::L_PAREN);
        emit_comma_separated_list(e, &n.options, |n, e| {
            let def_elem = assert_node_variant!(DefElem, n);
            super::emit_options_def_elem(e, def_elem);
        });
        e.token(TokenKind::R_PAREN);
        e.indent_end();
    }

    e.token(TokenKind::SEMICOLON);

    e.group_end();
}
