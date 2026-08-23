//! The OpenCode setup system.
//!
//! This file is the harness's *facts*. Every command over them lives in
//! [`harness_runtime`], shared with the other setup systems, so a change to
//! behaviour lands once and a change to OpenCode's surface lands here.
//!
//! Unlike its siblings, this harness's routes were not read off the product's
//! documentation alone: `ai-stp` already carries a composition table for
//! `opencode`, and the namespaces below are that table. Where a consumer has
//! already decided where a component goes, agreeing with it is not a shortcut --
//! disagreeing would make every bundle it sends land somewhere it will not look.

use std::process::ExitCode;

use harness_runtime::Harness;
use provider_v3::{ComponentKind, ProjectionKind};

/// Everything specific to OpenCode, verified against `opencode-baseline.json`.
pub const OPENCODE: Harness = Harness {
    harness_id: "opencode",
    provider_id: "opencode-setup-system",
    version: env!("CARGO_PKG_VERSION"),
    product: "OpenCode",
    vendor: "Anomaly",
    // XDG-based, and the only harness here whose home is not a dotted directory
    // under `$HOME`. The env override is what a caller should set; nothing is
    // ever inferred from either, because every command takes an explicit target.
    documented_config_home: "~/.config/opencode",
    config_home_env: "OPENCODE_CONFIG_DIR",
    control_directory: ".opencode-setup-system",
    state_file: "NDDEV-OPENCODE-PROVIDER.json",
    profile_id: "opencode/native-files/1",
    // Everything outside this list is a sibling overlay preserved verbatim.
    // `opencode.json` carries both the settings and the MCP entries: the product
    // reads one document, so this provider owns one file rather than inventing a
    // second that nothing would read.
    native_namespaces: &[
        "AGENTS.md",
        "opencode.json",
        "skills",
        "agents",
        "commands",
        "plugins",
    ],
    // The product's own: credentials and runtime caches. Never read, never
    // written, and never copied into a backup slot.
    never_touch: &["auth.json", "cache"],
    permission_profiles: &["default"],
    component_kinds: &[
        ComponentKind::Instruction,
        ComponentKind::Skill,
        ComponentKind::Agent,
        ComponentKind::Command,
        ComponentKind::Mcp,
        ComponentKind::Plugin,
        ComponentKind::Setting,
    ],
    projection_kinds: &[ProjectionKind::NativeFiles, ProjectionKind::Plugin],
    max_files: 8192,
    max_bytes: 64 * 1024 * 1024,
    kit_identity: include_str!("../../../provider-kit/v3/KIT-IDENTITY.json"),
};

fn main() -> ExitCode {
    harness_runtime::run(&OPENCODE, std::env::args().skip(1).collect())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic)]

    use super::*;

    #[test]
    fn the_declaration_is_valid_and_names_this_host() {
        let info = OPENCODE.provider_info().unwrap();
        assert_eq!(info.provider_id, env!("CARGO_PKG_NAME"));
        assert_eq!(info.harness_id, "opencode");
        assert_eq!(info.protocol_version, 3);
        assert!(info.supports_this_host());
    }

    #[test]
    fn no_namespace_is_both_owned_and_disclaimed() {
        for name in OPENCODE.never_touch {
            assert!(
                !OPENCODE.native_namespaces.contains(name),
                "{name} is claimed and disclaimed"
            );
        }
    }

    #[test]
    fn the_baseline_this_harness_cites_is_present_and_readable() {
        // The facts above are transcribed from it; a build whose baseline is
        // missing has no evidence for what it claims to own.
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../references/opencode-baseline.json");
        let value: serde_json::Value =
            serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        assert!(value.is_object());
    }

    #[test]
    fn the_control_directory_and_state_file_are_provider_owned_not_product_owned() {
        assert!(OPENCODE.control_directory.contains("setup-system"));
        assert!(OPENCODE.state_file.starts_with("NDDEV-"));
        assert!(!OPENCODE.native_namespaces.contains(&OPENCODE.state_file));
    }

    #[test]
    fn the_settings_document_is_owned_under_exactly_one_name() {
        // OpenCode also reads `opencode.jsonc`. Owning both would let a target
        // hold two documents that disagree, with the product picking one and
        // this provider reporting the other; owning one keeps the answer single.
        assert!(OPENCODE.native_namespaces.contains(&"opencode.json"));
        assert!(!OPENCODE.native_namespaces.contains(&"opencode.jsonc"));
    }
}
