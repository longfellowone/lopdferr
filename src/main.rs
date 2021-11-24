#![feature(option_result_contains)]

use lopdf::{Document, Object, StringFormat};

fn main() {
    let mut doc = Document::load("sample.pdf").unwrap();

    doc.objects.iter_mut().for_each(|(_, o)| {
        if let Ok(name) = o.type_name() {
            match name {
                "Annot" => process_object(o),
                _ => {}
            }
        }
    });

    doc.save("sample2.pdf").unwrap();
}

fn process_object(o: &mut Object) {
    println!("{:?}", o);

    let dict = o.as_dict_mut().unwrap();

    let string = r#"my new string"#.to_owned().into_bytes();

    dict.set("RC", Object::string_literal(string.clone()));
    dict.set("Contents", Object::string_literal(string));

    println!("{:?}", o);
}
