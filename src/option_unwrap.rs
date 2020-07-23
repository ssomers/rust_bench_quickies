use std::hint::unreachable_unchecked;

static IT: char = 'a';

fn options() -> [Option<&'static char>; 200] {
    [Some(&IT); 200]
}

#[bench]
pub fn unwrap(b: &mut test::Bencher) {
    let options = options();
    b.iter(|| {
        for option in options.iter() {
            test::black_box(option.unwrap());
        }
    })
}

#[bench]
pub fn unwrap_or_else_unreachable(b: &mut test::Bencher) {
    let options = options();
    b.iter(|| {
        for option in options.iter() {
            test::black_box(option.unwrap_or_else(|| unreachable!()));
        }
    })
}

#[bench]
pub fn unwrap_or_else_unreachable_unchecked(b: &mut test::Bencher) {
    let options = options();
    b.iter(|| {
        for option in options.iter() {
            test::black_box(option.unwrap_or_else(|| unsafe { unreachable_unchecked() }));
        }
    })
}
