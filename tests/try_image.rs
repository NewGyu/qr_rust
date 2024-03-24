use image::io::Reader as ImageReader;
use std::path::PathBuf;

use testresult::TestResult;

#[test]
fn test_try_image() -> TestResult {
    let _x = ImageReader::open(get_resouce_path("example1.png"))?
        .with_guessed_format()?
        .decode()?;
    Ok(())
}

fn get_resouce_path(name: &str) -> std::path::PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/resources");
    path.push(name);
    path
}
