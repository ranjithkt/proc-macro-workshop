use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Group, Ident, Literal, TokenStream as TokenStream2, TokenTree};
use proc_macro_error2::proc_macro_error;
use syn::{parse::Parse, parse_macro_input, LitInt, Token};

struct SeqInput {
    var: Ident,
    start: u64,
    end: u64,
    inclusive: bool,
    body: TokenStream2,
}

impl Parse for SeqInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let var: Ident = input.parse()?;
        input.parse::<Token![in]>()?;
        let start: LitInt = input.parse()?;

        // Check for inclusive or exclusive range
        let inclusive = if input.peek(Token![..=]) {
            input.parse::<Token![..=]>()?;
            true
        } else {
            input.parse::<Token![..]>()?;
            false
        };

        let end: LitInt = input.parse()?;

        // Parse the body as a braced group
        let content;
        syn::braced!(content in input);
        let body: TokenStream2 = content.parse()?;

        Ok(SeqInput {
            var,
            start: start.base10_parse()?,
            end: end.base10_parse()?,
            inclusive,
            body,
        })
    }
}

#[proc_macro]
#[proc_macro_error]
pub fn seq(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SeqInput);

    let end = if input.inclusive {
        input.end + 1
    } else {
        input.end
    };

    // Check if the body contains #(...)* pattern
    let has_repeat = contains_repeat_section(&input.body);

    let mut output = TokenStream2::new();

    if has_repeat {
        // Only repeat the #(...)* sections
        output = expand_with_repeat(&input.body, &input.var, input.start, end);
    } else {
        // Repeat the entire body for each value
        for n in input.start..end {
            let expanded = replace_var(&input.body, &input.var, n);
            output.extend(expanded);
        }
    }

    output.into()
}

/// Check if the token stream contains a repeat section #(...)*
fn contains_repeat_section(tokens: &TokenStream2) -> bool {
    let mut iter = tokens.clone().into_iter().peekable();

    while let Some(tt) = iter.next() {
        match &tt {
            TokenTree::Punct(p) if p.as_char() == '#' => {
                if let Some(TokenTree::Group(g)) = iter.peek() {
                    if g.delimiter() == Delimiter::Parenthesis {
                        iter.next(); // consume the group
                        if let Some(TokenTree::Punct(star)) = iter.peek() {
                            if star.as_char() == '*' {
                                return true;
                            }
                        }
                    }
                }
            }
            TokenTree::Group(g) => {
                if contains_repeat_section(&g.stream()) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

/// Expand a token stream that may contain #(...)* sections
fn expand_with_repeat(tokens: &TokenStream2, var: &Ident, start: u64, end: u64) -> TokenStream2 {
    let mut output = Vec::new();
    let mut iter = tokens.clone().into_iter().peekable();

    while let Some(tt) = iter.next() {
        match &tt {
            TokenTree::Punct(p) if p.as_char() == '#' => {
                if let Some(TokenTree::Group(g)) = iter.peek().cloned() {
                    if g.delimiter() == Delimiter::Parenthesis {
                        iter.next(); // consume the group
                        if let Some(TokenTree::Punct(star)) = iter.peek() {
                            if star.as_char() == '*' {
                                iter.next(); // consume the *

                                // This is a repeat section, expand it
                                let repeat_body = g.stream();
                                for n in start..end {
                                    let expanded = replace_var(&repeat_body, var, n);
                                    output.extend(expanded);
                                }
                                continue;
                            }
                        }
                        // Not a repeat section, put back the tokens
                        output.push(tt);
                        output.push(TokenTree::Group(g));
                        continue;
                    }
                }
                output.push(tt);
            }
            TokenTree::Group(g) => {
                // Recurse into groups
                let inner = expand_with_repeat(&g.stream(), var, start, end);
                let mut new_group = Group::new(g.delimiter(), inner);
                new_group.set_span(g.span());
                output.push(TokenTree::Group(new_group));
            }
            _ => {
                output.push(tt);
            }
        }
    }

    output.into_iter().collect()
}

/// Replace variable occurrences with the given value
fn replace_var(tokens: &TokenStream2, var: &Ident, value: u64) -> TokenStream2 {
    let mut output = Vec::new();
    let mut iter = tokens.clone().into_iter().peekable();

    while let Some(tt) = iter.next() {
        match &tt {
            TokenTree::Ident(ident) => {
                // Check for identifier pasting: ident~VAR
                if let Some(TokenTree::Punct(p)) = iter.peek() {
                    if p.as_char() == '~' {
                        // Look ahead to see if next is our variable
                        let mut lookahead = iter.clone();
                        lookahead.next(); // consume ~
                        if let Some(TokenTree::Ident(next_ident)) = lookahead.next() {
                            if next_ident == *var {
                                // This is ident~VAR, paste them together
                                iter.next(); // consume ~
                                iter.next(); // consume VAR
                                let new_name = format!("{}{}", ident, value);
                                let new_ident = Ident::new(&new_name, ident.span());
                                output.push(TokenTree::Ident(new_ident));
                                continue;
                            }
                        }
                    }
                }

                // Check if this is just the variable itself
                if ident == var {
                    let lit = Literal::u64_unsuffixed(value);
                    output.push(TokenTree::Literal(lit));
                } else {
                    output.push(tt);
                }
            }
            TokenTree::Group(g) => {
                // Recurse into groups
                let inner = replace_var(&g.stream(), var, value);
                let mut new_group = Group::new(g.delimiter(), inner);
                new_group.set_span(g.span());
                output.push(TokenTree::Group(new_group));
            }
            _ => {
                output.push(tt);
            }
        }
    }

    output.into_iter().collect()
}
