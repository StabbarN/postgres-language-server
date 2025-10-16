use pgt_query::protobuf::{AExpr, AExprKind};

use crate::{
    TokenKind,
    emitter::{EventEmitter, GroupKind},
};

pub(super) fn emit_a_expr(e: &mut EventEmitter, n: &AExpr) {
    e.group_start(GroupKind::AExpr);

    match n.kind() {
        AExprKind::AexprOp => emit_aexpr_op(e, n),
        AExprKind::AexprOpAny => emit_aexpr_op_any(e, n),
        AExprKind::AexprOpAll => emit_aexpr_op_all(e, n),
        AExprKind::AexprDistinct => emit_aexpr_distinct(e, n),
        AExprKind::AexprNotDistinct => emit_aexpr_not_distinct(e, n),
        AExprKind::AexprNullif => emit_aexpr_nullif(e, n),
        AExprKind::AexprIn => emit_aexpr_in(e, n),
        AExprKind::AexprLike => emit_aexpr_like(e, n),
        AExprKind::AexprIlike => emit_aexpr_ilike(e, n),
        AExprKind::AexprSimilar => emit_aexpr_similar(e, n),
        AExprKind::AexprBetween => emit_aexpr_between(e, n),
        AExprKind::AexprNotBetween => emit_aexpr_not_between(e, n),
        AExprKind::AexprBetweenSym => emit_aexpr_between_sym(e, n),
        AExprKind::AexprNotBetweenSym => emit_aexpr_not_between_sym(e, n),
        AExprKind::Undefined => {}
    }

    e.group_end();
}

// Basic binary operator: left op right
fn emit_aexpr_op(e: &mut EventEmitter, n: &AExpr) {
    if let Some(ref lexpr) = n.lexpr {
        super::emit_node(lexpr, e);
    }

    if !n.name.is_empty() {
        e.space();
        for name in &n.name {
            super::emit_node(name, e);
        }
        e.space();
    }

    if let Some(ref rexpr) = n.rexpr {
        super::emit_node(rexpr, e);
    }
}

// expr op ANY (subquery)
fn emit_aexpr_op_any(e: &mut EventEmitter, n: &AExpr) {
    if let Some(ref lexpr) = n.lexpr {
        super::emit_node(lexpr, e);
        e.space();
    }

    if !n.name.is_empty() {
        for name in &n.name {
            super::emit_node(name, e);
        }
        e.space();
    }

    e.token(TokenKind::ANY_KW);
    e.space();

    if let Some(ref rexpr) = n.rexpr {
        super::emit_node(rexpr, e);
    }
}

// expr op ALL (subquery)
fn emit_aexpr_op_all(e: &mut EventEmitter, n: &AExpr) {
    if let Some(ref lexpr) = n.lexpr {
        super::emit_node(lexpr, e);
        e.space();
    }

    if !n.name.is_empty() {
        for name in &n.name {
            super::emit_node(name, e);
        }
        e.space();
    }

    e.token(TokenKind::ALL_KW);
    e.space();

    if let Some(ref rexpr) = n.rexpr {
        super::emit_node(rexpr, e);
    }
}

// expr IS DISTINCT FROM expr2
fn emit_aexpr_distinct(e: &mut EventEmitter, n: &AExpr) {
    if let Some(ref lexpr) = n.lexpr {
        super::emit_node(lexpr, e);
        e.space();
    }

    e.token(TokenKind::IS_KW);
    e.space();
    e.token(TokenKind::DISTINCT_KW);
    e.space();
    e.token(TokenKind::FROM_KW);
    e.space();

    if let Some(ref rexpr) = n.rexpr {
        super::emit_node(rexpr, e);
    }
}

// expr IS NOT DISTINCT FROM expr2
fn emit_aexpr_not_distinct(e: &mut EventEmitter, n: &AExpr) {
    if let Some(ref lexpr) = n.lexpr {
        super::emit_node(lexpr, e);
        e.space();
    }

    e.token(TokenKind::IS_KW);
    e.space();
    e.token(TokenKind::NOT_KW);
    e.space();
    e.token(TokenKind::DISTINCT_KW);
    e.space();
    e.token(TokenKind::FROM_KW);
    e.space();

    if let Some(ref rexpr) = n.rexpr {
        super::emit_node(rexpr, e);
    }
}

// NULLIF(expr, expr2)
fn emit_aexpr_nullif(e: &mut EventEmitter, n: &AExpr) {
    e.token(TokenKind::NULLIF_KW);
    e.token(TokenKind::L_PAREN);

    if let Some(ref lexpr) = n.lexpr {
        super::emit_node(lexpr, e);
    }

    e.token(TokenKind::COMMA);
    e.space();

    if let Some(ref rexpr) = n.rexpr {
        super::emit_node(rexpr, e);
    }

    e.token(TokenKind::R_PAREN);
}

// expr IN (values)
fn emit_aexpr_in(e: &mut EventEmitter, n: &AExpr) {
    if let Some(ref lexpr) = n.lexpr {
        super::emit_node(lexpr, e);
        e.space();
    }

    e.token(TokenKind::IN_KW);
    e.space();

    // The rexpr is typically a List node, which emits comma-separated items
    // We need to wrap it in parentheses for IN clause
    e.token(TokenKind::L_PAREN);
    if let Some(ref rexpr) = n.rexpr {
        super::emit_node(rexpr, e);
    }
    e.token(TokenKind::R_PAREN);
}

