use std::fmt;

use rustc_hir::def_id::{DefId, LocalDefId};
use rustc_middle::{
    thir::{self, ExprKind, Thir},
    ty::{self, TyCtxt},
};

use crate::{
    ctx::RmlCtxt,
    error::{Error, RmlErr},
    util,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Purity {
    Impure,
    Pure,
    StrictlyPure,
}

impl Purity {
    pub fn may_call(&self, other: Purity) -> bool {
        use Purity::*;
        match (self, &other) {
            (Impure, _) => true,
            (Pure, StrictlyPure) => true,
            (p1, p2) if p1 == p2 => true,
            _ => false,
        }
    }
}

impl fmt::Display for Purity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Purity::Impure => write!(f, "impure"),
            Purity::Pure => write!(f, "pure"),
            Purity::StrictlyPure => write!(f, "strictly pure"),
        }
    }
}

impl<'tcx> RmlCtxt<'tcx> {
    pub fn check_purity(&self, did: LocalDefId) {
        let tcx = self.tcx;
        let (thir, expr) = tcx
            .thir_body(did)
            .unwrap_or_else(|_| Error::from(RmlErr).emit(tcx.sess));
        let thir = thir.borrow();
        if thir.exprs.is_empty() {
            Error::new(tcx.def_span(did), "type checking failed").emit(tcx.sess);
        }

        let did = did.to_def_id();
        let purity = get_purity(tcx, did);

        thir::visit::walk_expr(
            &mut PurityVisitor {
                tcx,
                thir: &thir,
                purity,
            },
            &thir[expr],
        )
    }
}

pub(crate) fn get_purity<'tcx>(tcx: TyCtxt<'tcx>, did: DefId) -> Purity {
    // Theoretically, since logic and spec functions may call pure functions, they are not strictly pure.
    // But, because they are not "really" called, they do not change the memory and are, thus, strictly pure
    if util::is_spec(tcx, did)
        || util::is_logic(tcx, did)
        || util::is_declared_strictly_pure(tcx, did)
    {
        Purity::StrictlyPure
    } else if util::is_declared_pure(tcx, did) {
        Purity::Pure
    } else {
        Purity::Impure
    }
}

pub(crate) struct PurityVisitor<'a, 'tcx> {
    pub(crate) tcx: TyCtxt<'tcx>,
    pub(crate) thir: &'a Thir<'tcx>,
    pub(crate) purity: Purity,
}

impl<'a, 'tcx> thir::visit::Visitor<'a, 'tcx> for PurityVisitor<'a, 'tcx> {
    fn thir(&self) -> &'a Thir<'tcx> {
        self.thir
    }

    fn visit_expr(&mut self, expr: &thir::Expr<'tcx>) {
        match expr.kind {
            ExprKind::Call { fun, .. } => {
                if let &ty::FnDef(func_did, _) = self.thir[fun].ty.kind() {
                    let called_purity = get_purity(self.tcx, func_did);
                    if !self.purity.may_call(called_purity) {
                        let msg = format!(
                            "called {} function '{}' from {} function",
                            called_purity,
                            self.tcx.def_path_str(func_did),
                            self.purity
                        );

                        self.tcx.sess.span_err_with_code(
                            self.thir[fun].span,
                            msg,
                            rustc_errors::DiagnosticId::Error(String::from("rml")),
                        );
                    }
                } else {
                    todo!("Why is this an error? {fun:?}")
                }
            }
            _ => {}
        }
        thir::visit::walk_expr(self, expr)
    }
}