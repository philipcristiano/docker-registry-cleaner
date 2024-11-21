# Changelog

## [0.2.1](https://github.com/philipcristiano/docker-registry-cleaner/compare/v0.2.0...v0.2.1) (2024-11-21)


### Bug Fixes

* paginate catalog ([bb6db4b](https://github.com/philipcristiano/docker-registry-cleaner/commit/bb6db4babf97612de60875165c87c409519f7e56))

## [0.2.0](https://github.com/philipcristiano/docker-registry-cleaner/compare/v0.1.1...v0.2.0) (2024-11-21)


### Features

* Add --dry-run ([5e06433](https://github.com/philipcristiano/docker-registry-cleaner/commit/5e06433ae8dfe1be9ed84c0431b5fb658680ec0c))


### Bug Fixes

* **deps:** update rust crate clap to v4.5.20 ([#8](https://github.com/philipcristiano/docker-registry-cleaner/issues/8)) ([e9dee0f](https://github.com/philipcristiano/docker-registry-cleaner/commit/e9dee0f4b1dd98648fc3773dcb5ad088f8c03f72))
* **deps:** update rust crate clap to v4.5.21 ([#18](https://github.com/philipcristiano/docker-registry-cleaner/issues/18)) ([c7601d2](https://github.com/philipcristiano/docker-registry-cleaner/commit/c7601d2ec42b591300d43fde70e2cf9394040276))
* **deps:** update rust crate reqwest to 0.12 ([#14](https://github.com/philipcristiano/docker-registry-cleaner/issues/14)) ([6de18c0](https://github.com/philipcristiano/docker-registry-cleaner/commit/6de18c02d1ac22d4267d5b5cdc6c60c2591385fb))
* **deps:** update rust crate serde to v1.0.214 ([#9](https://github.com/philipcristiano/docker-registry-cleaner/issues/9)) ([2cd16d1](https://github.com/philipcristiano/docker-registry-cleaner/commit/2cd16d107ad50ee407ae467644871923cc382135))
* **deps:** update rust crate serde to v1.0.215 ([#17](https://github.com/philipcristiano/docker-registry-cleaner/issues/17)) ([0d9ef1d](https://github.com/philipcristiano/docker-registry-cleaner/commit/0d9ef1d9731b4aa59312090b0a80bee414ab3fd4))
* **deps:** update rust crate serde_json to v1.0.132 ([#11](https://github.com/philipcristiano/docker-registry-cleaner/issues/11)) ([30d4c8f](https://github.com/philipcristiano/docker-registry-cleaner/commit/30d4c8f2fdc9da8f56ffc9f8e00fdb6dca9aaec9))
* **deps:** update rust crate serde_json to v1.0.133 ([#19](https://github.com/philipcristiano/docker-registry-cleaner/issues/19)) ([5a7ab72](https://github.com/philipcristiano/docker-registry-cleaner/commit/5a7ab7212984cc769580b344f152bd559a958c57))
* **deps:** update rust crate service_conventions to 0.0.23 ([#12](https://github.com/philipcristiano/docker-registry-cleaner/issues/12)) ([d82e237](https://github.com/philipcristiano/docker-registry-cleaner/commit/d82e23723b30c9f797ba54d3ce3e3fdee3e03ed6))
* **deps:** update rust crate tokio to v1.41.1 ([#15](https://github.com/philipcristiano/docker-registry-cleaner/issues/15)) ([c6c4e36](https://github.com/philipcristiano/docker-registry-cleaner/commit/c6c4e36eae7b42756bfb2c0adc61da74098c941b))
* Don't delete unlabeled images ([4461f61](https://github.com/philipcristiano/docker-registry-cleaner/commit/4461f61963f82ff372a27fe803d100ba9d283fc7))

## [0.1.1](https://github.com/philipcristiano/docker-registry-cleaner/compare/v0.1.0...v0.1.1) (2024-07-07)


### Bug Fixes

* Handle annotations that don't include a digest ([59140a0](https://github.com/philipcristiano/docker-registry-cleaner/commit/59140a00e3d040919ec6896e665352933cb6c0de))

## 0.1.0 (2024-07-07)


### Features

* Use CLI args instead of hardcoded values ([354feee](https://github.com/philipcristiano/docker-registry-cleaner/commit/354feee33377f41798da50a710e2aaa179f02de3))


### Bug Fixes

* Work with manifest `application/vnd.oci.image.manifest.v1+json` ([d283694](https://github.com/philipcristiano/docker-registry-cleaner/commit/d283694b1e0533cf08da3620b033dd95658eb6e4))
