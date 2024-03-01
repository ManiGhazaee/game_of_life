use std::collections::HashMap;

use once_cell::sync::Lazy;

pub const CHARS_WIDTH: usize = 6;
pub const CHARS_HEIGHT: usize = 5;
pub const CHARS_SPACE: usize = 1;

pub static CHAR_MAP: Lazy<HashMap<char, [u8; 30]>> = Lazy::new(|| {
    HashMap::from([
        ('S', CHAR_S),
        ('C', CHAR_C),
        ('O', CHAR_O),
        ('R', CHAR_R),
        ('E', CHAR_E),
        ('T', CHAR_T),
        ('B', CHAR_B),
        ('0', CHAR_0),
        ('1', CHAR_1),
        ('2', CHAR_2),
        ('3', CHAR_3),
        ('4', CHAR_4),
        ('5', CHAR_5),
        ('6', CHAR_6),
        ('7', CHAR_7),
        ('8', CHAR_8),
        ('9', CHAR_9),
        (' ', CHAR_SPACE),
        (':', CHAR_COLON),
    ])
});

pub const CHAR_S: [u8; 30] = format_font(
    b"\
.#####\
#.....\
.####.\
.....#\
#####.\
",
);

pub const CHAR_C: [u8; 30] = format_font(
    b"\
.#####\
#.....\
#.....\
#.....\
.#####\
",
);

pub const CHAR_O: [u8; 30] = format_font(
    b"\
.####.\
#....#\
#....#\
#....#\
.####.\
",
);

pub const CHAR_R: [u8; 30] = format_font(
    b"\
#####.\
#....#\
#####.\
#...#.\
#....#\
",
);

pub const CHAR_E: [u8; 30] = format_font(
    b"\
.#####\
#.....\
#####.\
#.....\
.#####\
",
);

pub const CHAR_T: [u8; 30] = format_font(
    b"\
.#####\
...#..\
...#..\
...#..\
...#..\
",
);

pub const CHAR_B: [u8; 30] = format_font(
    b"\
#####.\
#....#\
#####.\
#....#\
#####.\
",
);

pub const CHAR_SPACE: [u8; 30] = format_font(
    b"\
......\
......\
......\
......\
......\
",
);

pub const CHAR_0: [u8; 30] = format_font(
    b"\
.####.\
#...##\
#.##.#\
##...#\
.####.\
",
);

pub const CHAR_1: [u8; 30] = format_font(
    b"\
..##..\
.#.#..\
...#..\
...#..\
######\
",
);

pub const CHAR_2: [u8; 30] = format_font(
    b"\
#####.\
.....#\
.####.\
#.....\
######\
",
);

pub const CHAR_3: [u8; 30] = format_font(
    b"\
#####.\
.....#\
.####.\
.....#\
#####.\
",
);

pub const CHAR_4: [u8; 30] = format_font(
    b"\
#....#\
#....#\
.####.\
.....#\
.....#\
",
);

pub const CHAR_5: [u8; 30] = format_font(
    b"\
######\
#.....\
#####.\
.....#\
#####.\
",
);

pub const CHAR_6: [u8; 30] = format_font(
    b"\
.#####\
#.....\
#####.\
#....#\
.####.\
",
);

pub const CHAR_7: [u8; 30] = format_font(
    b"\
######\
....#.\
...#..\
..#...\
..#...\
",
);

pub const CHAR_8: [u8; 30] = format_font(
    b"\
.####.\
#....#\
.####.\
#....#\
.####.\
",
);

pub const CHAR_9: [u8; 30] = format_font(
    b"\
.####.\
#....#\
.#####\
.....#\
#####.\
",
);

pub const CHAR_COLON: [u8; 30] = format_font(
    b"\
......\
..#...\
......\
..#...\
......\
",
);

const fn format_font<const L: usize>(char: &'static [u8; L]) -> [u8; L] {
    let mut i = 0;
    let mut arr = [0; L];
    while i < char.len() {
        if char[i] == 46 {
            arr[i] = 0;
        } else {
            arr[i] = 1;
        }
        i += 1;
    }
    return arr;
}
