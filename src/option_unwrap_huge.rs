use std::hint::unreachable_unchecked;

#[derive(Copy, Clone)]
struct Big([char; 500]);
static IT: Big = Big(['a'; 500]);

fn options() -> [Option<Big>; 100] {
    [Some(IT); 100]
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
