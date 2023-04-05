use chumsky::prelude::*;

type E<'a> = extra::Err<Rich<'a, char, SimpleSpan<usize>>>;

fn text<'a>() -> impl Parser<'a, &'a str, String, E<'a>> {
    any()
        .and_is(one_of("=>").not())
        .repeated()
        .at_least(1)
        .map_slice(|s: &str| s.to_string())
}

#[derive(Clone, Debug)]
pub struct CategoryEdit {
    target: String,
    elements: String,
}

pub fn cat_edit<'a>() -> impl Parser<'a, &'a str, CategoryEdit, E<'a>> {
    text()
        .then_ignore(just('='))
        .then(text())
        .map(|(target, elements)| CategoryEdit { target, elements })
}

#[derive(Debug, Clone, Default)]
pub struct Rule {
    target: String,
    predicates: Vec<String>,
}

fn rule<'src>() -> impl Parser<'src, &'src str, Rule, E<'src>> {
    text()
        .then(
            just('>')
                .ignore_then(text())
                .repeated()
                //.at_least(1)
                .collect::<Vec<_>>(),
        )
        .map(|(target, predicates)| Rule { target, predicates })
}

#[derive(Debug, Clone)]
pub enum ASTElement {
    Rule(Rule),
    CatEdit(CategoryEdit),
}

pub fn ast_element<'src>() -> impl Parser<'src, &'src str, ASTElement, E<'src>> {
    rule()
        .map(ASTElement::Rule)
        .or(cat_edit().map(ASTElement::CatEdit))
}

fn main() {
    let (ast, errs) = ast_element().parse("A=b").into_output_errors();
    if let Some(ast) = ast {
        println!("ast: {ast:?}")
    }
    println!("errs: {errs:?}");
}
