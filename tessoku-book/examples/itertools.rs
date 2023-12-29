use itertools::Itertools;

fn main() {
    let a = vec![1, 2, 3, 4];

    let mut double = a.iter().tuple_combinations::<(_, _)>();
    assert_eq!(double.next(), Some((&1, &2)));
    assert_eq!(double.next(), Some((&1, &3)));
    assert_eq!(double.next(), Some((&1, &4)));
    assert_eq!(double.next(), Some((&2, &3)));
    assert_eq!(double.next(), Some((&2, &4)));
    assert_eq!(double.next(), Some((&3, &4)));
    assert_eq!(double.next(), None);

    let mut triple = a.iter().tuple_combinations::<(_, _, _)>();
    assert_eq!(triple.next(), Some((&1, &2, &3)));
    assert_eq!(triple.next(), Some((&1, &2, &4)));
    assert_eq!(triple.next(), Some((&1, &3, &4)));
    assert_eq!(triple.next(), Some((&2, &3, &4)));
    assert_eq!(triple.next(), None);
}
