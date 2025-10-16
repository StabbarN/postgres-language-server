use pgt_query::protobuf::{InferClause, OnConflictClause};

use crate::TokenKind;
use crate::emitter::EventEmitter;

use super::node_list::emit_comma_separated_list;
use super::res_target::emit_set_clause;

pub(super) fn emit_on_conflict_clause(e: &mut EventEmitter, n: &OnConflictClause) {
    e.space();
    e.token(TokenKind::ON_KW);
    e.space();
    e.token(TokenKind::IDENT("CONFLICT".to_string()));

    // Emit the inference clause (target columns or constraint name)
    if let Some(ref infer) = n.infer {
        emit_infer_clause(e, infer);
    }

    // Emit the action (DO NOTHING or DO UPDATE SET)
    e.space();
    e.token(TokenKind::DO_KW);
    e.space();

    match n.action {
        2 => {
            // OnconflictNothing
            e.token(TokenKind::IDENT("NOTHING".to_string()));
        }
        3 => {
            // OnconflictUpdate
            e.token(TokenKind::UPDATE_KW);
            e.space();
            e.token(TokenKind::SET_KW);
            e.space();

            // Emit the SET clause (target_list)
            if !n.target_list.is_empty() {
                emit_comma_separated_list(e, &n.target_list, |node, e| {
                    if let Some(pgt_query::NodeEnum::ResTarget(res_target)) = node.node.as_ref() {
                        emit_set_clause(e, res_target);
                    } else {
                        super::emit_node(node, e);
                    }
                });
            }

            // Emit WHERE clause if present
            if let Some(ref where_clause) = n.where_clause {
                e.space();
                e.token(TokenKind::WHERE_KW);
                e.space();
                super::emit_node(where_clause, e);
            }
        }
        _ => {
            // Undefined or OnconflictNone - should not happen in valid SQL
        }
    }
}

fn emit_infer_clause(e: &mut EventEmitter, n: &InferClause) {
    // Emit constraint name if present
    if !n.conname.is_empty() {
        e.space();
        e.token(TokenKind::ON_KW);
        e.space();
        e.token(TokenKind::IDENT("CONSTRAINT".to_string()));
        e.space();
        e.token(TokenKind::IDENT(n.conname.clone()));
    } else if !n.index_elems.is_empty() {
        // Emit index elements (columns)
        e.space();
        e.token(TokenKind::L_PAREN);
        emit_comma_separated_list(e, &n.index_elems, super::emit_node);
        e.token(TokenKind::R_PAREN);
    }

    // Emit WHERE clause if present
    if let Some(ref where_clause) = n.where_clause {
        e.space();
        e.token(TokenKind::WHERE_KW);
        e.space();
        super::emit_node(where_clause, e);
    }
}
