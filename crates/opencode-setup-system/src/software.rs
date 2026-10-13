//! Opencode's own program, as measured rather than as described.
//!
//! Generated from the `software_artifacts` block of
//! `references/opencode-baseline.json`. Every member path below was read out
//! of the archive it names, not assumed: codex's carries the target triple and
//! so genuinely differs per platform.
//!
//! Where a `previous_software_artifacts` block is present, it is transcribed
//! too. It is not a second choice: the outgoing current pin is stored there on
//! a bump, so the pair is always two consecutive real releases and there is
//! still exactly one value to keep fresh.
//!
//! Do not edit. The test at the bottom re-reads that baseline and compares it
//! field by field, so an edit here fails rather than silently installing bytes
//! nobody measured.

use harness_runtime::{Artifact, Delivery, Previous, Shape, Software};

/// The artifacts opencode is published as.
pub(crate) const ARTIFACTS: &[Artifact] = &[
    Artifact {
        platform: "linux/arm64",
        url: "https://registry.npmjs.org/opencode-linux-arm64/-/opencode-linux-arm64-1.19.0.tgz",
        bytes: 60_103_843,
        sha256: "sha256:8481102c5d475013302d2f4766cea02b7390e76e74c146408a8cebe2b7f6778f",
        shape: Shape::GzipTar,
        member: "package/bin/opencode",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://registry.npmjs.org/opencode-linux-x64/-/opencode-linux-x64-1.19.0.tgz",
        bytes: 60_325_293,
        sha256: "sha256:6ce9b8497f680c152a8d65b78640c0b8840d8b0f344530ea30b6c4b69e5a7282",
        shape: Shape::GzipTar,
        member: "package/bin/opencode",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://registry.npmjs.org/opencode-darwin-arm64/-/opencode-darwin-arm64-1.19.0.tgz",
        bytes: 45_290_684,
        sha256: "sha256:2fccf11218322be8de2d09ee0bf7d7fc2a90faeb3a7828991cc0e13273cc0b85",
        shape: Shape::GzipTar,
        member: "package/bin/opencode",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://registry.npmjs.org/opencode-darwin-x64/-/opencode-darwin-x64-1.19.0.tgz",
        bytes: 48_899_014,
        sha256: "sha256:858048a287cc3deab0b5630fbd756578156d31504c2daae7038ea62d99ebdc3d",
        shape: Shape::GzipTar,
        member: "package/bin/opencode",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://registry.npmjs.org/opencode-windows-arm64/-/opencode-windows-arm64-1.19.0.tgz",
        bytes: 58_554_722,
        sha256: "sha256:3da4c4076924f930b5bfcfc76819ebc9a964ada2436a4abafc63652c1568c93e",
        shape: Shape::GzipTar,
        member: "package/bin/opencode.exe",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://registry.npmjs.org/opencode-windows-x64/-/opencode-windows-x64-1.19.0.tgz",
        bytes: 60_241_178,
        sha256: "sha256:1d484abbde6ee3db4b0a34a22e469824410d0e274b92856f428673e7d915adb1",
        shape: Shape::GzipTar,
        member: "package/bin/opencode.exe",
    },
];

/// The artifacts 1.18.34 was published as, kept so
/// `software_update` has a version to move from and `rollback` a tree to
/// return to. Measured from bytes when it was the current pin.
pub(crate) const PREVIOUS_ARTIFACTS: &[Artifact] = &[
    Artifact {
        platform: "linux/arm64",
        url: "https://registry.npmjs.org/opencode-linux-arm64/-/opencode-linux-arm64-1.18.34.tgz",
        bytes: 60_094_599,
        sha256: "sha256:6f212b830b26bf72012f1665559ddb5fd7e928294971d3d1c99a0f9097a3d311",
        shape: Shape::GzipTar,
        member: "package/bin/opencode",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://registry.npmjs.org/opencode-linux-x64/-/opencode-linux-x64-1.18.34.tgz",
        bytes: 60_309_530,
        sha256: "sha256:b83e8ac66d752d05ead4b6a439d3a2cfa32bcd9817c708825a389b5d5cba4f19",
        shape: Shape::GzipTar,
        member: "package/bin/opencode",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://registry.npmjs.org/opencode-darwin-arm64/-/opencode-darwin-arm64-1.18.34.tgz",
        bytes: 45_264_760,
        sha256: "sha256:9a336191ee84c8f54364b5d1990ec87abb2acbb00dea189f11dda5e968d7bf05",
        shape: Shape::GzipTar,
        member: "package/bin/opencode",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://registry.npmjs.org/opencode-darwin-x64/-/opencode-darwin-x64-1.18.34.tgz",
        bytes: 48_879_002,
        sha256: "sha256:bda35c563697ca66b2b5c5b51f6f731376de4b007d9134a52cfe6b8216acba0a",
        shape: Shape::GzipTar,
        member: "package/bin/opencode",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://registry.npmjs.org/opencode-windows-arm64/-/opencode-windows-arm64-1.18.34.tgz",
        bytes: 58_544_117,
        sha256: "sha256:b9dfab4ffcd5df5a9286d613359701dd6c8d3d880f0c426ba7a7c37eb5a7aeeb",
        shape: Shape::GzipTar,
        member: "package/bin/opencode.exe",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://registry.npmjs.org/opencode-windows-x64/-/opencode-windows-x64-1.18.34.tgz",
        bytes: 60_230_073,
        sha256: "sha256:b4f4ae37a9ecbd6eceecf3a452573a5abdeab88ba50f93a9ad40762d7a10bae9",
        shape: Shape::GzipTar,
        member: "package/bin/opencode.exe",
    },
];

