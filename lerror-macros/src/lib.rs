use proc_macro::TokenStream;
use quote::quote;
use syn::visit_mut::{self, VisitMut};
use syn::{parse_macro_input, ExprTry, ItemFn};

#[proc_macro_attribute]
pub fn lerror_trace(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(input as ItemFn);

    // Transform all try operators (?) in the function to use context
    transform_try_operators(&mut input_fn);

    let output = quote! {
        #input_fn
    };
    output.into()
}

fn transform_try_operators(input_fn: &mut ItemFn) {
    let mut transformer = TryTransformer;
    transformer.visit_item_fn_mut(input_fn);
}

struct TryTransformer;

impl VisitMut for TryTransformer {
    fn visit_expr_try_mut(&mut self, i: &mut ExprTry) {
        // First recursively process any nested try expressions
        visit_mut::visit_expr_try_mut(self, i);

        // Get the inner expression and check if it's already a context call
        let inner_expr = &i.expr;
        if let syn::Expr::MethodCall(method_call) = &**inner_expr {
            if method_call.method == "context" {
                return;
            }
        }

        // Replace `expr?` with `expr.context("")?`
        let new_expr = syn::parse_quote! {
            #inner_expr.context("")
        };

        *i.expr = new_expr;
    }
}
