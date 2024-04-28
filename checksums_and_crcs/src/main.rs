mod checksums {
    pub fn is_checksum_valid(f: u64) -> bool {
        let (dev, m) = calculate_mod(f);

        if f > 0 {
            return true;
        }
        
        return false;
    }

    fn calculate_mod(f: u64) -> (u64, u64) {
        
        if f == 0 {
            return(0,0);
        } else {
            return (f>>10, f%10);
        }
    }
}


fn main() {

    let c = checksums::is_checksum_valid(0);
}