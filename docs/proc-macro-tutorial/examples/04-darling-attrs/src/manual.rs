//! Manual attribute parsing for comparison with darling.
//!
//! This module shows how much code you'd need WITHOUT darling.
//! Spoiler: it's a lot more.

use syn::{Attribute, Expr, ExprLit, Field, Lit, Meta, Token};
use syn::punctuated::Punctuated;

/// Configuration parsed from a field's attributes (manually).
#[derive(Default)]
pub struct FieldConfig {
    pub env: Option<String>,
    pub default: Option<String>,
    pub skip: bool,
}

/// Parse field attributes manually. Compare this to the darling version!
///
/// For `#[config(env = "DATABASE_URL", default = "localhost")]`:
pub fn parse_field_config(field: &Field) -> Result<FieldConfig, syn::Error> {
    let mut config = FieldConfig::default();
    
    for attr in &field.attrs {
        // Check if this is our attribute
        if !attr.path().is_ident("config") {
            continue;
        }
        
        // Parse the meta
        match &attr.meta {
            Meta::Path(_) => {
                // Just #[config] with no arguments - skip this field
                config.skip = true;
            }
            Meta::List(meta_list) => {
                // Parse #[config(...)]
                let nested = meta_list.parse_args_with(
                    Punctuated::<Meta, Token![,]>::parse_terminated
                )?;
                
                for meta in nested {
                    match meta {
                        Meta::Path(path) => {
                            if path.is_ident("skip") {
                                config.skip = true;
                            } else {
                                return Err(syn::Error::new_spanned(
                                    path, "Unknown attribute"
                                ));
                            }
                        }
                        Meta::NameValue(nv) => {
                            if nv.path.is_ident("env") {
                                config.env = Some(extract_string(&nv.value)?);
                            } else if nv.path.is_ident("default") {
                                config.default = Some(extract_string(&nv.value)?);
                            } else {
                                return Err(syn::Error::new_spanned(
                                    nv.path, "Unknown attribute"
                                ));
                            }
                        }
                        Meta::List(_) => {
                            return Err(syn::Error::new_spanned(
                                meta, "Nested lists not supported"
                            ));
                        }
                    }
                }
            }
            Meta::NameValue(_) => {
                return Err(syn::Error::new_spanned(
                    attr, "Expected #[config(...)] not #[config = ...]"
                ));
            }
        }
    }
    
    Ok(config)
}

/// Extract a string literal from an expression.
fn extract_string(expr: &Expr) -> Result<String, syn::Error> {
    if let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = expr {
        Ok(s.value())
    } else {
        Err(syn::Error::new_spanned(expr, "Expected string literal"))
    }
}

// Look at all that code! And this doesn't even include:
// - "Did you mean?" suggestions for typos
// - Accumulating multiple errors
// - Supporting all the attribute variations darling handles
//
// With darling, all of this becomes just:
//
// #[derive(FromField)]
// #[darling(attributes(config))]
// struct FieldConfig {
//     #[darling(default)]
//     env: Option<String>,
//     #[darling(default)]
//     default: Option<String>,
//     #[darling(default)]
//     skip: bool,
// }

