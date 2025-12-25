use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse_macro_input, spanned::Spanned, visit_mut::VisitMut, Error, ExprMatch, Item, ItemFn, Pat,
    Result,
};

#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    let item = parse_macro_input!(input as Item);

    match sorted_impl(&item) {
        Ok(tokens) => tokens.into(),
        Err(e) => {
            // Return both the error and the original item so compilation can continue
            let item_tokens = quote! { #item };
            let error_tokens = e.to_compile_error();
            quote! {
                #error_tokens
                #item_tokens
            }
            .into()
        }
    }
}

fn sorted_impl(item: &Item) -> Result<proc_macro2::TokenStream> {
    match item {
        Item::Enum(item_enum) => {
            // Check that variants are sorted
            let variants: Vec<_> = item_enum.variants.iter().collect();

            for i in 1..variants.len() {
                let prev_name = &variants[i - 1].ident;
                let curr_name = &variants[i].ident;

                if curr_name < prev_name {
                    // Find what this should sort before
                    let should_before = variants[..i]
                        .iter()
                        .find(|v| curr_name < &v.ident)
                        .map(|v| &v.ident)
                        .unwrap_or(prev_name);

                    return Err(Error::new(
                        curr_name.span(),
                        format!("{} should sort before {}", curr_name, should_before),
                    ));
                }
            }

            Ok(quote! { #item })
        }
        _ => Err(Error::new_spanned(
            quote! { #[sorted] },
            "expected enum or match expression",
        )),
    }
}

#[proc_macro_attribute]
pub fn check(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    let mut item_fn = parse_macro_input!(input as ItemFn);

    match check_impl(&mut item_fn) {
        Ok(()) => quote! { #item_fn }.into(),
        Err(e) => {
            let item_tokens = quote! { #item_fn };
            let error_tokens = e.to_compile_error();
            quote! {
                #error_tokens
                #item_tokens
            }
            .into()
        }
    }
}

fn check_impl(item_fn: &mut ItemFn) -> Result<()> {
    let mut visitor = SortedChecker { errors: Vec::new() };
    visitor.visit_item_fn_mut(item_fn);

    if let Some(err) = visitor.errors.into_iter().next() {
        return Err(err);
    }
    Ok(())
}

struct SortedChecker {
    errors: Vec<Error>,
}

impl VisitMut for SortedChecker {
    fn visit_expr_match_mut(&mut self, expr: &mut ExprMatch) {
        // Check if this match has a #[sorted] attribute
        let sorted_idx = expr
            .attrs
            .iter()
            .position(|attr| attr.path().is_ident("sorted"));

        if let Some(idx) = sorted_idx {
            // Remove the #[sorted] attribute
            expr.attrs.remove(idx);

            // Check that the match arms are sorted
            if let Err(e) = check_match_arms_sorted(&expr.arms) {
                self.errors.push(e);
            }
        }

        // Continue visiting nested expressions
        syn::visit_mut::visit_expr_match_mut(self, expr);
    }
}

fn check_match_arms_sorted(arms: &[syn::Arm]) -> Result<()> {
    // Extract arm names, handling patterns
    let mut arm_names: Vec<(String, Span)> = Vec::new();

    for arm in arms {
        match get_pattern_name(&arm.pat) {
            PatternKind::Name(name, span) => {
                arm_names.push((name, span));
            }
            PatternKind::Wild => {
                // Wildcard should be last, don't add to sorting check
            }
            PatternKind::Unsupported(span) => {
                return Err(Error::new(span, "unsupported by #[sorted]"));
            }
        }
    }

    // Check sorting
    for i in 1..arm_names.len() {
        let (prev_name, _) = &arm_names[i - 1];
        let (curr_name, curr_span) = &arm_names[i];

        if curr_name < prev_name {
            // Find what this should sort before
            let should_before = arm_names[..i]
                .iter()
                .find(|(n, _)| curr_name < n)
                .map(|(n, _)| n.as_str())
                .unwrap_or(prev_name.as_str());

            return Err(Error::new(
                *curr_span,
                format!("{} should sort before {}", curr_name, should_before),
            ));
        }
    }

    Ok(())
}

enum PatternKind {
    Name(String, Span),
    Wild,
    Unsupported(Span),
}

fn get_pattern_name(pat: &Pat) -> PatternKind {
    match pat {
        Pat::Ident(ident) => PatternKind::Name(ident.ident.to_string(), ident.ident.span()),
        Pat::Path(path) => {
            // Error::Fmt or similar
            let full_path = path_to_string(&path.path);
            PatternKind::Name(full_path, path_span(&path.path))
        }
        Pat::TupleStruct(tuple_struct) => {
            // Error::Fmt(e) or similar
            let full_path = path_to_string(&tuple_struct.path);
            PatternKind::Name(full_path, path_span(&tuple_struct.path))
        }
        Pat::Struct(pat_struct) => {
            // Error::Fmt { .. } or similar
            let full_path = path_to_string(&pat_struct.path);
            PatternKind::Name(full_path, path_span(&pat_struct.path))
        }
        Pat::Wild(_) => PatternKind::Wild,
        _ => PatternKind::Unsupported(pat.span()),
    }
}

/// Get a span covering the entire path (all segments)
fn path_span(path: &syn::Path) -> Span {
    // Get the span of the last segment's ident
    if let Some(last_seg) = path.segments.last() {
        // Use the last segment's ident span - this gives us a span that ends at the right place
        last_seg.ident.span()
    } else {
        path.span()
    }
}

fn path_to_string(path: &syn::Path) -> String {
    path.segments
        .iter()
        .map(|seg| seg.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}