// expr LIKE pattern [ESCAPE escape]
fn emit_aexpr_like(e: &mut EventEmitter, n: &AExpr) {
    if let Some(ref lexpr) = n.lexpr {
        super::emit_node(lexpr, e);
        e.space();
    }

    e.token(TokenKind::LIKE_KW);
    e.space();

    if let Some(ref rexpr) = n.rexpr {
        super::emit_node(rexpr, e);
    }
}

// expr ILIKE pattern [ESCAPE escape]
fn emit_aexpr_ilike(e: &mut EventEmitter, n: &AExpr) {
    if let Some(ref lexpr) = n.lexpr {
        super::emit_node(lexpr, e);
        e.space();
    }

    e.token(TokenKind::ILIKE_KW);
    e.space();

    if let Some(ref rexpr) = n.rexpr {
        super::emit_node(rexpr, e);
    }
}

// expr SIMILAR TO pattern [ESCAPE escape]
fn emit_aexpr_similar(e: &mut EventEmitter, n: &AExpr) {
    if let Some(ref lexpr) = n.lexpr {
        super::emit_node(lexpr, e);
        e.space();
    }

    e.token(TokenKind::SIMILAR_KW);
    e.space();
    e.token(TokenKind::TO_KW);
    e.space();

    if let Some(ref rexpr) = n.rexpr {
        super::emit_node(rexpr, e);
    }
}

// expr BETWEEN expr2 AND expr3
fn emit_aexpr_between(e: &mut EventEmitter, n: &AExpr) {
    if let Some(ref lexpr) = n.lexpr {
        super::emit_node(lexpr, e);
        e.space();
    }

    e.token(TokenKind::BETWEEN_KW);
    e.space();

    // rexpr is a List node with two elements, but we need "expr AND expr" not "expr, expr"
    if let Some(ref rexpr) = n.rexpr {
        if let Some(pgt_query::NodeEnum::List(list)) = rexpr.node.as_ref() {
            if list.items.len() >= 1 {
                super::emit_node(&list.items[0], e);
            }
            if list.items.len() >= 2 {
                e.space();
                e.token(TokenKind::AND_KW);
                e.space();
                super::emit_node(&list.items[1], e);
            }
        } else {
            super::emit_node(rexpr, e);
        }
    }
}

// expr NOT BETWEEN expr2 AND expr3
fn emit_aexpr_not_between(e: &mut EventEmitter, n: &AExpr) {
    if let Some(ref lexpr) = n.lexpr {
        super::emit_node(lexpr, e);
        e.space();
    }

    e.token(TokenKind::NOT_KW);
    e.space();
    e.token(TokenKind::BETWEEN_KW);
    e.space();

    // rexpr is a List node with two elements, but we need "expr AND expr" not "expr, expr"
    if let Some(ref rexpr) = n.rexpr {
        if let Some(pgt_query::NodeEnum::List(list)) = rexpr.node.as_ref() {
            if list.items.len() >= 1 {
                super::emit_node(&list.items[0], e);
            }
            if list.items.len() >= 2 {
                e.space();
                e.token(TokenKind::AND_KW);
                e.space();
                super::emit_node(&list.items[1], e);
            }
        } else {
            super::emit_node(rexpr, e);
        }
    }
}

// expr BETWEEN SYMMETRIC expr2 AND expr3
fn emit_aexpr_between_sym(e: &mut EventEmitter, n: &AExpr) {
    if let Some(ref lexpr) = n.lexpr {
        super::emit_node(lexpr, e);
        e.space();
    }

    e.token(TokenKind::BETWEEN_KW);
    e.space();
    e.token(TokenKind::SYMMETRIC_KW);
    e.space();

    // rexpr is a List node with two elements, but we need "expr AND expr" not "expr, expr"
    if let Some(ref rexpr) = n.rexpr {
        if let Some(pgt_query::NodeEnum::List(list)) = rexpr.node.as_ref() {
            if list.items.len() >= 1 {
                super::emit_node(&list.items[0], e);
            }
            if list.items.len() >= 2 {
                e.space();
                e.token(TokenKind::AND_KW);
                e.space();
                super::emit_node(&list.items[1], e);
            }
        } else {
            super::emit_node(rexpr, e);
        }
    }
}

// expr NOT BETWEEN SYMMETRIC expr2 AND expr3
fn emit_aexpr_not_between_sym(e: &mut EventEmitter, n: &AExpr) {
    if let Some(ref lexpr) = n.lexpr {
        super::emit_node(lexpr, e);
        e.space();
    }

    e.token(TokenKind::NOT_KW);
    e.space();
    e.token(TokenKind::BETWEEN_KW);
    e.space();
    e.token(TokenKind::SYMMETRIC_KW);
    e.space();

    // rexpr is a List node with two elements, but we need "expr AND expr" not "expr, expr"
    if let Some(ref rexpr) = n.rexpr {
        if let Some(pgt_query::NodeEnum::List(list)) = rexpr.node.as_ref() {
            if list.items.len() >= 1 {
                super::emit_node(&list.items[0], e);
            }
            if list.items.len() >= 2 {
                e.space();
                e.token(TokenKind::AND_KW);
                e.space();
                super::emit_node(&list.items[1], e);
            }
        } else {
            super::emit_node(rexpr, e);
        }
    }
}
