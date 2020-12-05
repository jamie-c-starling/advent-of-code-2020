use std::fs;

fn main() {
    let contents = fs::read_to_string("src/bin/resources/5_input.txt")
        .expect("Something went wrong reading the file");

    let seats: Vec<&str> = contents.lines().collect();

    let mut highest_id = 0;

    for seat in seats.iter() {
        let mut lb = 0;
        let mut ub = 127;

        for x in seat[0..=6].chars() {
            match x {
                'F' => ub = (ub + lb) / 2,
                'B' => lb = (ub + lb + 1) / 2,
                _ => panic!(),
            }
        }
        assert_eq!(lb, ub);
        let seat_row = lb;

        lb = 0;
        ub = 7;
        for x in seat[7..].chars() {
            match x {
                'L' => ub = (ub + lb) / 2,
                'R' => lb = (ub + lb + 1) / 2,
                _ => panic!(),
            }
        }
        assert_eq!(lb, ub);
        let seat_column = lb;

        let seat_id = seat_row * 8 + seat_column;
        if seat_id > highest_id {
            highest_id = seat_id;
        }
    }
    println!("{}", highest_id);
}
