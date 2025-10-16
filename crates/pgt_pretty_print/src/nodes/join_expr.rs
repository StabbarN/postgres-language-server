use pgt_query::protobuf::{JoinExpr, JoinType};

use crate::TokenKind;
use crate::emitter::{EventEmitter, GroupKind};

use super::node_list::emit_comma_separated_list;
use super::string::emit_identifier;

pub(super) fn emit_join_expr(e: &mut EventEmitter, n: &JoinExpr) {
    e.group_start(GroupKind::JoinExpr);

    // Left side
    if let Some(ref larg) = n.larg {
        super::emit_node(larg, e);
    }

    // NATURAL keyword
    if n.is_natural {
        e.space();
        e.token(TokenKind::NATURAL_KW);
    }

    // Join type
    match n.jointype {
        x if x == JoinType::JoinInner as i32 => {
            if !n.is_natural {
                e.space();
                e.token(TokenKind::INNER_KW);
            }
        }
        x if x == JoinType::JoinLeft as i32 => {
            e.space();
            e.token(TokenKind::LEFT_KW);
            if !n.is_natural {
                e.space();
                e.token(TokenKind::OUTER_KW);
            }
        }
        x if x == JoinType::JoinRight as i32 => {
            e.space();
            e.token(TokenKind::RIGHT_KW);
            if !n.is_natural {
                e.space();
                e.token(TokenKind::OUTER_KW);
            }
        }
        x if x == JoinType::JoinFull as i32 => {
            e.space();
            e.token(TokenKind::FULL_KW);
            if !n.is_natural {
                e.space();
                e.token(TokenKind::OUTER_KW);
            }
        }
        x if x == JoinType::JoinSemi as i32 => {
            e.space();
            e.token(TokenKind::IDENT("SEMI".to_string()));
        }
        x if x == JoinType::JoinAnti as i32 => {
            e.space();
            e.token(TokenKind::IDENT("ANTI".to_string()));
        }
        x if x == JoinType::JoinRightAnti as i32 => {
            e.space();
            e.token(TokenKind::RIGHT_KW);
            e.space();
            e.token(TokenKind::IDENT("ANTI".to_string()));
        }
        _ => {
            // CROSS JOIN or other types
            e.space();
            e.token(TokenKind::CROSS_KW);
        }
    }

    e.space();
    e.token(TokenKind::JOIN_KW);

    // Right side
    if let Some(ref rarg) = n.rarg {
        e.space();
        super::emit_node(rarg, e);
    }

    // Join qualification
    if !n.using_clause.is_empty() {
        e.space();
        e.token(TokenKind::USING_KW);
        e.space();
        e.token(TokenKind::L_PAREN);
        emit_comma_separated_list(e, &n.using_clause, |node, e| {
            // For USING clause, String nodes should be identifiers
            if let Some(pgt_query::NodeEnum::String(s)) = node.node.as_ref() {
                emit_identifier(e, &s.sval);
            } else {
                super::emit_node(node, e);
            }
        });
        e.token(TokenKind::R_PAREN);
    } else if let Some(ref quals) = n.quals {
        e.space();
        e.token(TokenKind::ON_KW);
        e.space();
        super::emit_node(quals, e);
    } else if n.jointype == JoinType::JoinInner as i32 && !n.is_natural {
        // For INNER JOIN without qualifications (converted from CROSS JOIN), add ON TRUE
        // This is semantically equivalent to CROSS JOIN
        e.space();
        e.token(TokenKind::ON_KW);
        e.space();
        e.token(TokenKind::TRUE_KW);
    }

    e.group_end();
}
