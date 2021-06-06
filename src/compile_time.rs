use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::time::SystemTime;

/// Writes the current values to `generated_stamp.rs` in the out directory,
/// intended to be run at compile time from `build.rs`.
pub fn write_stamp_file_at_compile_time(repo_path: &Path, out_dir: &Path) {
    let dest_path = Path::new(&out_dir).join("generated_stamp.rs");

    let rev = git_revision(&repo_path);
    let clean = git_clean(&repo_path);

    let git_revision_cleanness = git_revision_cleanness(&rev, clean);
    let git_status = git_status(&repo_path);

    let mut fd = std::fs::File::create(&dest_path).unwrap();

    let epoch = SystemTime::UNIX_EPOCH
        .elapsed()
        .expect("your clock is off by 50+ years");

    fd.write_all(
        &format!(
            r##"

pub const BUILD_STAMP: build_stamp::BuildStamp = build_stamp::BuildStamp{{
    git_revision: r#"{}"#,
    git_revision_cleanness: r#"{}"#,
    git_clean: {},
    git_status: r#"{}"#,
    build_time_seconds: {},
    build_time_nanos: {},
}};

"##,
            rev,
            git_revision_cleanness,
            clean,
            git_status,
            epoch.as_secs(),
            epoch.subsec_nanos(),
        )
        .as_bytes(),
    )
    .unwrap();
}

fn git_status(repo: &Path) -> String {
    let output = Command::new("git")
        .current_dir(&repo)
        .arg("status")
        .output()
        .expect("git status failed");
    std::str::from_utf8(&output.stdout)
        .expect("git status output not utf-8")
        .to_string()
}

fn git_clean(repo: &Path) -> bool {
    let git_status_porcelain = Command::new("git")
        .current_dir(&repo)
        .arg("status")
        .arg("--porcelain")
        .output()
        .unwrap()
        .stdout;

    // FIXME: This is probably fragile.
    std::str::from_utf8(&git_status_porcelain)
        .unwrap()
        .trim()
        .is_empty()
}

fn git_revision(repo: &Path) -> String {
    let output = Command::new("git")
        .current_dir(&repo)
        .arg("show-ref")
        .arg("--head")
        .arg("HEAD")
        .output()
        .unwrap();
    std::str::from_utf8(&output.stdout).unwrap()[..40].to_string()
}

fn git_revision_cleanness(rev: &str, clean: bool) -> String {
    let kleene_star = if clean { "" } else { "*" };
    format!("{}{}", rev, kleene_star)
}
