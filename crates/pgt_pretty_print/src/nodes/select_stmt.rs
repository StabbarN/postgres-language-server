use pgt_query::protobuf::SelectStmt;

use crate::TokenKind;
use crate::emitter::{EventEmitter, GroupKind, LineType};

use super::node_list::emit_comma_separated_list;

pub(super) fn emit_select_stmt(e: &mut EventEmitter, n: &SelectStmt) {
    emit_select_stmt_impl(e, n, true);
}

pub(super) fn emit_select_stmt_no_semicolon(e: &mut EventEmitter, n: &SelectStmt) {
    emit_select_stmt_impl(e, n, false);
}

fn emit_select_stmt_impl(e: &mut EventEmitter, n: &SelectStmt, with_semicolon: bool) {
    e.group_start(GroupKind::SelectStmt);

    // Emit WITH clause (Common Table Expressions) if present
    if let Some(ref with_clause) = n.with_clause {
        super::emit_with_clause(e, with_clause);
        e.line(LineType::SoftOrSpace);
    }

    // Check if this is a set operation (UNION/INTERSECT/EXCEPT)
    // SetOperation: Undefined = 0, SetopNone = 1, SetopUnion = 2, SetopIntersect = 3, SetopExcept = 4
    if n.op > 1 {
        // Emit left operand
        if let Some(ref larg) = n.larg {
            emit_select_stmt_no_semicolon(e, larg);
        }

        // Emit set operation keyword
        e.line(LineType::SoftOrSpace);
        match n.op {
            2 => e.token(TokenKind::UNION_KW),     // SetopUnion
            3 => e.token(TokenKind::INTERSECT_KW), // SetopIntersect
            4 => e.token(TokenKind::EXCEPT_KW),    // SetopExcept
            _ => {}
        }

        // Emit ALL keyword if present
        if n.all {
            e.space();
            e.token(TokenKind::ALL_KW);
        }

        // Emit right operand
        e.line(LineType::SoftOrSpace);
        if let Some(ref rarg) = n.rarg {
            emit_select_stmt_no_semicolon(e, rarg);
        }

        if with_semicolon {
            e.token(TokenKind::SEMICOLON);
        }

        e.group_end();
        return;
    }

    // Check if this is a VALUES clause (used in INSERT statements)
    if !n.values_lists.is_empty() {
        e.token(TokenKind::VALUES_KW);
        e.space();

        // Emit each row of values
        emit_comma_separated_list(e, &n.values_lists, |row, e| {
            e.token(TokenKind::L_PAREN);
            super::emit_node(row, e);
            e.token(TokenKind::R_PAREN);
        });

        if with_semicolon {
            e.token(TokenKind::SEMICOLON);
        }
    } else {
        e.token(TokenKind::SELECT_KW);

        if !n.target_list.is_empty() {
            e.indent_start();
            e.line(LineType::SoftOrSpace);

            emit_comma_separated_list(e, &n.target_list, super::emit_node);

            e.indent_end();
        }

        // Emit INTO clause if present (SELECT ... INTO table_name)
        if let Some(ref into_clause) = n.into_clause {
            e.space();
            e.token(TokenKind::INTO_KW);
            e.space();
            if let Some(ref rel) = into_clause.rel {
                super::emit_range_var(e, rel);
            }
        }

        if !n.from_clause.is_empty() {
            e.line(LineType::SoftOrSpace);
            e.token(TokenKind::FROM_KW);
            e.line(LineType::SoftOrSpace);

            e.indent_start();

            emit_comma_separated_list(e, &n.from_clause, super::emit_node);

            e.indent_end();
        }

        if let Some(ref where_clause) = n.where_clause {
            e.line(LineType::SoftOrSpace);
            e.token(TokenKind::WHERE_KW);
            e.space();
            super::emit_node(where_clause, e);
        }

        // Emit GROUP BY clause if present
        if !n.group_clause.is_empty() {
            e.line(LineType::SoftOrSpace);
            e.token(TokenKind::GROUP_KW);
            e.space();
            e.token(TokenKind::BY_KW);
            e.space();
            e.indent_start();
            emit_comma_separated_list(e, &n.group_clause, super::emit_node);
            e.indent_end();
        }

        // Emit HAVING clause if present
        if let Some(ref having_clause) = n.having_clause {
            e.line(LineType::SoftOrSpace);
            e.token(TokenKind::HAVING_KW);
            e.space();
            super::emit_node(having_clause, e);
        }

        // Emit ORDER BY clause if present
        if !n.sort_clause.is_empty() {
            e.line(LineType::SoftOrSpace);
            e.token(TokenKind::ORDER_KW);
            e.space();
            e.token(TokenKind::BY_KW);
            e.space();
            e.indent_start();
            emit_comma_separated_list(e, &n.sort_clause, super::emit_node);
            e.indent_end();
        }

        // Emit LIMIT clause if present
        if let Some(ref limit_count) = n.limit_count {
            e.line(LineType::SoftOrSpace);
            e.token(TokenKind::LIMIT_KW);
            e.space();
            super::emit_node(limit_count, e);
        }

        // Emit OFFSET clause if present
        if let Some(ref limit_offset) = n.limit_offset {
            e.line(LineType::SoftOrSpace);
            e.token(TokenKind::OFFSET_KW);
            e.space();
            super::emit_node(limit_offset, e);
        }

        if with_semicolon {
            e.token(TokenKind::SEMICOLON);
        }
    }

    e.group_end();
}
