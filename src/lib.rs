//! Yet another rust translation system

use std::borrow::Cow;

#[derive(Default, Clone)]
pub enum Lang {
    #[default]
    En,
    De,
    Sv,
}

pub trait Translate<'a> {
    fn translate(&self, lang: Lang) -> Cow<'a, str>;
}
