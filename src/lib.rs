extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro::TokenTree;
use proc_macro::Group;
use proc_macro::Ident;
use proc_macro::Span;
use proc_macro::Spacing;
use proc_macro::Punct;

use proc_macro::Delimiter;

struct TakeUntil<I, P>
    where I: Iterator,
          P: FnMut(&I::Item) -> bool
{
    inner: I,
    predicate: P,
    found: bool
}

impl<I, P> Iterator for TakeUntil<I, P>
    where I: Iterator,
          P: FnMut(&I::Item) -> bool {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.found { // a previous iteration found the searched element
            return None;
        }
        if let Some(item) = self.inner.next() {
            if (self.predicate)(&item) {
                self.found = true;
            }
            Some(item)
        } else {
            None
        }
    }
}
trait IteratorExt : Iterator  {
    fn take_until<P>(self, predicate: P) -> TakeUntil<Self, P>
    where
        P: FnMut(&Self::Item) -> bool,
        Self: Sized;
}

impl<T> IteratorExt for T
    where T: Iterator {

    fn take_until<P>(self, predicate: P) -> TakeUntil<Self, P>
    where
        P: FnMut(&Self::Item) -> bool,
        Self: Sized
    {
        TakeUntil { inner: self, predicate: predicate, found: false }
    }
}

#[proc_macro_attribute]
pub fn speed_limit(limit_expression: TokenStream, input: TokenStream) -> TokenStream {
    let mut output_stream = TokenStream::new();

    let mut input_iter = input.into_iter();
    
    let seek_parameters = (&mut input_iter)
                               .take_until(|tt|{
                                   function_parameters(tt)
                               });
    output_stream.extend(seek_parameters);

    let function_body = if let Some(TokenTree::Group(old_function_body)) = input_iter.next() {
        let mut assertion_trees = Vec::new();
        assertion_trees.push(TokenTree::Ident(Ident::new("debug_assert", Span::call_site())));
        assertion_trees.push(TokenTree::Punct(Punct::new('!', Spacing::Joint)));
        assertion_trees.push(TokenTree::Group(Group::new(Delimiter::Brace, limit_expression)));
        assertion_trees.push(TokenTree::Punct(Punct::new(';', Spacing::Alone)));

        let mut new_function_body = TokenStream::new();
        new_function_body.extend(assertion_trees.into_iter());
        new_function_body.extend(old_function_body.stream());

        TokenTree::Group(Group::new(Delimiter::Brace, new_function_body))
    } else {
        panic!("expected function to have a body");
    };

    output_stream.extend(std::iter::once(function_body));
    println!("{:?}", output_stream.to_string());

    output_stream
}

fn function_parameters(tt: &TokenTree) -> bool {
    println!("{:?}", tt);
    match tt {
        TokenTree::Group(g) => {
            g.delimiter() == Delimiter::Parenthesis
        }
        _ => false,
    }
}
