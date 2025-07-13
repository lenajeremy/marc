use crate::inline_container::InlineContainer;

use super::{Block, Node};

pub struct UnorderedList {
    items: Vec<Box<dyn Node>>,
}

impl UnorderedList {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn add_list_item(&mut self, item: Box<dyn Node>) {
        self.items.push(item);
    }
}

impl Node for UnorderedList {
    fn token_literal(&self) -> String {
        format!("UnorderedList(content={})", self.items.token_literal())
    }

    fn translate(&self) -> String {
        format!("<ul>{}</ul>", self.items.translate())
    }

    fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}

impl Block for UnorderedList {
    fn block_token(&self) {}
}

pub struct OrderedList {
    items: Vec<Box<dyn Node>>,
}

impl OrderedList {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn add_list_item(&mut self, item: Box<dyn Node>) {
        self.items.push(item);
    }
}

impl Node for OrderedList {
    fn token_literal(&self) -> String {
        format!("OrderedList(content={})", self.items.token_literal())
    }

    fn translate(&self) -> String {
        format!("<ol>{}</ol>", self.items.translate())
    }

    fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}

impl Block for OrderedList {
    fn block_token(&self) {}
}

pub struct ListItem {
    inner: Box<dyn Node>,
}

impl ListItem {
    pub fn new() -> Self {
        Self {
            inner: Box::new(InlineContainer::new()),
        }
    }

    pub fn set_inner(&mut self, inner: Box<dyn Node>) {
        self.inner = inner;
    }
}

impl Node for ListItem {
    fn token_literal(&self) -> String {
        format!("ListItem(content={})", self.inner.token_literal())
    }

    fn translate(&self) -> String {
        format!("<li>{}</li>", self.inner.translate())
    }

    fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}

impl Block for ListItem {
    fn block_token(&self) {}
}
