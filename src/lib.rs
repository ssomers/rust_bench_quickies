#![feature(test)]
extern crate test;

static IT: char = 'a';

fn options() -> [Option<&'static char>; 200] {
    let mut result = [None; 200];
    for i in 0..100 {
        result[i * 2] = Some(&IT);
    }
    result
}

#[bench]
pub fn iter_count(b: &mut test::Bencher) {
    let options = options();
    b.iter(|| {
        for option in options.iter() {
            let x: usize = option.iter().count();
            test::black_box(x);
        }
    })
}

#[bench]
pub fn is_some_as(b: &mut test::Bencher) {
    let options = options();
    b.iter(|| {
        for option in options.iter() {
            let x = option.is_some() as usize;
            test::black_box(x);
        }
    })
}

#[bench]
pub fn is_some_into(b: &mut test::Bencher) {
    let options = options();
    b.iter(|| {
        for option in options.iter() {
            let x: usize = option.is_some().into();
            test::black_box(x);
        }
    })
}

#[bench]
pub fn is_some_if(b: &mut test::Bencher) {
    let options = options();
    b.iter(|| {
        for option in options.iter() {
            let x: usize = if option.is_some() { 1 } else { 0 };
            test::black_box(x);
        }
    })
}
