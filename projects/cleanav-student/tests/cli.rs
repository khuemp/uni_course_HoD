#![allow(non_snake_case)]
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn terminate_N() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cleanav")?;

    cmd.arg("--slope")
        .arg("1,3")
        .arg("-t")
        .arg("3")
        .arg("-m")
        .arg("./maps/basic.txt")
        .arg("-o")
        .arg("P");

    cmd.assert().stdout(predicate::str::contains("(3, 0)"));

    Ok(())
}

#[test]
fn terminate_N_C() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cleanav")?;

    cmd.arg("--slope")
        .arg("1,2")
        .arg("-t")
        .arg("26")
        .arg("-m")
        .arg("./maps/basic.txt")
        .arg("-o")
        .arg("C,X");

    cmd.assert().stdout(predicate::str::contains("1"));

    let mut cmd = Command::cargo_bin("cleanav")?;

    cmd.arg("--slope")
        .arg("1,3")
        .arg("-t")
        .arg("120")
        .arg("-m")
        .arg("./maps/buoys.txt")
        .arg("-o")
        .arg("C,O");

    cmd.assert().stdout(predicate::str::contains("2"));

    Ok(())
}

#[test]
fn terminate_S_X() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cleanav")?;

    cmd.arg("--slope")
        .arg("1,2")
        .arg("-t")
        .arg("S,X")
        .arg("-m")
        .arg("./maps/basic.txt")
        .arg("-o")
        .arg("P");

    cmd.assert().stdout(predicate::str::contains("(3, 1)"));

    Ok(())
}

#[test]
fn terminate_S_O() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cleanav")?;

    cmd.arg("--slope")
        .arg("3,1")
        .arg("-t")
        .arg("S,O")
        .arg("-m")
        .arg("./maps/buoys.txt")
        .arg("-o")
        .arg("P");

    cmd.assert().stdout(predicate::str::contains("(12, 1)"));

    let mut cmd = Command::cargo_bin("cleanav")?;

    cmd.arg("--slope")
        .arg("3,1")
        .arg("-t")
        .arg("S,O")
        .arg("-m")
        .arg("./maps/buoys.txt")
        .arg("-o")
        .arg("N");

    cmd.assert().stdout(predicate::str::contains("190"));

    let mut cmd = Command::cargo_bin("cleanav")?;

    cmd.arg("--slope")
        .arg("3,1")
        .arg("-t")
        .arg("S,O")
        .arg("-m")
        .arg("./maps/buoys.txt")
        .arg("-o")
        .arg("D");

    cmd.assert().stdout(predicate::str::contains("3"));

    Ok(())
}

#[test]
fn terminate_P() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cleanav")?;

    cmd.arg("--slope")
        .arg("1,2")
        .arg("-t")
        .arg("P,2,1")
        .arg("-m")
        .arg("./maps/basic.txt")
        .arg("-o")
        .arg("P");

    cmd.assert().stdout(predicate::str::contains("(2, 1)"));

    Ok(())
}

#[test]
fn cleanup() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cleanav")?;

    cmd.arg("--slope")
        .arg("1,2")
        .arg("-t")
        .arg("900")
        .arg("-m")
        .arg("./maps/great_pacific_garbage_patch.txt")
        .arg("-o")
        .arg("C,X");

    cmd.assert().stdout(predicate::str::contains("27"));

    Ok(())
}

#[test]
fn my_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cleanav")?;

    cmd.arg("--slope")
        .arg("1,2")
        .arg("-t")
        .arg("P,0,0")
        .arg("-m")
        .arg("./maps/first_visited.txt")
        .arg("-o")
        .arg("D");

    cmd.assert().stdout(predicate::str::contains("1"));

    Ok(())
}
