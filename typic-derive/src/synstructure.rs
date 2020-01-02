// Copyright 2016 Nika Layzell
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
// of the Software, and to permit persons to whom the Software is furnished to do
// so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use syn::*;
use syn::visit::{self, Visit};

pub fn get_ty_params<'t, F>(fields: F, generics: &Generics) -> Vec<TypeParam>
where F: IntoIterator<Item=&'t Field>
{
    // Helper type. Discovers all identifiers inside of the visited type,
    // and calls a callback with them.
    struct BoundTypeLocator<'a> {
        result: Vec<bool>,
        generics: &'a Generics,
    }

    impl<'a> Visit<'a> for BoundTypeLocator<'a> {
        // XXX: This also (intentionally) captures paths like T::SomeType. Is
        // this desirable?
        fn visit_ident(&mut self, id: &Ident) {
            for (idx, i) in self.generics.params.iter().enumerate() {
                if let GenericParam::Type(tparam) = i {
                    if tparam.ident == *id {
                        self.result[idx] = true;
                    }
                }
            }
        }

        fn visit_type_macro(&mut self, x: &'a TypeMacro) {
            // If we see a type_mac declaration, then we can't know what type parameters
            // it might be binding, so we presume it binds all of them.
            for r in &mut self.result {
                *r = true;
            }
            visit::visit_type_macro(self, x)
        }
    }

    let mut btl = BoundTypeLocator {
        result: vec![false; generics.params.len()],
        generics,
    };

    for field in fields {
      btl.visit_type(&field.ty);
    }

    generics.type_params().enumerate()
      .filter(|&(i, g):&(usize,_)| btl.result[i])
      .map(|(i, g)| g.clone())
      .collect()
}