/// Opencode's program, and where its bytes come from.
pub(crate) const SOFTWARE: Software = Software {
    version: "1.19.0",
    command: "opencode",
    delivery: Delivery::Artifacts(ARTIFACTS),
    unsupported: &[],
    previous: Some(Previous {
        version: "1.18.34",
        artifacts: PREVIOUS_ARTIFACTS,
    }),
};

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic)]

    // Named rather than glob-imported: a product delivered by a package manager
    // has no `Artifact` in scope, and the test is the same text for all seven.
    use harness_runtime::{Delivery, Shape};

    use super::SOFTWARE;

    fn measured() -> serde_json::Value {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../references/opencode-baseline.json");
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
    }

    #[test]
    fn every_artifact_compiled_in_is_the_one_the_baseline_measured() {
        let block = &measured()["software_artifacts"];
        assert_eq!(block["version"], SOFTWARE.version);
        assert_eq!(block["command"], SOFTWARE.command);

        let Delivery::Artifacts(compiled) = SOFTWARE.delivery else {
            // A product delivered by a package manager has no artifacts, and
            // the baseline must agree that it has none.
            assert_eq!(block["shape"], "manager");
            assert!(block["platforms"].as_object().unwrap().is_empty());
            return;
        };
        let published = block["platforms"].as_object().unwrap();
        assert_eq!(
            compiled.len(),
            published.len(),
            "the table and the baseline disagree on how many platforms exist"
        );
        for artifact in compiled {
            let entry = &published[artifact.platform];
            assert_eq!(entry["url"], artifact.url, "{}", artifact.platform);
            assert_eq!(entry["bytes"], artifact.bytes, "{}", artifact.platform);
            assert_eq!(entry["sha256"], artifact.sha256, "{}", artifact.platform);
            let member = entry.get("member").and_then(serde_json::Value::as_str);
            assert_eq!(
                member.unwrap_or(""),
                artifact.member,
                "{} names a different member",
                artifact.platform
            );
            assert_eq!(
                artifact.shape == Shape::Raw,
                member.is_none(),
                "{} disagrees about whether the bytes are the program",
                artifact.platform
            );
        }
    }

    /// The second pin is the baseline's, or it is absent in both places.
    ///
    /// Asserted from either side rather than only where it exists: a harness
    /// that has never been bumped must compile in `None`, and a build that
    /// dropped the block while the baseline still carried it would otherwise
    /// pass by having nothing to compare.
    #[test]
    fn the_version_this_build_can_move_between_is_the_one_measured_before_it() {
        let baseline = measured();
        let recorded = baseline.get("previous_software_artifacts");
        let Some(earlier) = SOFTWARE.previous else {
            assert!(
                recorded.is_none(),
                "the baseline records a previous release and this build names none"
            );
            return;
        };
        let block = recorded.unwrap_or_else(|| {
            panic!("this build names a previous release the baseline does not record")
        });
        assert_eq!(block["version"], earlier.version);
        assert_ne!(
            earlier.version, SOFTWARE.version,
            "a second pin equal to the first is one version wearing two names"
        );
        let published = block["platforms"].as_object().unwrap();
        assert_eq!(
            earlier.artifacts.len(),
            published.len(),
            "the previous table and the baseline disagree on how many platforms exist"
        );
        for artifact in earlier.artifacts {
            let entry = &published[artifact.platform];
            assert_eq!(entry["url"], artifact.url, "{}", artifact.platform);
            assert_eq!(entry["bytes"], artifact.bytes, "{}", artifact.platform);
            assert_eq!(entry["sha256"], artifact.sha256, "{}", artifact.platform);
        }
    }

    #[test]
    fn a_platform_the_vendor_does_not_publish_is_listed_rather_than_missing() {
        let block = &measured()["software_artifacts"];
        let unpublished: Vec<&str> = block
            .get("unpublished")
            .and_then(serde_json::Value::as_array)
            .map(|entries| {
                entries
                    .iter()
                    .filter_map(serde_json::Value::as_str)
                    .collect()
            })
            .unwrap_or_default();
        assert_eq!(unpublished, SOFTWARE.unsupported);
    }

    #[test]
    fn no_release_calls_a_platform_both_published_and_unpublished() {
        let baseline = measured();
        for name in ["software_artifacts", "previous_software_artifacts"] {
            let Some(block) = baseline.get(name) else {
                continue;
            };
            let published = block["platforms"].as_object().unwrap();
            let unpublished = block
                .get("unpublished")
                .and_then(serde_json::Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(serde_json::Value::as_str);
            for platform in unpublished {
                assert!(
                    !published.contains_key(platform),
                    "{name}: {platform} is both published and unpublished"
                );
            }
        }
    }
}
