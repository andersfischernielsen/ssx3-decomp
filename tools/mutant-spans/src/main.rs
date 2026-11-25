//! mutant-spans FILE: print one line per expression in FILE, for tools/mutants.py to switch
//! each mutant at the smallest expression around it.
//!
//!   start_line <TAB> start_col <TAB> end_line <TAB> end_col <TAB> flags
//!
//! Lines are 1-based; columns are 1-based characters with an exclusive end, the way
//! cargo-mutants prints spans. Flags: `s` the expression is a statement of its block (it may
//! be written as a bare `if` there), `c` it sits in a const context (const/static item,
//! const fn, array length, enum discriminant, pattern, const generic argument), where a
//! runtime switch cannot go, `m` it is a macro call (its tokens are not parsed).
use proc_macro2::LineColumn;
use std::collections::HashSet;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};

struct V {
    const_depth: usize,
    stmt_spans: HashSet<(usize, usize, usize, usize)>,
    out: String,
}

fn key(s: LineColumn, e: LineColumn) -> (usize, usize, usize, usize) {
    (s.line, s.column, e.line, e.column)
}

impl V {
    fn in_const<F: FnOnce(&mut Self)>(&mut self, f: F) {
        self.const_depth += 1;
        f(self);
        self.const_depth -= 1;
    }
}

impl<'ast> Visit<'ast> for V {
    fn visit_expr(&mut self, e: &'ast syn::Expr) {
        let sp = e.span();
        let (s, t) = (sp.start(), sp.end());
        let mut flags = String::new();
        if self.stmt_spans.contains(&key(s, t)) {
            flags.push('s');
        }
        if self.const_depth > 0 {
            flags.push('c');
        }
        if matches!(e, syn::Expr::Macro(_)) {
            flags.push('m');
        }
        self.out.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\n",
            s.line,
            s.column + 1,
            t.line,
            t.column + 1,
            if flags.is_empty() { "-" } else { &flags }
        ));
        match e {
            // The length of `[x; N]` is a const context; the element is not.
            syn::Expr::Repeat(r) => {
                self.visit_expr(&r.expr);
                self.in_const(|v| v.visit_expr(&r.len));
            }
            // `const { .. }` blocks.
            syn::Expr::Const(c) => self.in_const(|v| visit::visit_expr_const(v, c)),
            _ => visit::visit_expr(self, e),
        }
    }
    fn visit_stmt(&mut self, s: &'ast syn::Stmt) {
        if let syn::Stmt::Expr(e, _) = s {
            let sp = e.span();
            self.stmt_spans.insert(key(sp.start(), sp.end()));
        }
        visit::visit_stmt(self, s);
    }
    fn visit_item_const(&mut self, i: &'ast syn::ItemConst) {
        self.in_const(|v| visit::visit_item_const(v, i));
    }
    fn visit_item_static(&mut self, i: &'ast syn::ItemStatic) {
        self.in_const(|v| visit::visit_item_static(v, i));
    }
    fn visit_impl_item_const(&mut self, i: &'ast syn::ImplItemConst) {
        self.in_const(|v| visit::visit_impl_item_const(v, i));
    }
    fn visit_trait_item_const(&mut self, i: &'ast syn::TraitItemConst) {
        self.in_const(|v| visit::visit_trait_item_const(v, i));
    }
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        if i.sig.constness.is_some() {
            self.in_const(|v| visit::visit_item_fn(v, i));
        } else {
            visit::visit_item_fn(self, i);
        }
    }
    fn visit_impl_item_fn(&mut self, i: &'ast syn::ImplItemFn) {
        if i.sig.constness.is_some() {
            self.in_const(|v| visit::visit_impl_item_fn(v, i));
        } else {
            visit::visit_impl_item_fn(self, i);
        }
    }
    fn visit_variant(&mut self, i: &'ast syn::Variant) {
        self.in_const(|v| visit::visit_variant(v, i));
    }
    fn visit_type(&mut self, i: &'ast syn::Type) {
        self.in_const(|v| visit::visit_type(v, i));
    }
    fn visit_pat(&mut self, i: &'ast syn::Pat) {
        self.in_const(|v| visit::visit_pat(v, i));
    }
    fn visit_generic_argument(&mut self, i: &'ast syn::GenericArgument) {
        self.in_const(|v| visit::visit_generic_argument(v, i));
    }
    fn visit_attribute(&mut self, _: &'ast syn::Attribute) {}
}

fn main() {
    let path = std::env::args().nth(1).expect("usage: mutant-spans FILE");
    let src = std::fs::read_to_string(&path).expect("read");
    let file = syn::parse_file(&src).expect("parse");
    let mut v = V {
        const_depth: 0,
        stmt_spans: HashSet::new(),
        out: String::new(),
    };
    v.visit_file(&file);
    print!("{}", v.out);
}
