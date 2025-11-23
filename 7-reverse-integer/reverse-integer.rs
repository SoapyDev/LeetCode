impl Solution {
pub fn reverse(x: i32) -> i32 {
    let is_negative = x < 0;
    let mut x = x.abs();
    let mut numbers = vec![];

    while x > 0 {
        numbers.push(x % 10);
        x /= 10;
    }

    let mut reversed: usize = 0;
    let mut multiple: usize = 1;
    numbers.into_iter().rev().for_each(|n| {
        reversed += multiple * n as usize;
        multiple *= 10;
    });

    if reversed > i32::MAX as usize {
        return 0;
    }

    if is_negative {
        return reversed as i32 * -1;
    }

    reversed as i32
}

}