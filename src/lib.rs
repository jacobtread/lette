use std::collections::HashMap;

pub struct LetteDocument<'a> {
    /// Styling groups
    style_groups: HashMap<&'a str, StyleGroup<'a>>,
}

pub struct Style<'a> {
    key: &'a str,
    value: &'a str,
}

pub struct StyleGroup<'a> {
    styles: Vec<Style<'a>>,
}

pub enum ElementType {
    Preview,
    Container,
    Column,
    Section,
    Button,
    Heading,
    Text,
}

pub struct Element<'a> {
    ///
    ty: ElementType,
    /// Element arguments
    args: Args<'a>,
    /// Styles for the element
    styles: StyleGroup<'a>,
    /// Attributes on the element
    attributes: Attributes<'a>,
    /// Child elements
    children: Vec<Box<Element<'a>>>,
}

pub struct Args<'a> {
    values: Vec<&'a str>,
}

pub struct Attributes<'a> {
    values: HashMap<&'a str, &'a str>,
}
