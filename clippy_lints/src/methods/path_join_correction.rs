use clippy_utils::diagnostics::span_lint_and_sugg;
use rustc_ast::ast::LitKind;
use rustc_errors::Applicability;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::LateContext;
use rustc_span::Span;

use super::PATH_JOIN_CORRECTION;

pub(super) fn check<'tcx>(cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>, join_arg: &'tcx Expr<'tcx>, span: Span) {
    let applicability = Applicability::MachineApplicable;
    if_chain!(
      if let ExprKind::Lit(spanned) = &join_arg.kind;
      if let LitKind::Str(symbol, _) = spanned.node;
      if symbol.as_str().starts_with('/');
      then {
        span_lint_and_sugg(
      cx,
      PATH_JOIN_CORRECTION,
      span.with_hi(expr.span.hi()),
      r#"argument in join called on path contains a starting '/'"#,
      "try removing first '/'",
      "join(\"your/path/here\")".to_owned(),
      applicability
        );
      }
    );
}
