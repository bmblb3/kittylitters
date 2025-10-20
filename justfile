default:
    @just --list

[group('ci')]
[parallel]
ci: lint test doctest doc

[group('ci')]
[parallel]
lint:
    cargo fmt --all --check
    cargo check --all-targets --all-features --workspace
    cargo clippy --all-targets --all-features --workspace -- -D warnings -D clippy::all

[group('ci')]
test:
    cargo test --all-features --all-targets --workspace

[group('ci')]
doctest:
    cargo test --doc

[group('ci')]
doc:
    cargo doc --no-deps --document-private-items --all-features --workspace --examples

[group('build')]
build:
    nix build

[group('pre-release')]
bump:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo release changes
    read -p "Bump type (major, minor, patch): " BUMP_TYPE
    cargo release version "${BUMP_TYPE}" --execute
    git cliff --config ./.cliff.toml --unreleased --bump="${BUMP_TYPE}" --prepend CHANGELOG.md

[group('release')]
tag:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo release commit --execute
    cargo release tag --execute
    cargo release push --execute

[group('release')]
upload:
    #!/usr/bin/env bash
    set -euo pipefail
    PACKAGE_NAME=$(cargo metadata --no-deps --format-version=1 | jq -r '.packages[0].name')
    PACKAGE_VERSION=$(cargo metadata --no-deps --format-version=1 | jq -r '.packages[0].version')
    ASSET_NAME="${PACKAGE_NAME}-v${PACKAGE_VERSION}-x86_64-unknown-linux-musl"
    TMPDIR=$(mktemp -d)
    trap "rm -rf ${TMPDIR}" EXIT
    tar -czf "${TMPDIR}/${ASSET_NAME}.tar.gz" -C ./result/bin .
    (cd "${TMPDIR}" && sha256sum "${ASSET_NAME}.tar.gz" > "${ASSET_NAME}.tar.gz.sha256")
    gh release create "v${PACKAGE_VERSION}" "${TMPDIR}/${ASSET_NAME}.tar.gz" "${TMPDIR}/${ASSET_NAME}.tar.gz.sha256" --notes "$(awk '/^## \[/{if(found) exit; found=1} found' CHANGELOG.md)"

[group('release')]
release: build tag upload
