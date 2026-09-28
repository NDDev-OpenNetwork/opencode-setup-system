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
        url: "https://registry.npmjs.org/opencode-linux-arm64/-/opencode-linux-arm64-1.18.33.tgz",
        bytes: 60_060_899,
        sha256: "sha256:196d0caf1c0553fcd12ef15ff14439ea7e6bc7a4e87ea5238bcb57bbe6e54b93",
        shape: Shape::GzipTar,
        member: "package/bin/opencode",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://registry.npmjs.org/opencode-linux-x64/-/opencode-linux-x64-1.18.33.tgz",
        bytes: 60_275_143,
        sha256: "sha256:149a676b59224b196626abda2fdbd34c8329ac326b57c4891ba3de446f3ca3c1",
        shape: Shape::GzipTar,
        member: "package/bin/opencode",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://registry.npmjs.org/opencode-darwin-arm64/-/opencode-darwin-arm64-1.18.33.tgz",
        bytes: 46_045_337,
        sha256: "sha256:cd2c704ad653137b62992bca2b32383f8daa31b3c9462e566278e1efba07b31d",
        shape: Shape::GzipTar,
        member: "package/bin/opencode",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://registry.npmjs.org/opencode-darwin-x64/-/opencode-darwin-x64-1.18.33.tgz",
        bytes: 48_221_422,
        sha256: "sha256:e23cf36d4db46214dbf947b7b78bac63d2f0a9e6e987c5d6e47c573d8bb6ff32",
        shape: Shape::GzipTar,
        member: "package/bin/opencode",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://registry.npmjs.org/opencode-windows-arm64/-/opencode-windows-arm64-1.18.33.tgz",
        bytes: 58_509_317,
        sha256: "sha256:20853b3b92e9dc3e9cf9f5867e0f5d0d50506655788e508397e4680271f9ce85",
        shape: Shape::GzipTar,
        member: "package/bin/opencode.exe",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://registry.npmjs.org/opencode-windows-x64/-/opencode-windows-x64-1.18.33.tgz",
        bytes: 60_196_031,
        sha256: "sha256:fc76bd4a0f258027594f6a8900e220fd06f7345cb5e67f73dc3b4f53ba3f580e",
        shape: Shape::GzipTar,
        member: "package/bin/opencode.exe",
    },
];

/// The artifacts 1.18.32 was published as, kept so
/// `software_update` has a version to move from and `rollback` a tree to
/// return to. Measured from bytes when it was the current pin.
pub(crate) const PREVIOUS_ARTIFACTS: &[Artifact] = &[
    Artifact {
        platform: "linux/arm64",
        url: "https://registry.npmjs.org/opencode-linux-arm64/-/opencode-linux-arm64-1.18.32.tgz",
        bytes: 60_032_395,
        sha256: "sha256:29ab2d61a70e99d1224d289115c3b8194ff254968e2609531186227be39b2001",
        shape: Shape::GzipTar,
        member: "package/bin/opencode",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://registry.npmjs.org/opencode-linux-x64/-/opencode-linux-x64-1.18.32.tgz",
        bytes: 60_256_961,
        sha256: "sha256:da0803c85eb86709c4f084adcbfb0b5329936670bfe43d2367f0ca034be0c1de",
        shape: Shape::GzipTar,
        member: "package/bin/opencode",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://registry.npmjs.org/opencode-darwin-arm64/-/opencode-darwin-arm64-1.18.32.tgz",
        bytes: 46_028_592,
        sha256: "sha256:a1707bb6cc9deaccca501c4097f1580b8af70dfc227f194ae0f576f32abbdebe",
        shape: Shape::GzipTar,
        member: "package/bin/opencode",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://registry.npmjs.org/opencode-darwin-x64/-/opencode-darwin-x64-1.18.32.tgz",
        bytes: 48_203_499,
        sha256: "sha256:e9c0cd81f873cb53b21394461f2b7119bd750329df7221d19642546d3b581386",
        shape: Shape::GzipTar,
        member: "package/bin/opencode",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://registry.npmjs.org/opencode-windows-arm64/-/opencode-windows-arm64-1.18.32.tgz",
        bytes: 58_487_775,
        sha256: "sha256:99ab07bf4e4309fa46d0e44a53a4bfffb72bde3e6be819d62cd2da106da5f625",
        shape: Shape::GzipTar,
        member: "package/bin/opencode.exe",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://registry.npmjs.org/opencode-windows-x64/-/opencode-windows-x64-1.18.32.tgz",
        bytes: 60_169_333,
        sha256: "sha256:701f23388207a4d2e2ff46cda46e707474fa01cf2a1a1afa404f045c06e5eaa4",
        shape: Shape::GzipTar,
        member: "package/bin/opencode.exe",
    },
];

/// Opencode's program, and where its bytes come from.
pub(crate) const SOFTWARE: Software = Software {
    version: "1.18.33",
    command: "opencode",
    delivery: Delivery::Artifacts(ARTIFACTS),
    unsupported: &[],
    previous: Some(Previous {
        version: "1.18.32",
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
