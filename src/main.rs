fn main() {
    let even_odd_iter = ["even", "odd"].into_iter().cycle();
    let even_odd_vec = (0..=5).zip(even_odd_iter).collect::<Vec<(i32, &str)>>();
    println!("{:?}", even_odd_vec);

    let ten_chars = ('a'..).take(10).collect::<Vec<char>>();
    let skip_then_ten_chars = ('a'..).skip(1_300).take(10).collect::<Vec<char>>();
    println!("{:?}", ten_chars);
    println!("{:?}", skip_then_ten_chars);
}
