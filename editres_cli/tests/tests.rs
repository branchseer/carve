use editres_cli::{inject, list};
use std::sync::Once;
use std::vec;
use std::{
    env,
    path::PathBuf,
    process::{Command, Output},
};
use tempfile::TempPath;

fn build_testbin_once() {
    static BUILD_ONCE: Once = Once::new();
    BUILD_ONCE.call_once(|| {
        let build_status = Command::new(env::var("CARGO").unwrap())
            .args(["build", "--release", "-p", "testbin", "--all-targets"])
            .env("CARGO_TARGET_DIR", env!("CARGO_TARGET_TMPDIR"))
            .status()
            .unwrap();
        assert!(build_status.success());
    });
}

fn testbin_command(name: &str) -> Command {
    build_testbin_once();
    Command::new(format!("{}/release/{}", env!("CARGO_TARGET_TMPDIR"), name))
}

fn temp_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    path.push(fastrand::u128(..).to_string());
    path
}

fn get_printed_resources(output: &Output) -> Vec<Option<&[u8]>> {
    assert_eq!(output.status.success(), true);
    bincode::deserialize(&output.stdout).unwrap()
}

#[test]
fn test_basic() {
    let mut single_command = testbin_command("single");
    let output = single_command.output().unwrap();
    assert_eq!(get_printed_resources(&output), &[None]);

    let resources = list(single_command.get_program()).unwrap();
    assert_eq!(&resources, &[("my_res".to_string(), false)]);

    let mut injected_path = temp_path();
    inject(
        single_command.get_program(),
        "my_res",
        b"res_content",
        &mut injected_path,
    )
    .unwrap();

    let resources = list(&injected_path).unwrap();
    assert_eq!(&resources, &[("my_res".to_string(), true)]);

    let _path_guard = TempPath::from_path(&injected_path);
    let output = Command::new(injected_path).output().unwrap();
    assert_eq!(
        get_printed_resources(&output),
        &[Some(b"res_content".as_slice())]
    );
}

#[test]
fn test_zero_size_resource() {
    let single_command = testbin_command("single");
    let mut injected_path = temp_path();
    inject(
        single_command.get_program(),
        "my_res",
        b"",
        &mut injected_path,
    )
    .unwrap();

    let _path_guard = TempPath::from_path(&injected_path);
    let output = Command::new(injected_path).output().unwrap();
    assert_eq!(get_printed_resources(&output), &[Some(b"".as_slice())]);
}

#[test]
fn test_multiple_resources() {
    let mut double_command = testbin_command("double");
    let output = double_command.output().unwrap();
    assert_eq!(get_printed_resources(&output), &[None, None]);
    let resources = list(double_command.get_program()).unwrap();
    assert_eq!(
        &resources,
        &[
            ("my_res1".to_string(), false),
            ("my_res2".to_string(), false)
        ]
    );

    let mut injected_path = temp_path();
    inject(
        double_command.get_program(),
        "my_res2",
        b"res2_content",
        &mut injected_path,
    )
    .unwrap();
    let _path_guard: TempPath = TempPath::from_path(&injected_path);

    assert_eq!(
        &list(&injected_path).unwrap(),
        &[
            ("my_res1".to_string(), false),
            ("my_res2".to_string(), true)
        ]
    );
    assert_eq!(
        get_printed_resources(&Command::new(&injected_path).output().unwrap()),
        &[None, Some(b"res2_content".as_slice())]
    );

    let mut injected_path2 = temp_path();
    inject(
        injected_path,
        "my_res1",
        b"res1_content",
        &mut injected_path2,
    )
    .unwrap();
    let _path_guard2: TempPath = TempPath::from_path(&injected_path2);

    assert_eq!(
        &list(&injected_path2).unwrap(),
        &[("my_res1".to_string(), true), ("my_res2".to_string(), true)]
    );
    assert_eq!(
        get_printed_resources(&Command::new(&injected_path2).output().unwrap()),
        &[
            Some(b"res1_content".as_slice()),
            Some(b"res2_content".as_slice())
        ]
    );
}

#[test]
fn test_resource_existed() {
    let single_command = testbin_command("single");

    let mut injected_path = temp_path();
    inject(
        single_command.get_program(),
        "my_res",
        b"res_content",
        &mut injected_path,
    )
    .unwrap();
    let _path_guard: TempPath = TempPath::from_path(&injected_path);

    let mut injected_path2 = temp_path();
    let inject_result = inject(injected_path, "my_res", b"res_content", &mut injected_path2);
    let _path_guard2: TempPath = TempPath::from_path(&injected_path2);
    assert!(inject_result
        .err()
        .unwrap()
        .to_string()
        .contains("already exists"))
}

#[test]
fn test_duplicated_resource_declarations() {
    let mut dup_command = testbin_command("dup");
    let output = dup_command.output().unwrap();
    assert_eq!(get_printed_resources(&output), &[None, None]);

    let resources = list(dup_command.get_program()).unwrap();
    assert_eq!(&resources, &[("my_res".to_string(), false)]);

    let mut injected_path = temp_path();
    inject(
        dup_command.get_program(),
        "my_res",
        b"res_content",
        &mut injected_path,
    )
    .unwrap();

    let resources = list(&injected_path).unwrap();
    assert_eq!(&resources, &[("my_res".to_string(), true)]);

    let _path_guard = TempPath::from_path(&injected_path);
    let output = Command::new(injected_path).output().unwrap();
    assert_eq!(
        get_printed_resources(&output),
        &[
            Some(b"res_content".as_slice()),
            Some(b"res_content".as_slice())
        ]
    );
}

#[test]
fn test_large_resource() {
    let large_resource = vec![0u8; 1 * 1024 * 1024 * 1024];

    let single_command = testbin_command("single");

    let mut injected_path = temp_path();

    inject(
        single_command.get_program(),
        "my_res",
        &large_resource,
        &mut injected_path,
    )
    .unwrap();
    let _path_guard = TempPath::from_path(&injected_path);

    let resources = list(&injected_path).unwrap();
    assert_eq!(&resources, &[("my_res".to_string(), true)]);

    let output = Command::new(injected_path).output().unwrap();
    assert_eq!(
        get_printed_resources(&output),
        &[Some(large_resource.as_slice())]
    );
}
