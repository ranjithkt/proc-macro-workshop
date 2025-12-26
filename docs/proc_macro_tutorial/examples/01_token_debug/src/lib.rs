//! Token debugging macros for the proc-macro tutorial.
//!
//! This crate demonstrates how to inspect TokenStreams and understand
//! what procedural macros actually receive as input.

use proc_macro::TokenStream;
use proc_macro2::TokenTree;

/// A simple macro that prints the token stream it receives.
///
/// # Example
///
/// ```ignore
/// debug_tokens!(struct Foo { x: i32 });
/// ```
///
/// This will print the token structure during compilation.
#[proc_macro]
pub fn debug_tokens(input: TokenStream) -> TokenStream {
    eprintln!("╔══════════════════════════════════════════╗");
    eprintln!("║         TOKEN STREAM DEBUG               ║");
    eprintln!("╠══════════════════════════════════════════╣");
    eprintln!("{:#?}", input);
    eprintln!("╚══════════════════════════════════════════╝");
    input
}

/// A more detailed token inspector that identifies each token type.
///
/// This macro iterates through tokens and prints what type each one is,
/// helping you understand the structure of Rust code as tokens.
///
/// # Example
///
/// ```ignore
/// inspect_tokens!(fn add(a: u32) -> u32 { a + 1 });
/// ```
#[proc_macro]
pub fn inspect_tokens(input: TokenStream) -> TokenStream {
    let input2: proc_macro2::TokenStream = input.into();

    eprintln!("┌──────────────────────────────────────────┐");
    eprintln!("│          TOKEN INSPECTOR                 │");
    eprintln!("├──────────────────────────────────────────┤");

    fn inspect_recursive(stream: proc_macro2::TokenStream, depth: usize) {
        let indent = "  ".repeat(depth);
        for (i, tree) in stream.into_iter().enumerate() {
            match &tree {
                TokenTree::Ident(ident) => {
                    eprintln!("{}[{}] Ident: `{}`", indent, i, ident);
                }
                TokenTree::Punct(punct) => {
                    eprintln!(
                        "{}[{}] Punct: '{}' (spacing: {:?})",
                        indent,
                        i,
                        punct.as_char(),
                        punct.spacing()
                    );
                }
                TokenTree::Group(group) => {
                    eprintln!("{}[{}] Group ({:?}) {{", indent, i, group.delimiter());
                    inspect_recursive(group.stream(), depth + 1);
                    eprintln!("{}}}", indent);
                }
                TokenTree::Literal(lit) => {
                    eprintln!("{}[{}] Literal: {}", indent, i, lit);
                }
            }
        }
    }

    inspect_recursive(input2.clone(), 0);
    eprintln!("└──────────────────────────────────────────┘");

    input2.into()
}

/// Counts and summarizes the tokens in the input.
///
/// Provides a quick overview of how many of each token type are present.
#[proc_macro]
pub fn count_tokens(input: TokenStream) -> TokenStream {
    let input2: proc_macro2::TokenStream = input.into();

    let mut idents = 0;
    let mut puncts = 0;
    let mut groups = 0;
    let mut literals = 0;

    fn count_recursive(
        stream: proc_macro2::TokenStream,
        idents: &mut usize,
        puncts: &mut usize,
        groups: &mut usize,
        literals: &mut usize,
    ) {
        for tree in stream {
            match tree {
                TokenTree::Ident(_) => *idents += 1,
                TokenTree::Punct(_) => *puncts += 1,
                TokenTree::Group(g) => {
                    *groups += 1;
                    count_recursive(g.stream(), idents, puncts, groups, literals);
                }
                TokenTree::Literal(_) => *literals += 1,
            }
        }
    }

    count_recursive(
        input2.clone(),
        &mut idents,
        &mut puncts,
        &mut groups,
        &mut literals,
    );

    let total = idents + puncts + groups + literals;
    eprintln!("┌─────────────────────────────┐");
    eprintln!("│     TOKEN COUNT SUMMARY     │");
    eprintln!("├─────────────────────────────┤");
    eprintln!("│ Idents:   {:>5}            │", idents);
    eprintln!("│ Puncts:   {:>5}            │", puncts);
    eprintln!("│ Groups:   {:>5}            │", groups);
    eprintln!("│ Literals: {:>5}            │", literals);
    eprintln!("├─────────────────────────────┤");
    eprintln!("│ TOTAL:    {:>5}            │", total);
    eprintln!("└─────────────────────────────┘");

    input2.into()
}
