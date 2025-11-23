#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Ascending,
    Descending,
}

impl Solution {


    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;

        if num_rows == 1 {
            return s;
        }

        let mut words: Vec<String> = vec![String::new(); num_rows];
        let mut row = 0;
        let mut direction = Direction::Descending;

        for c in s.chars() {
            words.get_mut(row).map(|word| word.push(c));

            if row == num_rows - 1 {
                direction = Direction::Ascending;
            } else if row == 0 {
                direction = Direction::Descending;
            }

            match direction {
                Direction::Ascending => row -= 1,
                Direction::Descending => row += 1,
            }
        }
        words.join("")
    }


}