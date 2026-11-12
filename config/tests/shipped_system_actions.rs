// SPDX-License-Identifier: MPL-2.0

use cosmic_settings_config::shortcuts::{self, action::System};
use std::{env, fs, os::unix::fs::PermissionsExt, path::Path, process::Command};

/// A stand-in for `name` that records the arguments it was run with.
fn stub(bin: &Path, log: &Path, name: &str) {
    let path = bin.join(name);
    let script = format!("#!/bin/sh\necho \"{name} $*\" >> '{}'\n", log.display());
    fs::write(&path, script).unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
}

// One test in its own binary, because it points the XDG directories at a scratch tree.
#[test]
fn chat_panel_opens_chord_the_way_the_launcher_does() {
    let root = env::temp_dir().join(format!("cosmic-settings-shipped-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    let system = root.join("data/cosmic").join(shortcuts::ID).join("v1");
    fs::create_dir_all(&system).unwrap();
    let shipped = Path::new(env!("CARGO_MANIFEST_DIR")).join("../data/system_actions.ron");
    fs::copy(shipped, system.join("system_actions")).unwrap();
    // SAFETY: this binary runs a single test, so no other thread reads the environment.
    unsafe {
        env::set_var("XDG_DATA_HOME", root.join("data"));
        env::set_var("XDG_DATA_DIRS", root.join("none"));
        env::set_var("XDG_CONFIG_HOME", root.join("config"));
    }

    let actions = shortcuts::system_actions(&shortcuts::context().unwrap());
    let command = &actions[&System::ChatPanel];

    let bin = root.join("bin");
    fs::create_dir_all(&bin).unwrap();
    let log = root.join("log");
    stub(&bin, &log, "kora-launch");
    stub(&bin, &log, "gtk-launch");
    let run = |workspaces: &str| {
        let _ = fs::remove_file(&log);
        let status = Command::new("/bin/sh")
            .args(["-c", command])
            .env("PATH", &bin)
            .env("COSMIC_WORKSPACES", workspaces)
            .status()
            .unwrap();
        assert!(status.success(), "{command}");
        fs::read_to_string(&log).unwrap()
    };

    // With workspaces it starts in the active one, as icetron's `AppStart` does from the machine.
    assert_eq!(
        run("1"),
        "kora-launch -- gtk-launch inc.kora.Chord.desktop\n"
    );
    assert_eq!(run(""), "gtk-launch inc.kora.Chord.desktop\n");

    fs::remove_dir_all(&root).unwrap();
}
