extern crate serde;
extern crate serde_json;

#[derive(Debug, PartialEq, Clone)]
pub enum Foo {
    Bar,
    Baz,
}

impl serde::de::Deserialize for Foo {
    fn deserialize<D>(deserializer: &mut D) -> Result<Foo, D::Error>
            where D: serde::de::Deserializer
    {
        deserializer.visit(FooVisitor)
    }
}

struct FooVisitor;

impl serde::de::Visitor for FooVisitor {
    type Value = Foo;

    fn visit_string<E>(&mut self, value: String) -> Result<Foo, E>
            where E: serde::de::Error
    {
        match &value[..] {
            "bar" => Ok(Foo::Bar),
            "baz" => Ok(Foo::Baz),
            a => Err(serde::de::Error::syntax(&format!("Unexpected value {}", a))),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Quux {
    foo: String,
    name: String,
}

enum QuuxField {
    Foo,
    Name,
}

impl serde::de::Deserialize for QuuxField {
    fn deserialize<D>(deserializer: &mut D) -> Result<QuuxField, D::Error>
            where D: serde::de::Deserializer
    {
        struct QuuxFieldVisitor;

        impl serde::de::Visitor for QuuxFieldVisitor {
            type Value = QuuxField;
            fn visit_str<E: serde::de::Error>(&mut self, value: &str) -> Result<QuuxField, E> {
                match value {
                    "foo" => Ok(QuuxField::Foo),
                    "name" => Ok(QuuxField::Name),
                    _ => Err(serde::de::Error::syntax("expected 'foo' or 'name'")),
                }
            }
        }

        deserializer.visit(QuuxFieldVisitor)
    }
}

impl serde::de::Deserialize for Quux {
    fn deserialize<D>(deserializer: &mut D) -> Result<Quux, D::Error>
            where D: serde::de::Deserializer
    {
        static FIELDS: &'static [&'static str] = &["foo", "name"];
        deserializer.visit_struct("Quux", FIELDS, QuuxVisitor)
    }
}

struct QuuxVisitor;

impl serde::de::Visitor for QuuxVisitor {
    type Value = Quux;

    fn visit_map<V>(&mut self, mut visitor: V) -> Result<Quux, V::Error>
            where V: serde::de::MapVisitor
    {
        let mut foo = None;
        let mut name = None;

        loop {
            match try!(visitor.visit_key()) {
                Some(QuuxField::Foo) => { foo = Some(try!(visitor.visit_value())); }
                Some(QuuxField::Name) => { name = Some(try!(visitor.visit_value())); }
                None => { break; }
            }
        }

        let foo = match foo {
            Some(foo) => foo,
            None => try!(visitor.missing_field("foo")),
        };
        let name = match name {
            Some(name) => name,
            None => try!(visitor.missing_field("name")),
        };

        try!(visitor.end());

        Ok(Quux {
            foo: foo,
            name: name,
        })
    }
}

#[test]
fn test_deser() {
    static SOURCE: &'static str = r#"{
        "foo": "bar",
        "name": "some name"
}"#;

    let q = serde_json::from_str::<Quux>(SOURCE).unwrap();
    assert_eq!(q, Quux { foo: "bar".to_string(), name: "some name".to_string() });
}
