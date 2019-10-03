extern crate restorable;
use restorable::Restorable;

#[derive(Clone, Debug, PartialEq)]
struct Element {
    val: String
}

#[test]
fn test_iter_restorable_for_clone() {
    let items = vec![String::from("ham"),
                     String::from("eggs"),
                     String::from("bacon"), 
                     String::from("tuna")];
    let mut iterator = items.into_iter().iter_restorable();
    assert_eq!(iterator.next(), Some(String::from("ham")));
    assert_eq!(iterator.next(), Some(String::from("eggs")));
    iterator.save();
    assert_eq!(iterator.next(), Some(String::from("bacon")));
    assert_eq!(iterator.next(), Some(String::from("tuna")));
    iterator.restore();
    assert_eq!(iterator.next(), Some(String::from("bacon")));
    assert_eq!(iterator.next(), Some(String::from("tuna")));
    assert_eq!(iterator.next(), None);
}

#[test]
fn test_iter_restorable_derive_clone() {
    let items = vec![Element { val: String::from("earth") },
                     Element { val: String::from("storm") },
                     Element { val: String::from("fire") }];
    let mut iterator = items.into_iter().iter_restorable();
    assert_eq!(iterator.next(), Some(Element {
        val: String::from("earth")
    }));
    assert_eq!(iterator.next(), Some(Element {
        val: String::from("storm")
    }));
    iterator.save();
    assert_eq!(iterator.next(), Some(Element {
        val: String::from("fire")
    }));
    iterator.restore();
    assert_eq!(iterator.next(), Some(Element {
        val: String::from("fire")
    }));
    assert_eq!(iterator.next(), None);
}