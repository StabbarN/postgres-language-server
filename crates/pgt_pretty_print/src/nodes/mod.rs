mod a_const;
mod a_expr;
mod a_star;
mod bitstring;
mod bool_expr;
mod boolean;
mod column_ref;
mod float;
mod integer;
mod node_list;
mod range_var;
mod res_target;
mod select_stmt;
mod string;
mod update_stmt;

use a_const::emit_a_const;
use a_expr::emit_a_expr;
use a_star::emit_a_star;
use bitstring::emit_bitstring;
use bool_expr::emit_bool_expr;
use boolean::emit_boolean;
use column_ref::emit_column_ref;
use float::emit_float;
use integer::emit_integer;
use range_var::emit_range_var;
use res_target::emit_res_target;
use select_stmt::emit_select_stmt;
use string::{emit_string, emit_string_identifier, emit_string_literal};
use update_stmt::emit_update_stmt;

use crate::emitter::EventEmitter;
use pgt_query::{NodeEnum, protobuf::Node};

pub fn emit_node(node: &Node, e: &mut EventEmitter) {
    if let Some(ref inner) = node.node {
        emit_node_enum(inner, e)
    }
}

pub fn emit_node_enum(node: &NodeEnum, e: &mut EventEmitter) {
    match &node {
        NodeEnum::SelectStmt(n) => emit_select_stmt(e, n),
        NodeEnum::UpdateStmt(n) => emit_update_stmt(e, n),
        NodeEnum::ResTarget(n) => emit_res_target(e, n),
        NodeEnum::ColumnRef(n) => emit_column_ref(e, n),
        NodeEnum::String(n) => emit_string(e, n),
        NodeEnum::RangeVar(n) => emit_range_var(e, n),
        NodeEnum::AConst(n) => emit_a_const(e, n),
        NodeEnum::Integer(n) => emit_integer(e, n),
        NodeEnum::Float(n) => emit_float(e, n),
        NodeEnum::Boolean(n) => emit_boolean(e, n),
        NodeEnum::BitString(n) => emit_bitstring(e, n),
        NodeEnum::AExpr(n) => emit_a_expr(e, n),
        NodeEnum::AStar(n) => emit_a_star(e, n),
        NodeEnum::BoolExpr(n) => emit_bool_expr(e, n),
        _ => todo!("emit_node_enum: unhandled node type {:?}", node),
    }
}
