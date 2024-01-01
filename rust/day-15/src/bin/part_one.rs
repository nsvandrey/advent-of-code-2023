fn main() {
    let input = include_str!("input.txt");
    let output = part_one(input);
    dbg!(output);
}

fn part_one(input: &str) -> u32 {
    let result = input
        .split(",")
        .map(|step| {
            let mut current_value = 0;
            for c in step.chars() {
                current_value += c as u32;
                current_value = current_value * 17;
                current_value = current_value % 256;
            }

            current_value
        })
        .sum();

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(result, 1320)
    }
}
