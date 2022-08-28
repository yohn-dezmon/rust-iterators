pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

fn main() {
    // Ex 1 (interate through an array)
    // ! indicates a macro 
    // vec! --> convenient initialization of a Vec, type inferred.
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for value in v1_iter {
        println!("Got: {value}");
    }

    // Ex 2 (get a single value from the iterator)
    // if you call next() you need the iterator to be mutable
    let mut v2_iter = v1.iter();
    let first_value = v2_iter.next();
    if let Some(x) = first_value {
        println!("first value: {x}");
    }

    // Ex 3 (assigning an index with zip() -- merging iterators)
    let v3_iter = v1.iter();
    v3_iter
    .zip(4..7)
    .for_each(|(v1_num, zip_num)| println!("v1_num: {}, zip_num: {}", v1_num, zip_num));

}

#[test]
fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        // Ex 4 Showing the results of the iterator with Some() and None 
        let mut v4_iter = v1.iter();
        // a reference to 1, inside of the Some variant
        assert_eq!(v4_iter.next(), Some(&1));
        assert_eq!(v4_iter.next(), Some(&2));
        assert_eq!(v4_iter.next(), Some(&3));
        // final iteration returns None
        assert_eq!(v4_iter.next(), None);
}