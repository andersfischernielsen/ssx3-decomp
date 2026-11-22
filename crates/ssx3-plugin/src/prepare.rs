//! Making a plugin's content from the user's disc: the plugin's own importer,
//! run once.

use std::path::Path;
use std::process::Command;

use crate::discover::Found;

/// Run the plugin's `prepare` command in its folder. It reads the disc and
/// writes the content folder, then creates `ready` there. `base_disc` is the
/// SSX 3 image.
pub fn prepare(f: &Found, base_disc: Option<&Path>) -> Result<(), String> {
    let m = f
        .manifest
        .as_ref()
        .ok_or_else(|| format!("plugin {}: no usable manifest", f.id))?;
    let cmd = &m
        .prepare
        .as_ref()
        .ok_or_else(|| {
            format!(
                "plugin {}: nothing to prepare (its folder ships its content)",
                f.id
            )
        })?
        .command;
    std::fs::create_dir_all(&f.content).map_err(|e| format!("{}: {e}", f.content.display()))?;
    let _ = std::fs::remove_file(f.content.join("ready"));
    let mut c = Command::new(&cmd[0]);
    c.args(&cmd[1..])
        .current_dir(&f.dir)
        .env("SSX3_PLUGIN_ID", &f.id)
        .env("SSX3_PLUGIN_OUT", &f.content);
    if let Some(d) = &f.disc {
        c.env("SSX3_PLUGIN_DISC", d);
    }
    if let Some(d) = base_disc {
        c.env("SSX3_BASE_DISC", d);
    }
    let status = c
        .status()
        .map_err(|e| format!("plugin {}: cannot run {:?}: {e}", f.id, cmd[0]))?;
    if !status.success() {
        return Err(format!("plugin {}: {:?} failed ({status})", f.id, cmd[0]));
    }
    if !f.content.join("ready").is_file() {
        return Err(format!(
            "plugin {}: {:?} finished but wrote no `ready` file in {}",
            f.id,
            cmd[0],
            f.content.display()
        ));
    }
    Ok(())
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use crate::discover::{discover, testutil::*, Filter, Roots, Status};

    fn setup(t: &Tmp, command: &str) -> Roots {
        let d = t.path().join("plugins/imp");
        std::fs::create_dir_all(&d).unwrap();
        std::fs::write(
            d.join("plugin.toml"),
            format!("[plugin]\nid = \"imp\"\napi = \"0.1\"\n[prepare]\ncommand = {command}\n"),
        )
        .unwrap();
        Roots {
            plugin_dirs: vec![t.path().join("plugins")],
            disc_dirs: vec![],
            cache: t.path().join("cache"),
        }
    }

    #[test]
    fn prepare_runs_the_importer_with_its_environment() {
        let t = Tmp::new("prep");
        let r = setup(
            &t,
            r#"["sh", "-c", "mkdir -p \"$SSX3_PLUGIN_OUT/overlay\" && echo $SSX3_PLUGIN_ID > \"$SSX3_PLUGIN_OUT/overlay/id\" && touch \"$SSX3_PLUGIN_OUT/ready\""]"#,
        );
        let f = &discover(&r, &Filter::All)[0];
        assert_eq!(f.status, Status::NeedsPrepare);
        prepare(f, None).unwrap();
        assert_eq!(
            std::fs::read_to_string(f.overlay().join("id"))
                .unwrap()
                .trim(),
            "imp"
        );
        assert_eq!(discover(&r, &Filter::All)[0].status, Status::Ready);
    }

    #[test]
    fn a_failing_or_incomplete_importer_is_an_error() {
        let t = Tmp::new("prepfail");
        let r = setup(&t, r#"["sh", "-c", "exit 3"]"#);
        let e = prepare(&discover(&r, &Filter::All)[0], None).unwrap_err();
        assert!(e.contains("failed"), "{e}");
        let t = Tmp::new("prepready");
        let r = setup(&t, r#"["true"]"#);
        let e = prepare(&discover(&r, &Filter::All)[0], None).unwrap_err();
        assert!(e.contains("no `ready` file"), "{e}");
    }
}
