use pgt_query::{NodeEnum, protobuf::ScalarArrayOpExpr};

use crate::{
    TokenKind,
    emitter::{EventEmitter, GroupKind},
};

use super::node_list::emit_comma_separated_list;

pub(super) fn emit_scalar_array_op_expr(e: &mut EventEmitter, n: &ScalarArrayOpExpr) {
    e.group_start(GroupKind::ScalarArrayOpExpr);

    // ScalarArrayOpExpr is used for "expr op ANY/ALL (array)" constructs
    // Common case: id IN (1, 2, 3) becomes: id = ANY(ARRAY[1, 2, 3])
    // However, we want to emit it as the more readable "id IN (values)" form

    // args[0] is the left operand (e.g., id)
    // args[1] is the right operand (e.g., the array)

    if n.args.len() >= 2 {
        // Emit left operand
        super::emit_node(&n.args[0], e);
        e.space();

        // For IN operator (use_or=true), emit as "IN (values)"
        // For other operators, might need different handling
        if n.use_or {
            e.token(TokenKind::IN_KW);
        } else {
            // NOT IN case - emit as "NOT IN (values)"
            e.token(TokenKind::NOT_KW);
            e.space();
            e.token(TokenKind::IN_KW);
        }
        e.space();

        // Emit the array/list
        // The right operand is typically an AArrayExpr (ARRAY[...])
        // For IN clause, we want to emit it as (values) not ARRAY[values]
        if let Some(NodeEnum::AArrayExpr(array_expr)) = &n.args[1].node {
            // Emit as (value1, value2, ...) instead of ARRAY[...]
            e.token(TokenKind::L_PAREN);
            if !array_expr.elements.is_empty() {
                emit_comma_separated_list(e, &array_expr.elements, super::emit_node);
            }
            e.token(TokenKind::R_PAREN);
        } else {
            // For other cases (subqueries, etc.), emit as-is
            super::emit_node(&n.args[1], e);
        }
    }

    e.group_end();
}
