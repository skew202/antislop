# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1](https://github.com/skew202/antislop/compare/v0.2.0...v0.2.1) (2026-01-01)


### Bug Fixes

* add Unicode-3.0 license to allow list ([79baa13](https://github.com/skew202/antislop/commit/79baa13953c64b621f47eb5012b4dd4728913180))
* cargo-deny config and code formatting ([37bfb69](https://github.com/skew202/antislop/commit/37bfb69e9b9e84adec080d9e85bfe5d2f7bbbab9))
* ci failures ([979c119](https://github.com/skew202/antislop/commit/979c119793ae0f54a3b910d351e6033254ed0ff5))
* CI failures - Docker build and release-please config ([cc197c0](https://github.com/skew202/antislop/commit/cc197c0fde57a4737de454ae10308fcf5d10c2f8))
* container scan job ([7c31539](https://github.com/skew202/antislop/commit/7c315396a7cf2f5808aaada9f572fb5d49191fb5))
* remove non-existent workspace.dependencies xpath ([5537f4e](https://github.com/skew202/antislop/commit/5537f4e38b7d26b1696c65854e8a8715dac668d3))
* remove unsupported 'markdown' type from release-please config ([ed7d276](https://github.com/skew202/antislop/commit/ed7d27669693eb48865e49ec6e11a11a85ec9fb0))
* remove unsupported 'text' type from release-please config ([ba9da7e](https://github.com/skew202/antislop/commit/ba9da7e11110381fe0831a281ce32f754efee018))
* remove unused licenses and fix skip list in deny.toml ([5ee99bc](https://github.com/skew202/antislop/commit/5ee99bc310114f3a8b621b4c90073e7afea048a7))
* remove unused licenses and unnecessary skips from deny.toml ([dd7a7c1](https://github.com/skew202/antislop/commit/dd7a7c1028b37f24723b9f8622258031e145ec94))
* replace criterion::black_box with std::hint::black_box ([325dc5d](https://github.com/skew202/antislop/commit/325dc5d5c9f58d3ea532a8f6cd57bdf3a0cb87ff))
* simplify release-please config with per-package release-types ([50261f4](https://github.com/skew202/antislop/commit/50261f4c53e7462e1ed765d837c1ebdd174d2c67))
* update cargo-deny config to modern format ([4f52e6d](https://github.com/skew202/antislop/commit/4f52e6dad2eea087b74bbaf39c86298d8ead8e22))
* update serde-sarif and allow duplicate transitive deps ([64c4d00](https://github.com/skew202/antislop/commit/64c4d00e0b03098542dea587d7d644e3356ec166))
* update serde-sarif to 0.8 and cleanup dependencies ([b600252](https://github.com/skew202/antislop/commit/b6002521faf4b0a1d1895ab08b797738a1a63090))
* use 'manifest' command for release-please ([5eabffe](https://github.com/skew202/antislop/commit/5eabffeee11dafad571b9dd5834b57d552b097c3))
* windows integration tests ([6a3cf3d](https://github.com/skew202/antislop/commit/6a3cf3d9c3a1c8254e0ec32d5a7536118f1ac469))


### Miscellaneous

* add changelog.md to release-please config ([c902cfa](https://github.com/skew202/antislop/commit/c902cfabeab9f05035f47bf354f0770c76be60b7))
* enhance release-please config ([544c946](https://github.com/skew202/antislop/commit/544c94621b527e835387c34691bbbc2982b22965))
* initialize release-please manifest with current version 0.2.0 ([516e870](https://github.com/skew202/antislop/commit/516e870bdfdff99718c64214e8aafeeb5ee210c9))
* optimize docker build context ([7333679](https://github.com/skew202/antislop/commit/7333679102c0e7fcc0e8810c13c195afaa6405df))
* **vscode-dev:** bump @types/node in /editors/vscode ([#6](https://github.com/skew202/antislop/issues/6)) ([51254e9](https://github.com/skew202/antislop/commit/51254e9ed065396b9b00e08bccc5ad3e8af77cbc))
* **vscode-dev:** bump typescript in /editors/vscode ([#3](https://github.com/skew202/antislop/issues/3)) ([efb470d](https://github.com/skew202/antislop/commit/efb470d81625478eab92e83d1dfd7c277dbcd879))
* **vscode:** bump vscode-languageclient in /editors/vscode ([#9](https://github.com/skew202/antislop/issues/9)) ([fad4030](https://github.com/skew202/antislop/commit/fad4030d0b4518da882caaac3a98a6dd80de551d))


### CI/CD

* bump actions/configure-pages from 4 to 5 ([#7](https://github.com/skew202/antislop/issues/7)) ([7a2efdd](https://github.com/skew202/antislop/commit/7a2efdd29d186bf795d931cf84b5f1d9cf60f2a7))
* bump actions/upload-pages-artifact from 3 to 4 ([#5](https://github.com/skew202/antislop/issues/5)) ([535e4aa](https://github.com/skew202/antislop/commit/535e4aaeb0eb30593e39a8d595417f3d60a075f0))
* bump aquasecurity/trivy-action from 0.29.0 to 0.33.1 ([#4](https://github.com/skew202/antislop/issues/4)) ([9bbf6c3](https://github.com/skew202/antislop/commit/9bbf6c3324448602039591a7e196089519d1bdc5))
* bump peter-evans/create-pull-request from 7 to 8 ([#1](https://github.com/skew202/antislop/issues/1)) ([10d748d](https://github.com/skew202/antislop/commit/10d748d10f5c70ead41afed38ba04936922efad0))
* remove aarch64-linux build from dist matrix (requires cross-toolchain) ([8c495de](https://github.com/skew202/antislop/commit/8c495dec27eae505528db7195ae26f78dbeee782))

## [Unreleased]

### Added
- Initial release
- Multi-language comment detection (Python, JS, TypeScript, Rust, Go, Java, C/C++, and more)
- Configurable slop patterns via TOML
- JSON and human-readable output formats
- Parallel file scanning with gitignore support
- Tree-sitter integration for accurate comment extraction
- Shell completion support
