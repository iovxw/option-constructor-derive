#![allow(dead_code)]

#[macro_use]
extern crate option_constructor_derive;

#[test]
fn empty_struct() {
    #[derive(OptionConstructor, Debug, PartialEq)]
    struct Test;

    let x = Test::new();
    assert_eq!(x, Test);
}

#[test]
fn base() {
    #[derive(OptionConstructor, Debug, PartialEq)]
    struct Test {
        field1: bool,
        field2: Option<i32>,
        field3: Option<bool>,
    }

    let x = Test::new(true).field2(1);
    assert_eq!(x,
               Test {
                   field1: true,
                   field2: Some(1),
                   field3: None,
               });
}

#[test]
fn tuple() {
    #[derive(OptionConstructor, Debug, PartialEq)]
    struct Test {
        field1: (i32,),
        field2: Option<(i32, bool)>,
        field3: Option<()>,
    }

    let x = Test::new((1,)).field2((2, false));
    assert_eq!(x,
               Test {
                   field1: (1,),
                   field2: Some((2, false)),
                   field3: None,
               });
}

#[test]
fn static_ptr() {
    #[derive(OptionConstructor, Debug, PartialEq)]
    struct Test {
        field1: &'static str,
        field2: Option<&'static str>,
        field3: Option<&'static str>,
    }

    let x = Test::new("x").field2("abc");
    assert_eq!(x,
               Test {
                   field1: "x",
                   field2: Some("abc"),
                   field3: None,
               });
}

#[test]
fn lifetime() {
    #[derive(OptionConstructor, Debug, PartialEq)]
    struct Test<'a, 'b> {
        field1: &'a str,
        field2: Option<&'b str>,
        field3: Option<&'a str>,
    }

    let x = Test::new("x").field2("abc");
    assert_eq!(x,
               Test {
                   field1: "x",
                   field2: Some("abc"),
                   field3: None,
               });
}

#[test]
fn generic() {
    #[derive(OptionConstructor, Debug, PartialEq)]
    struct Test<T, B> {
        field1: T,
        field2: Option<B>,
        field3: Option<T>,
    }

    let x = Test::new(true).field2(1);
    assert_eq!(x,
               Test {
                   field1: true,
                   field2: Some(1),
                   field3: None,
               });
}

#[test]
fn trait_generic() {
    #[derive(OptionConstructor, Debug, PartialEq)]
    struct Test<T: Clone, B: Eq> {
        field1: T,
        field2: Option<B>,
        field3: Option<T>,
    }

    let x = Test::new(true).field2(1);
    assert_eq!(x,
               Test {
                   field1: true,
                   field2: Some(1),
                   field3: None,
               });
}

#[test]
fn trait_generic_where() {
    #[derive(OptionConstructor, Debug, PartialEq)]
    struct Test<T, B>
        where T: Clone,
              B: Eq
    {
        field1: T,
        field2: Option<B>,
        field3: Option<T>,
    }

    let x = Test::new(true).field2(1);
    assert_eq!(x,
               Test {
                   field1: true,
                   field2: Some(1),
                   field3: None,
               });
}
