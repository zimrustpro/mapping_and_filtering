#[derive(Debug)]
struct CombinedEvents {
    num_of_events: u32,
    data: Vec<String>,
}

fn main() {
    let even_odd_iter = ["even", "odd"].into_iter().cycle();
    let even_odd_vec = (0..=5).zip(even_odd_iter).collect::<Vec<(i32, &str)>>();
    println!("{:?}", even_odd_vec);

    let ten_chars = ('a'..).take(10).collect::<Vec<char>>();
    let skip_then_ten_chars = ('a'..).skip(1_300).take(10).collect::<Vec<char>>();
    println!("{:?}", ten_chars);
    println!("{:?}", skip_then_ten_chars);

    let some_numbers = vec![9, 6, 9, 10, 11];
    println!(
        "{}",
        some_numbers
            .iter()
            .fold(0, |total_so_far, next_number| total_so_far + next_number)
    );

    let events = [
        "Went to grocery store",
        "Came home",
        "Fed cat",
        "Fed cat again",
    ];
    let empty_events = CombinedEvents {
        num_of_events: 0,
        data: vec![],
    };

    let combined_events = events
        .iter()
        .fold(empty_events, |mut total_events, next_event| {
            total_events.num_of_events += 1;
            total_events.data.push(next_event.to_string());
            total_events
        });
    println!("{:#?}", combined_events);

    let mut number_iter = [7, 8, 9, 10].into_iter();
    let _first_two = number_iter.by_ref().take(2).collect::<Vec<_>>();
    let _second_two = number_iter.take(2).collect::<Vec<_>>();

    let num_vec = vec![1, 2, 3, 4, 5, 6, 7];
    for chunk in num_vec.chunks(3) {
        println!("{:?}", chunk);
    }
    println!();
    for window in num_vec.windows(3) {
        println!("{:?}", window);
    }

    let some_str = "Er ist noch nicht erklärt. Aber es gibt Krieg. Verlaß dich drauf.";
    // match_indices() takes anything that matches a trait called `Pattern`
    // &str, char and even closures can be passsed into this method
    for (index, item) in some_str.match_indices(|c| c > 'z') {
        println!("{} at {}", item, index);
    }
    for (index, item) in some_str.match_indices(". ") {
        println!("{} at {}", item, index);
    }

    let just_numbers = vec![1, 5, 100];
    let mut number_iter = just_numbers.iter().peekable();

    for _ in 0..3 {
        println!("I love the number {}", number_iter.peek().unwrap());
        println!("I really love the number {}", number_iter.peek().unwrap());
        println!("{} is such a nice number", number_iter.peek().unwrap());
        number_iter.next();
    }
}
