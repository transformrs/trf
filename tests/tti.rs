mod common;

use common::load_key;
use common::trf;
use transformrs::Provider;

#[test]
fn no_args() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir().unwrap();
    let mut cmd = trf();
    let key = load_key(&Provider::DeepInfra);
    let cmd = cmd
        .arg("tti")
        .env("DEEPINFRA_KEY", key)
        .write_stdin("Hello world")
        .current_dir(&dir);
    let output = cmd.assert().success().get_output().stdout.clone();

    assert!(output.len() > 0);

    Ok(())
}

fn default_settings_helper(provider: &Provider) -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir().unwrap();
    let mut cmd = trf();
    let key = load_key(provider);
    let name = provider.key_name();
    cmd.arg("--verbose")
        .arg("tti")
        .arg("--output=myfile")
        .arg("--width=128")
        .arg("--height=128")
        .arg("--steps=10")
        .arg("--cfg-scale=3")
        .env(name, key)
        .write_stdin("image of a beach")
        .current_dir(&dir)
        .assert()
        .success();

    let path = dir.path().join("myfile.png");
    assert!(path.exists());

    Ok(())
}

#[test]
fn default_settings_deepinfra() -> Result<(), Box<dyn std::error::Error>> {
    default_settings_helper(&Provider::DeepInfra)
}
