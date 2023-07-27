fn main() {
    #[cfg(feature = "with_metadata")]
    built::write_built_file().expect("Failed to acquire build-time information");
}
