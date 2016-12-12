extern crate lcov_parser;
extern crate tempdir;

use lcov_parser:: { merge_files };
use tempdir::TempDir;

fn main() {
    let trace_files = [
        "../../../tests/fixtures/fixture1.info",
        "../../../tests/fixtures/fixture2.info"
    ];
    let _ = match merge_files(&trace_files) {
        Ok(report) => {
            let tmp_dir = TempDir::new("example_report").expect("create temp dir");
            let file_path = tmp_dir.path().join("merged_report.lcov");
            report.save_as(file_path.as_path().clone())
        },
        Err(err) => panic!(err)
    };
}
