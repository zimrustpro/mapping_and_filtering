fn main() {
    let even_odd_iter = ["even", "odd"].into_iter().cycle();
    let even_odd_vec = (0..=5).zip(even_odd_iter).collect::<Vec<(i32, &str)>>();
    println!("{:?}", even_odd_vec);
}
