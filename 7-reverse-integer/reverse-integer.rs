impl Solution {

pub fn reverse(x: i32) -> i32 {
    let is_negative = x < 0;
    let mut x = x.abs();
    let mut reversed: usize = 0;

    while x > 0 {
        let n = x % 10;
        reversed *= 10;
        reversed += n as usize;
        x /= 10;
    }

    if reversed > i32::MAX as usize {
        return 0;
    }

    if is_negative {
        return reversed as i32 * -1;
    }

    reversed as i32
}


}