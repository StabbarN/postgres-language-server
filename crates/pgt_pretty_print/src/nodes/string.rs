use pgt_query::protobuf::String;

use crate::{
    TokenKind,
    emitter::{EventEmitter, GroupKind},
};

pub(super) fn emit_string(e: &mut EventEmitter, n: &String) {
    e.group_start(GroupKind::String);
    e.token(TokenKind::IDENT(n.sval.clone()));
    e.group_end();
}

pub(super) fn emit_string_literal(e: &mut EventEmitter, n: &String) {
    e.group_start(GroupKind::String);
    // Escape single quotes by doubling them (PostgreSQL string literal syntax)
    let escaped = n.sval.replace('\'', "''");
    e.token(TokenKind::IDENT(format!("'{}'", escaped)));
    e.group_end();
}

pub(super) fn emit_string_identifier(e: &mut EventEmitter, n: &String) {
    e.group_start(GroupKind::String);
    emit_identifier(e, &n.sval);
    e.group_end();
}

pub(super) fn emit_identifier(e: &mut EventEmitter, n: &str) {
    // Escape double quotes by doubling them (PostgreSQL identifier syntax)
    let escaped = n.replace('"', "\"\"");
    e.token(TokenKind::IDENT(format!("\"{}\"", escaped)));
}

/// Emit an identifier, adding quotes only if necessary.
/// Quotes are needed if:
/// - Contains special characters (space, comma, quotes, etc.)
/// - Is a SQL keyword
/// - Starts with a digit
/// - Contains uppercase letters (to preserve case)
/// Note: Empty strings are emitted as plain identifiers (not quoted)
pub(super) fn emit_identifier_maybe_quoted(e: &mut EventEmitter, n: &str) {
    // Don't emit empty identifiers at all
    if n.is_empty() {
        return;
    }

    if needs_quoting(n) {
        emit_identifier(e, n);
    } else {
        e.token(TokenKind::IDENT(n.to_string()));
    }
}

/// Check if an identifier needs to be quoted
fn needs_quoting(s: &str) -> bool {
    if s.is_empty() {
        return true;
    }

    // Check if starts with digit
    if s.chars().next().unwrap().is_ascii_digit() {
        return true;
    }

    // Check for uppercase letters (need to preserve case)
    if s.chars().any(|c| c.is_uppercase()) {
        return true;
    }

    // Check for special characters or non-alphanumeric/underscore
    if s.chars().any(|c| !c.is_alphanumeric() && c != '_') {
        return true;
    }

    // Check if it's a SQL keyword (simplified list of common ones)
    // In a real implementation, this would check against the full keyword list
    const KEYWORDS: &[&str] = &[
        "select",
        "from",
        "where",
        "insert",
        "update",
        "delete",
        "create",
        "drop",
        "alter",
        "table",
        "index",
        "view",
        "schema",
        "database",
        "user",
        "role",
        "grant",
        "revoke",
        "with",
        "as",
        "on",
        "in",
        "into",
        "values",
        "set",
        "default",
        "null",
        "not",
        "and",
        "or",
        "between",
        "like",
        "ilike",
        "case",
        "when",
        "then",
        "else",
        "end",
        "join",
        "left",
        "right",
        "inner",
        "outer",
        "cross",
        "union",
        "intersect",
        "except",
        "order",
        "group",
        "having",
        "limit",
        "offset",
        "by",
        "for",
        "to",
        "of",
    ];

    KEYWORDS.contains(&s.to_lowercase().as_str())
}
