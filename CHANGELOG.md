# ChangeLog

The following are lists of the notable changes included with each release.
This is intended to help keep people informed about notable changes between
versions, as well as provide a rough history. Each item is prefixed with
one of the following labels: `Added`, `Changed`, `Deprecated`,
`Removed`, `Fixed`, `Security`. We also use [Semantic
Versioning](http://semver.org) to manage the versions of this gem so
that you can set version constraints properly.

#### [Unreleased][unreleased]

* `Fixed`: bug preventing Rake test alternates from working with Minitest

#### [v2.1.1][v2.1.1] - 2017-06-15

* `Fixed`: Bug preventing Cucumber support from working

#### [v2.1.0][v2.1.0] - 2016-12-14

* `Added`: Swift Quick Tests support
* `Added`: Swift XCTest & XCUI Tests support
* `Added`: Swift Package Manager Tests support

#### [v2.0.1][v2.0.1] - 2016-07-22

* `Fixed`: bug causing crash when dealing with files named a single char
* `Removed`: Ruby implementation from benchmarks

#### [v2.0.0][v2.0.0] - 2016-07-21

* `Changed`: the -- option switch to -f -
* `Removed`: the --debug option switch
* `Added`: complete rewrite in Rust
* `Removed`: Ruby implementation

#### [v1.0.0][v1.0.0] - 2016-06-14

* `Fixed`: bug with alternating between rake task files and tests
* `Added`: ability to source possible alternates from STDIN
* `Changed`: the overall algorithm to simplify it now that have new scoring
* `Changed`: Scoring equation to take rank of secondary string into account
* `Changed`: debug output to understand matches to be more valuable
* `Fixed`: bug where path cleansing was incorrectly skewing scores

#### [v0.0.1][v0.0.1] - 2016-04-15

* `Added`: initial functional version with basic docs

[unreleased]: https://github.com/cyphactor/alt/compare/v2.1.1...HEAD
[v0.0.1]: https://github.com/cyphactor/alt/compare/7c9b152...v0.0.1
[v1.0.0]: https://github.com/cyphactor/alt/compare/v0.0.1...v1.0.0
[v2.0.0]: https://github.com/cyphactor/alt/compare/v1.0.0...v2.0.0
[v2.0.1]: https://github.com/cyphactor/alt/compare/v2.0.0...v2.0.1
[v2.1.0]: https://github.com/cyphactor/alt/compare/v2.0.1...v2.1.0
[v2.1.1]: https://github.com/cyphactor/alt/compare/v2.1.0...v2.1.1

