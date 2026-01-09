# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
## [1.0.0] (2026-01-09)

### Features

* Unified versioning across all components (Rust, NPM, VSCode)
* Production release v1.0.0


## [0.4.0](https://github.com/skew202/antislop/compare/v0.3.0...v0.4.0) (2026-01-08)


### Features

* Enhance CLI output, add hygiene survey, and improve filename checks ([69f71a3](https://github.com/skew202/antislop/commit/69f71a3b64125caccbcbd3993873cd4abcabcb36))
* reorganize marketing assets and update docs ([268ca2b](https://github.com/skew202/antislop/commit/268ca2b101281ff46bd1b047866da87ce32475a5))


### Bug Fixes

* **ci:** allow npm publish to proceed even if crates.io version exists ([ffa065f](https://github.com/skew202/antislop/commit/ffa065f6a48c5351b46a127e42fa80f13b6677e1))
* **ci:** resolve cargo-deny and docker build failures ([4d0b44e](https://github.com/skew202/antislop/commit/4d0b44e617904f46068cb9400f402100e3bd0eb5))
* **docker:** add data directory to Dockerfile COPY for hygiene_tools.toml ([b5d56b3](https://github.com/skew202/antislop/commit/b5d56b30a1e1cd28c79986338edf431b5595f903))
* **docker:** use rust:1.85 for edition2024 support (globset dependency) ([e9ad2ea](https://github.com/skew202/antislop/commit/e9ad2ea89ffd01b8240e370e7ad4e33af1144b5b))


### Documentation

* overhaul design to 'Information Architect' style ([bfa521a](https://github.com/skew202/antislop/commit/bfa521a7144e6b3b6a480047dfc6bb4c56318c66))


### Miscellaneous

* **npm:** bump package version to 0.3.0 ([ae082c1](https://github.com/skew202/antislop/commit/ae082c1ab46896011936bc9cdb37730a6e356268))


### CI/CD

* bump actions/cache from 4 to 5 ([7ec5933](https://github.com/skew202/antislop/commit/7ec5933921d18a2cacbf3993d088ca7952dfd13e))
* bump actions/setup-node from 4 to 6 ([472be62](https://github.com/skew202/antislop/commit/472be622e1fb1f8ba197a630880e1d6e1cae1c18))
* bump actions/setup-node from 4 to 6 ([2872eb4](https://github.com/skew202/antislop/commit/2872eb451f639977269c96ada243ce947fb8bf7a))

## [0.3.0](https://github.com/skew202/antislop/compare/v0.2.1...v0.3.0) (2026-01-07)


### Features

* add filename convention checking for AI-generated duplicates ([9f6d2e0](https://github.com/skew202/antislop/commit/9f6d2e04aeff886b708b145fb4b0d2c26050cd69))
* v0.3.0 Unix Philosophy Redesign ([aaee8c3](https://github.com/skew202/antislop/commit/aaee8c38672fb1192da5d89ff9fee47989f81072))


### Bug Fixes

* **ci:** exclude source files with intentional pattern docs from antislop scan ([aba8487](https://github.com/skew202/antislop/commit/aba84873a879a94544b1483e9b4c1e72b683646b))
* **ci:** Resolve CI failures and test suite issues ([931c37c](https://github.com/skew202/antislop/commit/931c37c0acdb06892192f49bb0532b8c1a445118))
* reduce keywords to 5 for crates.io limit ([e356186](https://github.com/skew202/antislop/commit/e356186a372fa70add21bbe27f67471bcbe66b4f))


### Documentation

* improve license section in README ([ee9f21a](https://github.com/skew202/antislop/commit/ee9f21ae9c85684bd72acd5368bb100f353bfbd3))
* Update README with profile philosophy and CI examples ([1aaaf3e](https://github.com/skew202/antislop/commit/1aaaf3e8771540e181d69e53f31478f27f4d0786))


### Tests

* add 9 CLI output tests to improve mutation score ([8ba26f1](https://github.com/skew202/antislop/commit/8ba26f13920be863f04636c2742e9ca044ca2405))
* add hedging patterns and reorganize snapshot tests ([bf8a91f](https://github.com/skew202/antislop/commit/bf8a91f70558641f680f55faac719d1461225bfa))
* improve coverage from 52% to 89% ([7b8da59](https://github.com/skew202/antislop/commit/7b8da59a36e30e57bdeaee6932231f517fce5c87))


### Miscellaneous

* bump npm package version to 0.2.1 ([7d009f4](https://github.com/skew202/antislop/commit/7d009f4dbd3239b92beabd2e76971b1b6e515f90))
* **deps:** update all cargo dependencies ([cf50817](https://github.com/skew202/antislop/commit/cf5081720e25dd50e5a5f3e8af39507427556762))
* ignore cargo-mutants output directories ([d0e1d31](https://github.com/skew202/antislop/commit/d0e1d31db40ee44f7de96a5ad5fb04ca8b667aec))
* integrate antislop into CI and pre-commit hooks ([b099754](https://github.com/skew202/antislop/commit/b09975473877027de7b11e663443c84880a74f20))
* release v0.2.2 ([f8ed149](https://github.com/skew202/antislop/commit/f8ed14906761cc0751f4c916d2ddb1966706a050))
* remove deprecated 'command' parameter from release-please ([ee9f21a](https://github.com/skew202/antislop/commit/ee9f21ae9c85684bd72acd5368bb100f353bfbd3))
* update actions/checkout v4-&gt;v6, @types/vscode 1.75-&gt;1.107 ([ae37f47](https://github.com/skew202/antislop/commit/ae37f4710db7d39bc94e656c306e7eb6376557e9))


### CI/CD

* add publish workflow for manual and auto-trigger ([a30cd12](https://github.com/skew202/antislop/commit/a30cd12712314632bc0cbe8d287dbe980a47e072))
* remove deprecated cache option from docker build ([7007813](https://github.com/skew202/antislop/commit/7007813cf06d4682f40bce7c584ad75f188e949b))

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
