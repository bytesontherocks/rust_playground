mod checksums {
    pub fn is_checksum_valid(f: u64) -> bool {
        let (f, check_value) = calculate_mod(f);        

        if check_value == calculate_checksum_ex1(f) {
            return true;
        }
        
        return false;
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
            return (0,0);
        } else {
            return (f/10, f%10);
        }
    }
}


fn main() {

    let correct_value = 41255501238;

    let is_number_valid = checksums::is_checksum_valid(correct_value);

    println!("The number {} has a crc valid == {}", correct_value, is_number_valid);

    
}