use pgt_query::protobuf::CreateTableSpaceStmt;

use crate::{
    TokenKind,
    emitter::{EventEmitter, GroupKind},
};

pub(super) fn emit_create_table_space_stmt(e: &mut EventEmitter, n: &CreateTableSpaceStmt) {
    e.group_start(GroupKind::CreateTableSpaceStmt);

    e.token(TokenKind::CREATE_KW);
    e.space();
    e.token(TokenKind::TABLESPACE_KW);

    if !n.tablespacename.is_empty() {
        e.space();
        e.token(TokenKind::IDENT(n.tablespacename.clone()));
    }

    // OWNER
    if let Some(ref owner) = n.owner {
        e.space();
        e.token(TokenKind::IDENT("OWNER".to_string()));
        e.space();
        super::emit_role_spec(e, owner);
    }

    // LOCATION (always required in CREATE TABLESPACE, even if empty string)
    e.space();
    e.token(TokenKind::IDENT("LOCATION".to_string()));
    e.space();
    // Emit location as a string literal with proper escaping
    let escaped_location = n.location.replace('\'', "''");
    e.token(TokenKind::IDENT(format!("'{}'", escaped_location)));

    // WITH options
    if !n.options.is_empty() {
        e.space();
        e.token(TokenKind::WITH_KW);
        e.space();
        e.token(TokenKind::L_PAREN);
        super::node_list::emit_comma_separated_list(e, &n.options, super::emit_node);
        e.token(TokenKind::R_PAREN);
    }

    e.token(TokenKind::SEMICOLON);
    e.group_end();
}
