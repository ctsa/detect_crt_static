fn main() {
    #[cfg(target_feature = "crt-static")]
    panic!("Detected crt-static mode");
}