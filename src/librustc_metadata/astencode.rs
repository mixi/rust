// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use isolated_encoder::IsolatedEncoder;
use schema::*;

use rustc::hir;
use rustc::ty;

#[derive(RustcEncodable, RustcDecodable)]
pub struct Ast<'tcx> {
    pub tables: Lazy<ty::TypeckTables<'tcx>>,
    pub rvalue_promotable_to_static: bool,
}

impl_stable_hash_for!(struct Ast<'tcx> {
    tables,
    rvalue_promotable_to_static
});

impl<'a, 'b, 'tcx> IsolatedEncoder<'a, 'b, 'tcx> {
    pub fn encode_body(&mut self, body_id: hir::BodyId) -> Lazy<Ast<'tcx>> {
        let body_owner_def_id = self.tcx.hir.body_owner_def_id(body_id);
        let tables = self.tcx.typeck_tables_of(body_owner_def_id);
        let lazy_tables = self.lazy(tables);

        let rvalue_promotable_to_static =
            self.tcx.const_is_rvalue_promotable_to_static(body_owner_def_id);

        self.lazy(&Ast {
            tables: lazy_tables,
            rvalue_promotable_to_static,
        })
    }
}
