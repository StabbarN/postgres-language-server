use crate::{
    TokenKind,
    emitter::{EventEmitter, GroupKind, LineType},
};
use pgt_query::protobuf::CreateForeignServerStmt;

use super::node_list::emit_comma_separated_list;

pub(super) fn emit_create_foreign_server_stmt(e: &mut EventEmitter, n: &CreateForeignServerStmt) {
    e.group_start(GroupKind::CreateForeignServerStmt);

    e.token(TokenKind::CREATE_KW);
    e.space();
    e.token(TokenKind::IDENT("SERVER".to_string()));

    // Emit IF NOT EXISTS if present
    if n.if_not_exists {
        e.space();
        e.token(TokenKind::IF_KW);
        e.space();
        e.token(TokenKind::NOT_KW);
        e.space();
        e.token(TokenKind::EXISTS_KW);
    }

    // Emit server name
    e.space();
    e.token(TokenKind::IDENT(n.servername.clone()));

    // Emit TYPE if present
    if !n.servertype.is_empty() {
        e.line(LineType::SoftOrSpace);
        e.indent_start();
        e.token(TokenKind::TYPE_KW);
        e.space();
        e.token(TokenKind::IDENT(format!("'{}'", n.servertype)));
        e.indent_end();
    }

    // Emit VERSION if present
    if !n.version.is_empty() {
        e.line(LineType::SoftOrSpace);
        e.indent_start();
        e.token(TokenKind::IDENT("VERSION".to_string()));
        e.space();
        e.token(TokenKind::IDENT(format!("'{}'", n.version)));
        e.indent_end();
    }

    // Emit FOREIGN DATA WRAPPER
    e.line(LineType::SoftOrSpace);
    e.indent_start();
    e.token(TokenKind::IDENT("FOREIGN".to_string()));
    e.space();
    e.token(TokenKind::IDENT("DATA".to_string()));
    e.space();
    e.token(TokenKind::IDENT("WRAPPER".to_string()));
    e.space();
    e.token(TokenKind::IDENT(n.fdwname.clone()));
    e.indent_end();

    // Emit OPTIONS if present
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
