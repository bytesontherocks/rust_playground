mod checksums {
    pub fn is_checksum_valid(f: u64) -> bool {
        let (f, check_value) = calculate_mod(f);

        if check_value == calculate_checksum_ex1(f) {
            return true;
        }

        return false;
    }

    pub fn calculate_isbn_10_validity(s_isbn10: String) -> bool {
        let c_vec: Vec<char> = s_isbn10.chars().collect();
        assert!(c_vec.len() == 10);

        let mut sum: usize = 0;

        for n in 0..9 {      
            let d = c_vec[n].to_digit(10).ok_or("Not a numeric character");      
            //sum += Ok(d) * (n+1);
        }

        print!("sum: {}", sum.to_string());

        true
    }

    fn num_digits(num: i32) -> Vec<i32> {
        let mut x = num;
        let mut result: Vec<i32> = Vec::new();

        loop {
            result.push(x % 10);
            x /= 10;
            if x == 0 {
                break;
            }
        }

        result.reverse();
        result
    }

    // 10 digit number + the top right one being the check value. ex1 is adding all digits in the data_value
    // and do modulo 10 to calculate the check value.
    fn calculate_checksum_ex1(f: u64) -> u64 {
        let mut checksum = 0;
        let mut value_to_checksum = f;

        while value_to_checksum > 0 {
            let (new_value, m) = calculate_mod(value_to_checksum);
            checksum += m;
            value_to_checksum = new_value;
        }

        checksum %= 10;

        return checksum;
    }

    fn calculate_mod(f: u64) -> (u64, u64) {
        if f == 0 {
            return (0, 0);
        } else {
            return (f / 10, f % 10);
        }
    }
}

#[cfg(test)]
mod checksums_tests {
    use super::*;

    #[test]
    fn ten_digit_number_checksum_included() {
        let correct_value = 41255501238; //4125550123 (data value) and 8 (check value)
        let is_number_valid = checksums::is_checksum_valid(correct_value);
        assert!(is_number_valid, "checksum calculation is not valid");
    }

    #[test]
    fn test_isbn10() {
        let correct_value: u64 = 4125550123;
        assert!(checksums::calculate_isbn_10_validity(correct_value.to_string()));
    }
}

fn main() {}
