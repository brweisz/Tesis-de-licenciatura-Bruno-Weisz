// use assert_cmd::prelude::*;
// use predicates::prelude::*;
// use std::process::Command;
//
// use assert_fs::prelude::{FileWriteStr, PathAssert, PathChild};
//
// fn build_custom_nargo() -> bool {
//     false
// }
//
// fn build_custom_plonky2() -> bool {
//     false
// }
//
// fn build_plonky2_backend() -> bool {
//     false
// }
//
// fn deploy_plonky2_backend() -> bool {
//     false
// }


#[test]
fn integration_test() {
    // assert_eq!(true, build_custom_nargo());
    // assert_eq!(true, build_custom_plonky2());
    // assert_eq!(true, build_plonky2_backend());
    // assert_eq!(true, deploy_plonky2_backend()); // Puts the backend binary where nargo expects it

    // let test_dir = assert_fs::TempDir::new().unwrap();
    // std::env::set_current_dir(&test_dir).unwrap();
    //
    // let project_name = "integration_test";
    // let project_dir = test_dir.child(project_name);
    //
    // // `nargo new hello_world`
    // let custom_nargo_path = "/home/bweisz/Documentos/proyectos/ethf-grants/plonky-2-backend-for-acir/noir/target/debug/nargo";
    // let mut cmd = Command::new(custom_nargo_path);
    // cmd.arg("new").arg(project_name);
    // cmd.assert().success().stdout(predicate::str::contains("Project successfully created!"));
    //
    // project_dir.child("src").assert(predicate::path::is_dir());
    // project_dir.child("Nargo.toml").assert(predicate::path::is_file());
    //
    // std::env::set_current_dir(&project_dir).unwrap();
    //
    // // `nargo check`
    // let mut cmd = Command::new(custom_nargo_path);
    // cmd.arg("check");
    // cmd.assert().success().stdout(predicate::str::contains("Constraint system successfully built!"));
    //
    // project_dir.child("Prover.toml").assert(predicate::path::is_file());
    // project_dir.child("Verifier.toml").assert(predicate::path::is_file());
    //
    // // `nargo prove`
    // project_dir.child("Prover.toml").write_str("x = 1\ny = 2").unwrap(); // Example program
    //
    // let mut cmd = Command::new(custom_nargo_path);
    // cmd.env("NARGO_BACKEND_PATH", "~/.nargo/backends/acvm-backend-plonky2/backend_binary");
    // cmd.arg("prove");
    // cmd.assert().success();
    //
    // project_dir
    //     .child("proofs")
    //     .child(format!("{project_name}.proof"))
    //     .assert(predicate::path::is_file());
    //
    // // nargo verify
    // let mut cmd = Command::new(custom_nargo_path);
    // cmd.arg("verify");
    // cmd.assert().success();
}