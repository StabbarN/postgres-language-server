use crate::{
    TokenKind,
    emitter::{EventEmitter, GroupKind},
    nodes::node_list::emit_comma_separated_list,
};
use pgt_query::protobuf::TypeName;

pub(super) fn emit_type_name(e: &mut EventEmitter, n: &TypeName) {
    e.group_start(GroupKind::TypeName);

    // Add SETOF prefix if present
    if n.setof {
        e.token(TokenKind::SETOF_KW);
        e.space();
    }

    // Collect name parts from the names list
    if !n.names.is_empty() {
        let mut name_parts = Vec::new();

        for node in &n.names {
            if let Some(pgt_query::NodeEnum::String(s)) = &node.node {
                name_parts.push(s.sval.clone());
            }
        }

        // Skip pg_catalog schema for built-in types
        if name_parts.len() == 2 && name_parts[0].to_lowercase() == "pg_catalog" {
            name_parts.remove(0);
        }

        // Normalize type name
        let type_name = if name_parts.len() == 1 {
            normalize_type_name(&name_parts[0])
        } else {
            // Qualified type name - emit with dots
            for (i, part) in name_parts.iter().enumerate() {
                if i > 0 {
                    e.token(TokenKind::DOT);
                }
                e.token(TokenKind::IDENT(part.clone()));
            }
            // Already emitted, return early after modifiers
            emit_type_modifiers(e, n);
            emit_array_bounds(e, n);
            e.group_end();
            return;
        };

        e.token(TokenKind::IDENT(type_name));
    }

    // Add type modifiers if present (e.g., VARCHAR(255))
    emit_type_modifiers(e, n);

    // Add array bounds if present (e.g., INT[], INT[10])
    emit_array_bounds(e, n);

    e.group_end();
}

fn emit_type_modifiers(e: &mut EventEmitter, n: &TypeName) {
    if !n.typmods.is_empty() {
        // TODO: Handle special INTERVAL type modifiers
        e.token(TokenKind::L_PAREN);
        emit_comma_separated_list(e, &n.typmods, |node, emitter| {
            super::emit_node(node, emitter)
        });
        e.token(TokenKind::R_PAREN);
    }
}

fn emit_array_bounds(e: &mut EventEmitter, n: &TypeName) {
    // Emit array bounds (e.g., [] or [10])
    for bound in &n.array_bounds {
        if let Some(pgt_query::NodeEnum::Integer(int_bound)) = &bound.node {
            if int_bound.ival == -1 {
                e.token(TokenKind::L_BRACK);
                e.token(TokenKind::R_BRACK);
            } else {
                e.token(TokenKind::L_BRACK);
                e.token(TokenKind::IDENT(int_bound.ival.to_string()));
                e.token(TokenKind::R_BRACK);
            }
        }
    }
}

fn normalize_type_name(name: &str) -> String {
    // Normalize common type names
    match name.to_lowercase().as_str() {
        "int2" => "SMALLINT".to_string(),
        "int4" => "INT".to_string(),
        "int8" => "BIGINT".to_string(),
        "float4" => "REAL".to_string(),
        "float8" => "DOUBLE PRECISION".to_string(),
        "bool" => "BOOLEAN".to_string(),
        "bpchar" => "CHAR".to_string(),
        // Keep other types as-is but uppercase common SQL types
        "integer" => "INT".to_string(),
        "smallint" => "SMALLINT".to_string(),
        "bigint" => "BIGINT".to_string(),
        "real" => "REAL".to_string(),
        "boolean" => "BOOLEAN".to_string(),
        "char" => "CHAR".to_string(),
        "varchar" => "VARCHAR".to_string(),
        "text" => "TEXT".to_string(),
        "date" => "DATE".to_string(),
        "time" => "TIME".to_string(),
        "timestamp" => "TIMESTAMP".to_string(),
        "timestamptz" => "TIMESTAMPTZ".to_string(),
        "interval" => "INTERVAL".to_string(),
        "numeric" => "NUMERIC".to_string(),
        "decimal" => "DECIMAL".to_string(),
        "uuid" => "UUID".to_string(),
        "json" => "JSON".to_string(),
        "jsonb" => "JSONB".to_string(),
        "bytea" => "BYTEA".to_string(),
        _ => name.to_string(),
    }
}
