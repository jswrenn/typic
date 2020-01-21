use syn::visit::{self, Visit};
use syn::*;
use std::collections::HashSet as Set;

pub fn get_params<'t, F>(fields: F, params: &'t Generics) -> Generics
where
    F: IntoIterator<Item = &'t Field>,
{
    struct BoundParamsVisitor<'a> {
        params: &'a Generics,
        bound_lifetimes: Set<&'a LifetimeDef>,
        bound_type_params: Set<&'a TypeParam>,
    }

    impl<'a> Visit<'a> for BoundParamsVisitor<'a> {
        fn visit_lifetime(&mut self, lifetime: &Lifetime) {
            self.bound_lifetimes.extend(
                self.params.lifetimes()
                    .filter(|param| param.lifetime == *lifetime));
        }

        fn visit_ident(&mut self, id: &Ident) {
            self.bound_type_params.extend(
                self.params.type_params()
                    .filter(|param| param.ident == *id));
        }
    }

    let mut btl = BoundParamsVisitor {
        params,
        bound_lifetimes: Set::default(),
        bound_type_params: Set::default(),
    };

    for field in fields {
        btl.visit_type(&field.ty);
    }

    let lifetimes =
        btl.bound_lifetimes.into_iter().cloned()
          .map(|param| GenericParam::Lifetime(param));

    let type_params =
        btl.bound_type_params.into_iter().cloned()
          .map(|param| GenericParam::Type(param));

    Generics {
        lt_token: btl.params.lt_token,
        gt_token: btl.params.gt_token,
        where_clause: btl.params.where_clause.clone(),

        params:
          lifetimes.chain(type_params).collect()
    }
}
