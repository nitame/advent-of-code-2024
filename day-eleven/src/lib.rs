fn blinks(stones: Vec<String>, number_of_blinks: usize) -> usize {
    let mut rearranged_stones = vec![stones];
    for i in 0..number_of_blinks {
        let mut intermediate_stones = Vec::new();
        for j in 0..rearranged_stones[i].len() {
            if rearranged_stones[i][j] == "0" {
                intermediate_stones.push("1".to_string());
            } else if rearranged_stones[i][j].len() % 2 == 0 {
                let middle_idx = rearranged_stones[i][j].len() / 2;
                let stones = rearranged_stones[i][j].clone();
                let left = &stones[0..middle_idx].to_string();
                intermediate_stones.push(left.clone());
                let right = &stones[middle_idx..].to_string();
                let mut tr = right.trim_start_matches("0").to_string();
                if tr == "" {
                    tr = "0".to_string()
                }
                intermediate_stones.push(tr);
            } else {
                let n = rearranged_stones[i][j].clone();
                let m = n.parse::<u64>().unwrap() * 2024;
                let o = m.to_string();
                intermediate_stones.push(o);
            }
        }
        rearranged_stones.push(intermediate_stones);
    }
    let re_stones = rearranged_stones.last().unwrap();
    re_stones.iter().count()
}

#[cfg(test)]
mod tests {
    use crate::blinks;

    #[test]
    fn it_returns_22() {
        let test_input = "125 17";
        let data = test_input
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let result = blinks(data, 6);
        assert_eq!(result, 22);
    }

    #[test]
    fn it_returns_55312() {
        let test_input = "125 17";
        let data = test_input
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let result = blinks(data, 25);
        assert_eq!(result, 55312);
    }

    #[test]
    fn it_returns_puzzle_1_score() {
        let test_input = "1 24596 0 740994 60 803 8918 9405859";
        let data = test_input
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let result = blinks(data, 25);
        assert_eq!(result, 203457);
    }
}
