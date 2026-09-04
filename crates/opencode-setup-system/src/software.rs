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
        url: "https://registry.npmjs.org/opencode-linux-arm64/-/opencode-linux-arm64-1.18.27.tgz",
        bytes: 59_945_385,
        sha256: "sha256:83bf3812ecad71b3a463c5c0a7ceb0dba9db96964f3e7f8ba6bf30ca138287e8",
        shape: Shape::GzipTar,
        member: "package/bin/opencode",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://registry.npmjs.org/opencode-linux-x64/-/opencode-linux-x64-1.18.27.tgz",
        bytes: 60_168_253,
        sha256: "sha256:0aba86ba404f52e57bd154ec3565cd3e86d344743bf32e3004bf7fdbd3363ac4",
        shape: Shape::GzipTar,
        member: "package/bin/opencode",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://registry.npmjs.org/opencode-darwin-arm64/-/opencode-darwin-arm64-1.18.27.tgz",
        bytes: 45_940_410,
        sha256: "sha256:dba942c12128491b7c00f5d4b395bb8d36061f293b59db501ca9b0911a701680",
        shape: Shape::GzipTar,
        member: "package/bin/opencode",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://registry.npmjs.org/opencode-darwin-x64/-/opencode-darwin-x64-1.18.27.tgz",
        bytes: 48_115_145,
        sha256: "sha256:8e379467c2f911d5a6bb14a453b8f760e093daf8c5c6b9ee1da8f3515477e8f2",
        shape: Shape::GzipTar,
        member: "package/bin/opencode",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://registry.npmjs.org/opencode-windows-arm64/-/opencode-windows-arm64-1.18.27.tgz",
        bytes: 58_397_893,
        sha256: "sha256:3da5a83466c814922fc1472ef4eef1c37cae990a1cfe1530959c83d3f5b13cda",
        shape: Shape::GzipTar,
        member: "package/bin/opencode.exe",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://registry.npmjs.org/opencode-windows-x64/-/opencode-windows-x64-1.18.27.tgz",
        bytes: 60_079_608,
        sha256: "sha256:d940ca3115e9a87107bb666c30c3efea88bcad1c2d34212c8deb4401a3054792",
        shape: Shape::GzipTar,
        member: "package/bin/opencode.exe",
    },
];

/// The artifacts 1.18.26 was published as, kept so
/// `software_update` has a version to move from and `rollback` a tree to
/// return to. Measured from bytes when it was the current pin.
pub(crate) const PREVIOUS_ARTIFACTS: &[Artifact] = &[
    Artifact {
        platform: "linux/arm64",
        url: "https://registry.npmjs.org/opencode-linux-arm64/-/opencode-linux-arm64-1.18.26.tgz",
        bytes: 59_947_971,
        sha256: "sha256:5e0cc6c6c48d6629c8f5d3d5c9f9670e8dac7ba14d295801bb3f6a783a8f841b",
        shape: Shape::GzipTar,
        member: "package/bin/opencode",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://registry.npmjs.org/opencode-linux-x64/-/opencode-linux-x64-1.18.26.tgz",
        bytes: 60_169_535,
        sha256: "sha256:990d8b07111517a78ba779709ff8f438e0dcf2a7fb66d36df7507c8e93358f02",
        shape: Shape::GzipTar,
        member: "package/bin/opencode",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://registry.npmjs.org/opencode-darwin-arm64/-/opencode-darwin-arm64-1.18.26.tgz",
        bytes: 45_942_652,
        sha256: "sha256:d9c09ba039dd62f983fc66c65777910f20eead2c4e30cbff888f26d640607e15",
        shape: Shape::GzipTar,
        member: "package/bin/opencode",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://registry.npmjs.org/opencode-darwin-x64/-/opencode-darwin-x64-1.18.26.tgz",
        bytes: 48_118_308,
        sha256: "sha256:dff2571b3ad3f04dff7f0555bf4e679615c1f70afb35258f139d22a491da57e3",
        shape: Shape::GzipTar,
        member: "package/bin/opencode",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://registry.npmjs.org/opencode-windows-arm64/-/opencode-windows-arm64-1.18.26.tgz",
        bytes: 58_398_040,
        sha256: "sha256:419799338b25d5e62a393136c61166ddf0e78229b784daf0a9fabfb0df66eb9f",
        shape: Shape::GzipTar,
        member: "package/bin/opencode.exe",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://registry.npmjs.org/opencode-windows-x64/-/opencode-windows-x64-1.18.26.tgz",
        bytes: 60_082_922,
        sha256: "sha256:fca4106836f9ca9d9485d010a247d0d928eecfff972b9019ff522b6ba9885934",
        shape: Shape::GzipTar,
        member: "package/bin/opencode.exe",
    },
];

/// Opencode's program, and where its bytes come from.
pub(crate) const SOFTWARE: Software = Software {
    version: "1.18.27",
    command: "opencode",
    delivery: Delivery::Artifacts(ARTIFACTS),
    unsupported: &[],
    previous: Some(Previous {
        version: "1.18.26",
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
