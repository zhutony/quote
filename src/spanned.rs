use crate::ToTokens;
use proc_macro2::{Span, TokenStream};

pub trait Spanned {
    fn span(&self) -> Span;
}

impl Spanned for Span {
    fn span(&self) -> Span {
        *self
    }
}

impl<T: ToTokens> Spanned for T {
    fn span(&self) -> Span {
        join_spans(self.into_token_stream())
    }
}

fn join_spans(tokens: TokenStream) -> Span {
    let mut iter = tokens.into_iter().filter_map(|tt| {
        // FIXME: This shouldn't be required, since optimally spans should
        // never be invalid. This filter_map can probably be removed when
        // https://github.com/rust-lang/rust/issues/43081 is resolved.
        let span = tt.span();
        let debug = format!("{:?}", span);
        if debug.ends_with("bytes(0..0)") {
            None
        } else {
            Some(span)
        }
    });

    let first = match iter.next() {
        Some(span) => span,
        None => return Span::call_site(),
    };

    #[cfg(procmacro2_semver_exempt)]
    return iter
        .fold(None, |_prev, next| Some(next))
        .and_then(|last| first.join(last))
        .unwrap_or(first);

    // We can't join spans without procmacro2_semver_exempt so just grab the
    // first one.
    #[cfg(not(procmacro2_semver_exempt))]
    return first;
}
