use std::collections::BTreeMap;
use test::Bencher;

fn bigmap() -> BTreeMap<i32, i32> {
    (0..100).map(|i| (i, i)).collect()
}

#[bench]
fn map_first(b: &mut Bencher) {
    let m = bigmap();
    b.iter(|| m.first_key_value());
}

#[bench]
fn map_last(b: &mut Bencher) {
    let m = bigmap();
    b.iter(|| m.last_key_value());
}

#[bench]
fn map_iter_min(b: &mut Bencher) {
    let m = bigmap();
    b.iter(|| m.iter().min());
}

#[bench]
fn map_iter_max(b: &mut Bencher) {
    let m = bigmap();
    b.iter(|| m.iter().max());
}

#[bench]
fn map_iter_first(b: &mut Bencher) {
    let m = bigmap();
    b.iter(|| m.iter().next());
}

#[bench]
fn map_iter_last(b: &mut Bencher) {
    let m = bigmap();
    b.iter(|| m.iter().last());
}
