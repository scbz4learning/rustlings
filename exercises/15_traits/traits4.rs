trait Licensed {
    fn licensing_info(&self) -> String {
        "Default license".to_string()
    }
}

struct SomeSoftware;
struct OtherSoftware;

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// TODO: Fix the compiler error by only changing the signature of this function.
fn compare_license_types(software1: Box<dyn Licensed>, software2: Box<dyn Licensed>) -> bool {
    software1.licensing_info() == software2.licensing_info()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        assert!(compare_license_types(Box::new(SomeSoftware), Box::new(OtherSoftware)));
    }

    #[test]
    fn compare_license_information_backwards() {
        assert!(compare_license_types(Box::new(OtherSoftware), Box::new(SomeSoftware)));
    }
}
