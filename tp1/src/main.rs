
fn mad(a: i32, b: i32, c: i32) -> i32 {
    return a * b + c;
}

fn sum_classic(a: i32, b: i32) -> i32 {
    return (b - a) * ((b - a) - 1) / 2;
}

fn sum_while(mut a: i32, b: i32) -> i32 {
    let mut res: i32 = 0;

    while a <= b {
        res += a;
        a += 1;
    }

    return res;
}

fn sum_for(a: i32, b: i32) -> i32 {
    let mut res: i32 = 0;

    for it in a..b {
        res += it;
    }

    return res;
}

struct Livre {
    titre: String,
    année: u32,
    genre: Genre,
}

fn age_livre(l: Livre) -> u32 {
    2022 - l.année
}

enum Genre {
    Fiction,
    Histoire,
    Fantasy,
    Informatique,
}

fn score(l: Livre) -> u32 {
    let genre_score = match l.genre {
        Genre::Fiction => 12,
        Genre::Histoire => 15,
        Genre::Fantasy => 42,
        Genre::Informatique => 55,
    };

    let len: u32 = l.titre.len() as u32;
    (len + l.année) * genre_score
}

enum DivisionResult {
    DivisionByZero,
    DivisionOk(i32),
}

fn division(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    print!("Mad\n");
    print!("{}\n", mad(1, 2, 3));
    print!("Sum\n");
    print!("]1;5[ {}\n", sum_classic(1, 5));

    print!("[1;5] {}\n", sum_while(1, 5));
    print!("[1-5] {}\n", sum_for(1, 5));

    print!("[1, n] \n");

    print!("[1-5] {}\n", sum_while(1, 5));
    print!("[1-5] {}\n", sum_for(1, 5));

    let a = Livre {
        titre: "Les poulets".to_string(),
        année: 2021,
        genre: Genre::Fiction,
    };
    print!("{}", a.titre)
}
