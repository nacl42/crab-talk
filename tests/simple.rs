use crab_talk::*;

pub enum Count {
    One,
    Two,
    Three,
}

impl<'a> Translate<'a> for Count {
    fn translate(&self, lang: Lang) -> std::borrow::Cow<'a, str> {
        match self {
            Count::One => match lang {
                Lang::De => "eins",
                Lang::Sv => "ett",
                Lang::En => "one",
            }
            .into(),
            Count::Two => match lang {
                Lang::De => "zwei",
                Lang::Sv => "två",
                Lang::En => "two",
            }
            .into(),
            Count::Three => match lang {
                Lang::De => "drei",
                Lang::Sv => "tre",
                Lang::En => "three",
            }
            .into(),
        }
    }
}

#[test]
fn test_simple() {
    let c1 = Count::One;
    assert_eq!(c1.translate(Lang::De).as_ref(), "eins");
    assert_eq!(c1.translate(Lang::En).as_ref(), "one");

    let c3 = Count::Three;
    assert_eq!(c3.translate(Lang::De).as_ref(), "drei");
    assert_eq!(c3.translate(Lang::En).as_ref(), "three");
}
