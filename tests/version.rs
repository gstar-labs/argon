//! An external consumer can inspect the package version without an allocator.

#[test]
fn exposes_the_package_version() {
    assert_eq!(argon::VERSION, "0.1.0");
}
