use std::process::Command;

use assert_cmd::{assert::OutputAssertExt, output::OutputOkExt};

// true で終了するか？
#[test]
fn works() {
    assert!(true);
}

// コマンドの実行結果が成功するか？
#[test]
fn runs() {
    let mut cmd = Command::new("hello").unwrap();
    cmd.assert().success();
}
