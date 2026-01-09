# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0](https://github.com/skew202/antislop/compare/v0.4.0...v0.5.0) (2026-01-09)


### Features

* add editor integrations and npm/LSP packages ([70c5bb7](https://github.com/skew202/antislop/commit/70c5bb773eda6e3d968a340f4d34d2444124a1ff))
* add filename convention checking for AI-generated duplicates ([99ba373](https://github.com/skew202/antislop/commit/99ba37361949756259c3fd91221d69c57eca2269))
* add npm publishing workflow ([a887c96](https://github.com/skew202/antislop/commit/a887c9640624078faab2e83623318e5c4712594f))
* add optional tree-sitter support for AST-level slop detection ([b96e6dc](https://github.com/skew202/antislop/commit/b96e6dc917b9c74d4191f6a6f3e72448015556cd))
* add release-please automation ([e48cc28](https://github.com/skew202/antislop/commit/e48cc28f15a9a7e59b86d5e368970012f11d56a4))
* Enhance CLI output, add hygiene survey, and improve filename checks ([29aa07b](https://github.com/skew202/antislop/commit/29aa07b35354e72ca3739cb2ceb08b12f3695bbe))
* **hygiene:** add MECE verification scripts for linter overlap testing ([47daf56](https://github.com/skew202/antislop/commit/47daf5610f64311caaa5df507c2026e4de45ac2c))
* reorganize marketing assets and update docs ([45de0fc](https://github.com/skew202/antislop/commit/45de0fc2dad6a82a7c1353d96358473e11771099))
* restructure config into category-based pattern files ([b59e25f](https://github.com/skew202/antislop/commit/b59e25f53c4b7330120ce37c4246fa661f524fd8))
* v0.3.0 Unix Philosophy Redesign ([d0c8550](https://github.com/skew202/antislop/commit/d0c85502db3542c153f7b03cb139618892c318f3))


### Bug Fixes

* add allow(dead_code) to fix Clippy CI failure ([d224271](https://github.com/skew202/antislop/commit/d22427184701c1c99200df3e486fa7eb49db19ae))
* add Unicode-3.0 license to allow list ([79baa13](https://github.com/skew202/antislop/commit/79baa13953c64b621f47eb5012b4dd4728913180))
* cargo-deny config and code formatting ([37bfb69](https://github.com/skew202/antislop/commit/37bfb69e9b9e84adec080d9e85bfe5d2f7bbbab9))
* ci failures ([979c119](https://github.com/skew202/antislop/commit/979c119793ae0f54a3b910d351e6033254ed0ff5))
* CI failures - Docker build and release-please config ([cc197c0](https://github.com/skew202/antislop/commit/cc197c0fde57a4737de454ae10308fcf5d10c2f8))
* **ci:** allow npm publish to proceed even if crates.io version exists ([1bc162e](https://github.com/skew202/antislop/commit/1bc162eb7ea34f9a56d821e7353301c8e7291f8f))
* **ci:** exclude source files with intentional pattern docs from antislop scan ([9778ce4](https://github.com/skew202/antislop/commit/9778ce4a6abaa350a7892e3cda039dbda3e66509))
* **ci:** resolve cargo-deny and docker build failures ([afc928e](https://github.com/skew202/antislop/commit/afc928e831cd6f3d03b646789830c8e380f43cfe))
* **ci:** Resolve CI failures and test suite issues ([f665cf5](https://github.com/skew202/antislop/commit/f665cf59c172e734bb77a06a8599739829e0f86f))
* container scan job ([7c31539](https://github.com/skew202/antislop/commit/7c315396a7cf2f5808aaada9f572fb5d49191fb5))
* **docker:** add data directory to Dockerfile COPY for hygiene_tools.toml ([a262aae](https://github.com/skew202/antislop/commit/a262aae97716269239989167c55861c5fb309711))
* **docker:** use rust:1.85 for edition2024 support (globset dependency) ([bc467bd](https://github.com/skew202/antislop/commit/bc467bd0e1913c8898a0f9143dabe8a0126f949c))
* reduce keywords to 5 for crates.io limit ([e356186](https://github.com/skew202/antislop/commit/e356186a372fa70add21bbe27f67471bcbe66b4f))
* remove non-existent workspace.dependencies xpath ([5537f4e](https://github.com/skew202/antislop/commit/5537f4e38b7d26b1696c65854e8a8715dac668d3))
* remove unsupported 'markdown' type from release-please config ([ed7d276](https://github.com/skew202/antislop/commit/ed7d27669693eb48865e49ec6e11a11a85ec9fb0))
* remove unsupported 'text' type from release-please config ([ba9da7e](https://github.com/skew202/antislop/commit/ba9da7e11110381fe0831a281ce32f754efee018))
* remove unused licenses and fix skip list in deny.toml ([5ee99bc](https://github.com/skew202/antislop/commit/5ee99bc310114f3a8b621b4c90073e7afea048a7))
* remove unused licenses and unnecessary skips from deny.toml ([dd7a7c1](https://github.com/skew202/antislop/commit/dd7a7c1028b37f24723b9f8622258031e145ec94))
* replace criterion::black_box with std::hint::black_box ([325dc5d](https://github.com/skew202/antislop/commit/325dc5d5c9f58d3ea532a8f6cd57bdf3a0cb87ff))
* simplify dist workflow and dockerfile ([dda56e6](https://github.com/skew202/antislop/commit/dda56e6fc8e77ec0a5c6cb73aa8aa7c88e68d703))
* simplify release-please config with per-package release-types ([50261f4](https://github.com/skew202/antislop/commit/50261f4c53e7462e1ed765d837c1ebdd174d2c67))
* update cargo-deny config to modern format ([4f52e6d](https://github.com/skew202/antislop/commit/4f52e6dad2eea087b74bbaf39c86298d8ead8e22))
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
* improve license section in README ([ee9f21a](https://github.com/skew202/antislop/commit/ee9f21ae9c85684bd72acd5368bb100f353bfbd3))
* overhaul design to 'Information Architect' style ([93a2be0](https://github.com/skew202/antislop/commit/93a2be09781307ce0a905be3581a7d55d24e291f))
* **qa:** add comprehensive QA strategy with MECE pattern hygiene ([8e786ad](https://github.com/skew202/antislop/commit/8e786ad3b34226a87c358e793437c6a75f54b76d))
* Update README with profile philosophy and CI examples ([91c1bf7](https://github.com/skew202/antislop/commit/91c1bf74c14eda649cb771f87a3743e470714f12))


### Tests

* add 9 CLI output tests to improve mutation score ([d5bf6a4](https://github.com/skew202/antislop/commit/d5bf6a4c6f98ec12cff50878ab930b77ac592dbd))
* add hedging patterns and reorganize snapshot tests ([a7fa2a8](https://github.com/skew202/antislop/commit/a7fa2a8ab7a918e36434b78ab2fa3f7e705d13e1))
* add property-based, snapshot, and integration tests ([bb9a1eb](https://github.com/skew202/antislop/commit/bb9a1eb15e70078a0d09978b6fb446d7b35a0081))
* **bench:** add comprehensive multi-language benchmark suite ([8c24674](https://github.com/skew202/antislop/commit/8c24674fb0787c7732ec755ead2a44d20cc3c1ab))
* **fuzz:** add cargo-fuzz targets for security hardening ([beac862](https://github.com/skew202/antislop/commit/beac86219bd0526587dac4666bb564d0e7adfe4d))
* improve coverage from 52% to 89% ([7b8da59](https://github.com/skew202/antislop/commit/7b8da59a36e30e57bdeaee6932231f517fce5c87))


### Miscellaneous

* add changelog.md to release-please config ([c902cfa](https://github.com/skew202/antislop/commit/c902cfabeab9f05035f47bf354f0770c76be60b7))
* bump npm package version to 0.2.1 ([7d009f4](https://github.com/skew202/antislop/commit/7d009f4dbd3239b92beabd2e76971b1b6e515f90))
* **deps:** update all cargo dependencies ([8c93ac5](https://github.com/skew202/antislop/commit/8c93ac526dba3c9e48571c43844fb79cf75660d4))
* enhance release-please config ([544c946](https://github.com/skew202/antislop/commit/544c94621b527e835387c34691bbbc2982b22965))
* ignore cargo-mutants output directories ([6246fb6](https://github.com/skew202/antislop/commit/6246fb65d6585f00835c26a85b852d0d24783dd0))
* initialize release-please manifest with current version 0.2.0 ([516e870](https://github.com/skew202/antislop/commit/516e870bdfdff99718c64214e8aafeeb5ee210c9))
* integrate antislop into CI and pre-commit hooks ([ede20d3](https://github.com/skew202/antislop/commit/ede20d3954c43658fcf6d0467ca8a20f6c395b51))
* **npm:** bump package version to 0.3.0 ([d08b0ee](https://github.com/skew202/antislop/commit/d08b0ee788a6d055d91167a72592be0b2cf75152))
* optimize docker build context ([7333679](https://github.com/skew202/antislop/commit/7333679102c0e7fcc0e8810c13c195afaa6405df))
* release main ([704ca40](https://github.com/skew202/antislop/commit/704ca4043f3f9c11da59185be11160c89d32f460))
* release main ([#25](https://github.com/skew202/antislop/issues/25)) ([7ccb02b](https://github.com/skew202/antislop/commit/7ccb02b18e88d478e0264a0d6bd8d03adb341427))
* release v0.2.0 ([a6b4dd4](https://github.com/skew202/antislop/commit/a6b4dd4debf5c3b4d882ec7feccfeecf078f2881))
* release v0.2.2 ([ff6e3a9](https://github.com/skew202/antislop/commit/ff6e3a91f25646f552556e0d071bc58e592b0e00))
* remove CODE_OF_CONDUCT.md (use GitHub default) ([4c2d3fd](https://github.com/skew202/antislop/commit/4c2d3fd2be52925011e2acd572b7b488193cb468))
* remove deprecated 'command' parameter from release-please ([ee9f21a](https://github.com/skew202/antislop/commit/ee9f21ae9c85684bd72acd5368bb100f353bfbd3))
* **scripts:** add QA and coverage helper scripts ([8f634db](https://github.com/skew202/antislop/commit/8f634db7fa38e8109805503b4df1f4804ee86ce8))
* update actions/checkout v4-&gt;v6, @types/vscode 1.75-&gt;1.107 ([1d17d06](https://github.com/skew202/antislop/commit/1d17d069ba0eea6e4041efc7ac90b48f43fb0648))
* update dependencies and config files ([f62d12a](https://github.com/skew202/antislop/commit/f62d12a3e935e3bb40721603a7c4de04aaab48b5))
* **vscode-dev:** bump @types/node in /editors/vscode ([#6](https://github.com/skew202/antislop/issues/6)) ([51254e9](https://github.com/skew202/antislop/commit/51254e9ed065396b9b00e08bccc5ad3e8af77cbc))
* **vscode-dev:** bump typescript in /editors/vscode ([#3](https://github.com/skew202/antislop/issues/3)) ([efb470d](https://github.com/skew202/antislop/commit/efb470d81625478eab92e83d1dfd7c277dbcd879))
* **vscode:** bump vscode-languageclient in /editors/vscode ([#9](https://github.com/skew202/antislop/issues/9)) ([fad4030](https://github.com/skew202/antislop/commit/fad4030d0b4518da882caaac3a98a6dd80de551d))


### CI/CD

* add GitHub Action, Docker, and release workflow ([6ef2b06](https://github.com/skew202/antislop/commit/6ef2b06b8a4e1f241c84ac84e0a371302a90e495))
* add publish workflow for manual and auto-trigger ([a30cd12](https://github.com/skew202/antislop/commit/a30cd12712314632bc0cbe8d287dbe980a47e072))
* bump actions/cache from 4 to 5 ([ecb9501](https://github.com/skew202/antislop/commit/ecb950130c8d225295e71c97f8a33f27f3cd62c9))
* bump actions/configure-pages from 4 to 5 ([#7](https://github.com/skew202/antislop/issues/7)) ([7a2efdd](https://github.com/skew202/antislop/commit/7a2efdd29d186bf795d931cf84b5f1d9cf60f2a7))
* bump actions/download-artifact from 4 to 7 ([8660cc4](https://github.com/skew202/antislop/commit/8660cc40b2c7fa672fde379583c67ca24d4ecc8e))
* bump actions/setup-node from 4 to 6 ([ca9da6c](https://github.com/skew202/antislop/commit/ca9da6cea51c13b41cdff37d4688c66c68b6fd1d))
* bump actions/upload-pages-artifact from 3 to 4 ([#5](https://github.com/skew202/antislop/issues/5)) ([535e4aa](https://github.com/skew202/antislop/commit/535e4aaeb0eb30593e39a8d595417f3d60a075f0))
* bump aquasecurity/trivy-action from 0.29.0 to 0.33.1 ([#4](https://github.com/skew202/antislop/issues/4)) ([9bbf6c3](https://github.com/skew202/antislop/commit/9bbf6c3324448602039591a7e196089519d1bdc5))
* bump docker/build-push-action from 5 to 6 ([14287aa](https://github.com/skew202/antislop/commit/14287aa9e7503ad426acf5ae1dba7c5a84f06edf))
* bump peter-evans/create-pull-request from 7 to 8 ([#1](https://github.com/skew202/antislop/issues/1)) ([10d748d](https://github.com/skew202/antislop/commit/10d748d10f5c70ead41afed38ba04936922efad0))
* **hooks:** add pre-commit configuration for local linting ([62a0154](https://github.com/skew202/antislop/commit/62a01546d9f75ab737d6a1ffb076b83cff818847))
* remove aarch64-linux build from dist matrix (requires cross-toolchain) ([8c495de](https://github.com/skew202/antislop/commit/8c495dec27eae505528db7195ae26f78dbeee782))
* remove deprecated cache option from docker build ([7007813](https://github.com/skew202/antislop/commit/7007813cf06d4682f40bce7c584ad75f188e949b))

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
