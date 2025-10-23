set dotenv-load

default:
    @just --list

fmt:
    cargo fmt --all

[group('ci')]
[parallel]
ci: lint test doctest doc

[group('ci')]
[parallel]
lint:
    pre-commit run --all-files
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
bump BUMP_TYPE='':
    #!/usr/bin/env bash
    set -euo pipefail
    if [ -z "{{BUMP_TYPE}}" ] || ! [[ "{{BUMP_TYPE}}" =~ ^(major|minor|patch)$ ]]; then
      cargo release changes
      echo "Re-run and specify 'major', 'minor', or 'patch'."
      exit 1
    fi
    cargo release version "{{BUMP_TYPE}}" --execute --no-confirm
    remote_url=$(git config --get remote.origin.url | sed 's/\.git$//')
    git cliff --config ./.cliff.toml --unreleased --bump="{{BUMP_TYPE}}" --context |
      jq --arg u "${remote_url}" '.[0].extra.remote_url= $u' |
      git cliff --config ./.cliff.toml --unreleased --bump="{{BUMP_TYPE}}" --prepend CHANGELOG.md --from-context -

[group('release')]
tag:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo release commit --execute --no-confirm
    cargo release tag --execute --no-confirm
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
    ASSETS=("${TMPDIR}/${ASSET_NAME}.tar.gz" "${TMPDIR}/${ASSET_NAME}.tar.gz.sha256")
    gh release create "v${PACKAGE_VERSION}" \
      "${ASSETS[@]}" \
      --notes "$(awk '/^## \[/{if(found) exit; found=1} found' CHANGELOG.md)"

[group('release')]
release: build tag upload
