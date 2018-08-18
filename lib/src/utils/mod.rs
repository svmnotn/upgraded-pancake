mod constants;
pub use self::constants::*;

crate fn gen_strings(columns: usize, heading: bool) -> crate::Strings {
    use lipsum::lipsum_title;

    if columns > 1 {
        let mut vec: Vec<String> = Vec::with_capacity(columns);
        for _ in 0..columns {
            vec.push(if heading {
                lipsum_title()
            } else {
                lipsum_title() + " " + &lipsum_title() + " " + &lipsum_title()
            });
        }
        vec.into()
    } else if heading {
        lipsum_title().into()
    } else {
        (lipsum_title() + " " + &lipsum_title() + " " + &lipsum_title()).into()
    }
}
