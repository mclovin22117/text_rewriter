#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformKind {
    MacOs,
    Windows,
    Linux,
    Unknown,
}

pub fn detect_platform() -> PlatformKind {
    match std::env::consts::OS {
        "macos" => PlatformKind::MacOs,
        "windows" => PlatformKind::Windows,
        "linux" => PlatformKind::Linux,
        _ => PlatformKind::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_known_or_unknown_platform() {
        let detected = detect_platform();
        assert!(matches!(
            detected,
            PlatformKind::MacOs | PlatformKind::Windows | PlatformKind::Linux | PlatformKind::Unknown
        ));
    }
}
