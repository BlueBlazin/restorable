# Restorable: an iterator adapter for saving and restoring iterator state

This is ideal for writing lexers and parsers where backtracking may be required.

## Usage

1. Adapter

```rs
use restorable::RestorableIter;

#[test]
fn test_restorable_new() {
    let mut iterator = RestorableIter::new(0..4);
    assert_eq!(iterator.next(), Some(0));
    assert_eq!(iterator.next(), Some(1));
    iterator.save();
    assert_eq!(iterator.next(), Some(2));
    assert_eq!(iterator.next(), Some(3));
    iterator.restore();
    assert_eq!(iterator.next(), Some(2));
    assert_eq!(iterator.next(), Some(3));
    assert_eq!(iterator.next(), None);
}
```

2. Trait

```rs
use restorable::Restorable;

#[test]
fn test_iter_restorable() {
    let mut iterator = (0..4).iter_restorable();
    assert_eq!(iterator.next(), Some(0));
    assert_eq!(iterator.next(), Some(1));
    iterator.save();
    assert_eq!(iterator.next(), Some(2));
    assert_eq!(iterator.next(), Some(3));
    iterator.restore();
    assert_eq!(iterator.next(), Some(2));
    assert_eq!(iterator.next(), Some(3));
    assert_eq!(iterator.next(), None);
}
```

## Installation

Add `restorable` under `[dependecies]` with the appropriate version number in your cargo.toml.

```
[dependencies]
restorable = "^MAJOR.MINOR"
```

## Changelog

0.2.0 - Added `clear` to stop saving and clear the buffer.
