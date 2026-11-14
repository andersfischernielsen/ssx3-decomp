//! Screenshot file handling for the desktop frontend.

/// Save a complete frame without overwriting captures from earlier runs.
pub fn save(
    dir: &std::path::Path,
    next: &mut u64,
    width: u32,
    height: u32,
    rgba: &[u32],
) -> std::io::Result<std::path::PathBuf> {
    use std::io::{Error, ErrorKind, Write};
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
            Ok(mut file) => {
                file.write_all(&data)?;
                file.flush()?;
                return Ok(path);
            }
            Err(e) if e.kind() == ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(e),
        }
    }
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
}
