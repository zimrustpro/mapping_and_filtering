use core::num;

fn in_char_vec(char_vec: &Vec<char>, check: char) {
    println!(
        "Is {} inside? {}",
        check,
        char_vec.iter().any(|&char| char == check)
    );
}

fn main() {
    let char_vec = ('a'..'働').collect::<Vec<char>>();
    in_char_vec(&char_vec, 'i');
    in_char_vec(&char_vec, '뷁');
    in_char_vec(&char_vec, '鑿');

    let smaller_vec = ('A'..'z').collect::<Vec<char>>();
    println!(
        "All alphabetic? {}",
        smaller_vec.iter().all(|x| x.is_alphabetic())
    );
    println!(
        "All less than the character 행? {}",
        smaller_vec.iter().all(|&x| x < '행')
    );

    let mut big_vec = vec![6; 1000];
    big_vec.push(5);

    let mut iterator = big_vec.iter().rev();
    assert_eq!(iterator.next(), Some(&5));
    assert_eq!(iterator.next(), Some(&6));

    println!("{:?}", big_vec.iter().rev().any(|&number| number == 5));

    let mut big_vec = vec![6; 1000];
    big_vec.push(5);

    let mut num_loops = 0;
    let mut big_iter = big_vec.into_iter();
    loop {
        num_loops += 1;
        if big_iter.next() == Some(5) {
            break;
        }
    }
    println!("Number of loops: {}", num_loops);
}
