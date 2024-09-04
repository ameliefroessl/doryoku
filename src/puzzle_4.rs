use crate::utils::read_file;
use regex::Regex;
use std::io;

// main entrypoint to puzzle_4
// TODO find a nice way to pass u32 here and later convert to u64
pub fn run(input_path: &str) -> Result<u64, io::Error> {
    let content = read_file(input_path)?;

    // parse string into a list of Cards
    let mut cards: Vec<Card> = Vec::new();

    parse_input(content, &mut cards);

    set_winning_numbers_2(&mut cards);

    let answer = solve_part_1(&cards);
    println!("The answer for puzzle 4 part 1 is: {answer}.");

    let answer = solve_part2(&mut cards);
    println!("The answer for puzzle 4 part 2 is : {answer}");
    Ok(u64::from(answer))
}

struct Card {
    card_n: usize,
    winning_sequence: Vec<u32>,
    my_sequence: Vec<u32>,
    copies: u32,
    wins: u32,
}

fn solve_part2(cards: &mut Vec<Card>) -> u32 {
    for card_index in 0..cards.len() {
        // for card in cards {
        let curr_index = cards[card_index].card_n;
        let curr_wins = usize::try_from(cards[card_index].wins).unwrap();

        let trailing_wins = curr_index + curr_wins;
        println!(
            "\nCard {curr_index} with {curr_wins} wins, creating copies of cards: \n{}",
            "-".repeat(45)
        );
        for i in curr_index..trailing_wins {
            if i < cards.len() {
                print!(
                    "Card {:?}. This card had: {:?} copies.",
                    cards[i].card_n, cards[i].copies
                );
                // each copy of our cards wins a copy of the trailing cards.
                cards[i].copies += cards[card_index].copies;
                println!(" It now has {:?} copies. ", cards[i].copies);
            }
            println!();
        }
    }

    let mut sum = 0;

    for card in cards {
        sum += card.copies;
        println!(
            "Card {:?} has {:?} copies. The sum is now : {:?}",
            card.card_n, card.copies, sum
        );
    }

    return sum;
}

fn solve_part_1(cards: &Vec<Card>) -> u32 {
    let mut sum = 0;
    for card in cards {
        sum += card.wins;
    }

    return sum;
}
fn set_winning_numbers_2(cards: &mut Vec<Card>) {
    for card in cards {
        let mut points = 0;
        for my_number in &card.my_sequence {
            if card.winning_sequence.contains(&my_number) {
                points += 1;
            }
        }

        card.wins = points;
    }
}

fn set_winning_numbers(cards: &mut Vec<Card>) {
    for card in cards {
        let mut points = 0;
        for my_number in &card.my_sequence {
            if card.winning_sequence.contains(&my_number) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }

        card.wins = points;
    }
}

fn parse_input(content: String, cards: &mut Vec<Card>) {
    for (ind, card) in content.split('\n').enumerate() {
        // use regular expression to split off the `Card n:` prefix
        let re = Regex::new(r"Card\s+\d+:").unwrap();

        // Replace all occurrences with a desired string, e.g., "Replaced:"
        let striped_card = re.replace_all(&card, "");

        // the first sequence is the winning numbers and the second sequence is my numbers""
        let sequences: Vec<&str> = striped_card.split('|').collect();

        let mut winning_numbers: Vec<u32> = Vec::new();
        let mut my_sequence: Vec<u32> = Vec::new();

        for char_number in sequences[0].split_whitespace() {
            let number = char_number
                .parse()
                .expect(&format!("Could not unwrap {}", char_number));
            winning_numbers.push(number);
        }
        for char_number in sequences[1].split_whitespace() {
            my_sequence.push(char_number.parse().unwrap());
        }

        cards.push(Card {
            card_n: ind + 1,
            winning_sequence: winning_numbers,
            my_sequence: my_sequence,
            copies: 1,
            wins: 0,
        });
    }
}
