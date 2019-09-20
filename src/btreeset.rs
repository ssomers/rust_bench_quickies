use std::collections::BTreeSet;

fn sets() -> Vec<BTreeSet<i32>> {
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

#[bench]
pub fn btreeset_range(b: &mut test::Bencher) {
    let sets = sets();
    b.iter(|| {
        for set in sets.iter() {
            let x = set.range(..);
            test::black_box(x);
        }
    })
}

#[bench]
pub fn btreeset_iter(b: &mut test::Bencher) {
    let sets = sets();
    b.iter(|| {
        for set in sets.iter() {
            let x = set.iter();
            test::black_box(x);
        }
    })
}
