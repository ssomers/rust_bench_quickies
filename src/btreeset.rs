use std::collections::BTreeSet;

fn various_size_sets() -> Vec<BTreeSet<i32>> {
    (0..100)
        .map(|i| {
            let mut set = BTreeSet::<i32>::new();
            for j in 0..i {
                set.insert(i32::from(j));
            }
            set
        })
        .collect()
}

fn empty_or_singleton_sets() -> Vec<BTreeSet<i32>> {
    (0..100)
        .map(|i| {
            let mut set = BTreeSet::<i32>::new();
            if i % 2 == 0 {
                set.insert(i32::from(i));
            }
            set
        })
        .collect()
}

#[bench]
pub fn btreeset_range(b: &mut test::Bencher) {
    let sets = various_size_sets();
    b.iter(|| {
        for set in sets.iter() {
            let x = set.range(..);
            test::black_box(x);
        }
    })
}

#[bench]
pub fn btreeset_iter(b: &mut test::Bencher) {
    let sets = various_size_sets();
    b.iter(|| {
        for set in sets.iter() {
            let x = set.iter();
            test::black_box(x);
        }
    })
}

#[bench]
pub fn btreeset_binary_contains(b: &mut test::Bencher) {
    let sets = empty_or_singleton_sets();
    b.iter(|| {
        for set in sets.iter() {
            let x = set.contains(&42);
            test::black_box(x);
        }
    })
}

#[bench]
pub fn btreeset_general_contains(b: &mut test::Bencher) {
    let sets = various_size_sets();
    b.iter(|| {
        for set in sets.iter() {
            let x = set.contains(&42);
            test::black_box(x);
        }
    })
}

#[bench]
pub fn btreeset_binary_contains_if_not_empty(b: &mut test::Bencher) {
    let sets = empty_or_singleton_sets();
    b.iter(|| {
        for set in sets.iter() {
            let x = !set.is_empty() && set.contains(&42);
            test::black_box(x);
        }
    })
}

#[bench]
pub fn btreeset_binary_contains_iter(b: &mut test::Bencher) {
    let sets = empty_or_singleton_sets();
    b.iter(|| {
        for set in sets.iter() {
            let x = match set.len() {
                0 => false,
                1 => *set.iter().next().unwrap() == 42,
                _ => panic!(),
            };
            test::black_box(x);
        }
    })
}

#[bench]
pub fn btreeset_general_contains_if_not_empty(b: &mut test::Bencher) {
    let sets = various_size_sets();
    b.iter(|| {
        for set in sets.iter() {
            let x = !set.is_empty() && set.contains(&42);
            test::black_box(x);
        }
    })
}
