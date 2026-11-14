//! Screenshot file handling for the desktop frontend.

/// Save a complete frame without overwriting captures from earlier runs.
pub fn save(
    dir: &std::path::Path,
    next: &mut u64,
    width: u32,
    height: u32,
    rgba: &[u32],
) -> std::io::Result<std::path::PathBuf> {
    use std::io::{Error, ErrorKind};
    // The encoder uses u32 arithmetic for its scanline allocation.
    let size = width.checked_mul(height);
    let bytes = size.and_then(|n| n.checked_mul(4)?.checked_add(height));
    if width == 0 || height == 0 || bytes.is_none() || size.map(|n| n as usize) != Some(rgba.len())
    {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "no complete frame available",
        ));
    }
    let data = crate::png::encode(width, height, rgba);
    loop {
        let path = dir.join(format!("ssx3-shot-{next}.png"));
        *next = next
            .checked_add(1)
            .ok_or_else(|| Error::other("screenshot counter exhausted"))?;
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
        {
            Ok(file) => {
                write_capture(file, &path, &data)?;
                return Ok(path);
            }
            Err(e) if e.kind() == ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(e),
        }
    }
}

fn write_capture(
    mut file: impl std::io::Write,
    path: &std::path::Path,
    data: &[u8],
) -> std::io::Result<()> {
    let result = file.write_all(data).and_then(|()| file.flush());
    // Windows must have the file closed before it can be removed. Preserve
    // the write error even if best-effort cleanup also fails.
    drop(file);
    if result.is_err() {
        let _ = std::fs::remove_file(path);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::ErrorKind;

    #[test]
    fn screenshots_use_the_directory_and_preserve_existing_files() {
        let dir = std::env::temp_dir().join(format!("ssx3-shots-test-{}", std::process::id()));
        std::fs::create_dir(&dir).unwrap();
        let result = (|| -> std::io::Result<()> {
            let existing = dir.join("ssx3-shot-0.png");
            std::fs::write(&existing, b"existing capture")?;
            let mut next = 0;
            let path = save(&dir, &mut next, 2, 1, &[0xFF0000FF, 0xFF00FF00])?;
            assert_eq!(path, dir.join("ssx3-shot-1.png"));
            assert_eq!(next, 2);
            assert_eq!(std::fs::read(existing)?, b"existing capture");
            let png = std::fs::read(path)?;
            assert_eq!(&png[..8], b"\x89PNG\r\n\x1a\n");
            assert_eq!(&png[16..24], &[0, 0, 0, 2, 0, 0, 0, 1]);
            Ok(())
        })();
        // Delete only the test's known files and directory.
        for name in ["ssx3-shot-0.png", "ssx3-shot-1.png"] {
            let _ = std::fs::remove_file(dir.join(name));
        }
        std::fs::remove_dir(&dir).unwrap();
        result.unwrap();
    }

    #[test]
    fn missing_or_incomplete_frames_are_rejected_before_writing() {
        let mut next = 0;
        for (w, h, pixels) in [(0, 0, &[][..]), (2, 1, &[0][..]), (u32::MAX, 2, &[][..])] {
            let error = save(&std::env::temp_dir(), &mut next, w, h, pixels).unwrap_err();
            assert_eq!(error.kind(), ErrorKind::InvalidInput);
            assert_eq!(next, 0);
        }
    }

    #[test]
    fn write_errors_are_returned_to_the_caller() {
        // A directory below a file can never be a valid destination.
        let file = std::env::current_exe().unwrap();
        assert!(save(&file.join("shots"), &mut 0, 1, 1, &[0]).is_err());
    }

    #[test]
    fn partial_write_and_flush_failures_remove_only_the_new_capture() {
        use std::io::Write;

        struct FailingFile {
            file: std::fs::File,
            fail_flush: bool,
            wrote: bool,
        }
        impl Write for FailingFile {
            fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
                if !self.fail_flush && self.wrote {
                    return Err(std::io::Error::other("injected write failure"));
                }
                self.wrote = true;
                let count = if self.fail_flush {
                    bytes.len()
                } else {
                    bytes.len().min(8)
                };
                self.file.write(&bytes[..count])
            }

            fn flush(&mut self) -> std::io::Result<()> {
                if self.fail_flush {
                    Err(std::io::Error::other("injected flush failure"))
                } else {
                    self.file.flush()
                }
            }
        }

        let dir = std::env::temp_dir().join(format!("ssx3-write-failure-{}", std::process::id()));
        std::fs::create_dir(&dir).unwrap();
        let existing = dir.join("existing.png");
        std::fs::write(&existing, b"keep this capture").unwrap();
        let data = crate::png::encode(1, 1, &[0]);
        for fail_flush in [false, true] {
            let path = dir.join(format!("new-{fail_flush}.png"));
            let file = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path)
                .unwrap();
            let writer = FailingFile {
                file,
                fail_flush,
                wrote: false,
            };
            let error = write_capture(writer, &path, &data).unwrap_err();
            assert_eq!(
                error.to_string(),
                if fail_flush {
                    "injected flush failure"
                } else {
                    "injected write failure"
                }
            );
            assert!(
                !path.exists(),
                "failed capture must be removed after closing its file"
            );
            assert_eq!(std::fs::read(&existing).unwrap(), b"keep this capture");
        }
        std::fs::remove_file(existing).unwrap();
        std::fs::remove_dir(dir).unwrap();
    }
}
