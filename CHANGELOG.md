# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.0 (2026-01-01)


### Features

* add editor integrations and npm/LSP packages ([70c5bb7](https://github.com/skew202/antislop/commit/70c5bb773eda6e3d968a340f4d34d2444124a1ff))
* add npm publishing workflow ([a887c96](https://github.com/skew202/antislop/commit/a887c9640624078faab2e83623318e5c4712594f))
* add optional tree-sitter support for AST-level slop detection ([b96e6dc](https://github.com/skew202/antislop/commit/b96e6dc917b9c74d4191f6a6f3e72448015556cd))
* add release-please automation ([e48cc28](https://github.com/skew202/antislop/commit/e48cc28f15a9a7e59b86d5e368970012f11d56a4))
* **hygiene:** add MECE verification scripts for linter overlap testing ([47daf56](https://github.com/skew202/antislop/commit/47daf5610f64311caaa5df507c2026e4de45ac2c))
* restructure config into category-based pattern files ([b59e25f](https://github.com/skew202/antislop/commit/b59e25f53c4b7330120ce37c4246fa661f524fd8))


### Bug Fixes

* add allow(dead_code) to fix Clippy CI failure ([d224271](https://github.com/skew202/antislop/commit/d22427184701c1c99200df3e486fa7eb49db19ae))
* cargo-deny config and code formatting ([37bfb69](https://github.com/skew202/antislop/commit/37bfb69e9b9e84adec080d9e85bfe5d2f7bbbab9))
* ci failures ([979c119](https://github.com/skew202/antislop/commit/979c119793ae0f54a3b910d351e6033254ed0ff5))
* CI failures - Docker build and release-please config ([cc197c0](https://github.com/skew202/antislop/commit/cc197c0fde57a4737de454ae10308fcf5d10c2f8))
* container scan job ([7c31539](https://github.com/skew202/antislop/commit/7c315396a7cf2f5808aaada9f572fb5d49191fb5))
* remove non-existent workspace.dependencies xpath ([5537f4e](https://github.com/skew202/antislop/commit/5537f4e38b7d26b1696c65854e8a8715dac668d3))
* remove unsupported 'markdown' type from release-please config ([ed7d276](https://github.com/skew202/antislop/commit/ed7d27669693eb48865e49ec6e11a11a85ec9fb0))
* remove unsupported 'text' type from release-please config ([ba9da7e](https://github.com/skew202/antislop/commit/ba9da7e11110381fe0831a281ce32f754efee018))
* simplify dist workflow and dockerfile ([dda56e6](https://github.com/skew202/antislop/commit/dda56e6fc8e77ec0a5c6cb73aa8aa7c88e68d703))
* simplify release-please config with per-package release-types ([50261f4](https://github.com/skew202/antislop/commit/50261f4c53e7462e1ed765d837c1ebdd174d2c67))
* update CI/CD workflows and fix build issues ([e5d5635](https://github.com/skew202/antislop/commit/e5d5635e05b4df6eacc7a810c1bd2515e3510ee4))
* update dockerfile and dist workflow ([75e7042](https://github.com/skew202/antislop/commit/75e7042dd6306b72aab4fac43f6bf23a16ed1a00))
* update serde-sarif and allow duplicate transitive deps ([64c4d00](https://github.com/skew202/antislop/commit/64c4d00e0b03098542dea587d7d644e3356ec166))
* update serde-sarif to 0.8 and cleanup dependencies ([b600252](https://github.com/skew202/antislop/commit/b6002521faf4b0a1d1895ab08b797738a1a63090))
* use 'manifest' command for release-please ([5eabffe](https://github.com/skew202/antislop/commit/5eabffeee11dafad571b9dd5834b57d552b097c3))
* windows integration tests ([6a3cf3d](https://github.com/skew202/antislop/commit/6a3cf3d9c3a1c8254e0ec32d5a7536118f1ac469))


### Refactor

* **detector:** enhance tree-sitter integration and report module ([dd506b7](https://github.com/skew202/antislop/commit/dd506b7130d7caeb02bd9d60d2a6233f01884331))
* **examples:** update sloppy code examples with AI shortcut patterns ([0743770](https://github.com/skew202/antislop/commit/07437707ef2a0f257c6167b3a4e729e687749531))


### Documentation

* document MECE pattern hygiene strategy across all guides ([c4fd42d](https://github.com/skew202/antislop/commit/c4fd42d33284f95e99cc3c1c623e45b9f2e2da37))
* **qa:** add comprehensive QA strategy with MECE pattern hygiene ([8e786ad](https://github.com/skew202/antislop/commit/8e786ad3b34226a87c358e793437c6a75f54b76d))


### Tests

* add property-based, snapshot, and integration tests ([bb9a1eb](https://github.com/skew202/antislop/commit/bb9a1eb15e70078a0d09978b6fb446d7b35a0081))
* **bench:** add comprehensive multi-language benchmark suite ([8c24674](https://github.com/skew202/antislop/commit/8c24674fb0787c7732ec755ead2a44d20cc3c1ab))
* **fuzz:** add cargo-fuzz targets for security hardening ([beac862](https://github.com/skew202/antislop/commit/beac86219bd0526587dac4666bb564d0e7adfe4d))


### Miscellaneous

* add changelog.md to release-please config ([c902cfa](https://github.com/skew202/antislop/commit/c902cfabeab9f05035f47bf354f0770c76be60b7))
* enhance release-please config ([544c946](https://github.com/skew202/antislop/commit/544c94621b527e835387c34691bbbc2982b22965))
* optimize docker build context ([7333679](https://github.com/skew202/antislop/commit/7333679102c0e7fcc0e8810c13c195afaa6405df))
* release v0.2.0 ([a6b4dd4](https://github.com/skew202/antislop/commit/a6b4dd4debf5c3b4d882ec7feccfeecf078f2881))
* remove CODE_OF_CONDUCT.md (use GitHub default) ([4c2d3fd](https://github.com/skew202/antislop/commit/4c2d3fd2be52925011e2acd572b7b488193cb468))
* **scripts:** add QA and coverage helper scripts ([8f634db](https://github.com/skew202/antislop/commit/8f634db7fa38e8109805503b4df1f4804ee86ce8))
* update dependencies and config files ([f62d12a](https://github.com/skew202/antislop/commit/f62d12a3e935e3bb40721603a7c4de04aaab48b5))
* **vscode-dev:** bump @types/node in /editors/vscode ([#6](https://github.com/skew202/antislop/issues/6)) ([51254e9](https://github.com/skew202/antislop/commit/51254e9ed065396b9b00e08bccc5ad3e8af77cbc))
* **vscode-dev:** bump typescript in /editors/vscode ([#3](https://github.com/skew202/antislop/issues/3)) ([efb470d](https://github.com/skew202/antislop/commit/efb470d81625478eab92e83d1dfd7c277dbcd879))
* **vscode:** bump vscode-languageclient in /editors/vscode ([#9](https://github.com/skew202/antislop/issues/9)) ([fad4030](https://github.com/skew202/antislop/commit/fad4030d0b4518da882caaac3a98a6dd80de551d))


### CI/CD

* add GitHub Action, Docker, and release workflow ([6ef2b06](https://github.com/skew202/antislop/commit/6ef2b06b8a4e1f241c84ac84e0a371302a90e495))
* bump actions/configure-pages from 4 to 5 ([#7](https://github.com/skew202/antislop/issues/7)) ([7a2efdd](https://github.com/skew202/antislop/commit/7a2efdd29d186bf795d931cf84b5f1d9cf60f2a7))
* bump actions/upload-pages-artifact from 3 to 4 ([#5](https://github.com/skew202/antislop/issues/5)) ([535e4aa](https://github.com/skew202/antislop/commit/535e4aaeb0eb30593e39a8d595417f3d60a075f0))
* bump aquasecurity/trivy-action from 0.29.0 to 0.33.1 ([#4](https://github.com/skew202/antislop/issues/4)) ([9bbf6c3](https://github.com/skew202/antislop/commit/9bbf6c3324448602039591a7e196089519d1bdc5))
* bump peter-evans/create-pull-request from 7 to 8 ([#1](https://github.com/skew202/antislop/issues/1)) ([10d748d](https://github.com/skew202/antislop/commit/10d748d10f5c70ead41afed38ba04936922efad0))
* **hooks:** add pre-commit configuration for local linting ([62a0154](https://github.com/skew202/antislop/commit/62a01546d9f75ab737d6a1ffb076b83cff818847))
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
