# ChangeLog

The following are lists of the notable changes included with each release.
This is intended to help keep people informed about notable changes between
versions, as well as provide a rough history. Each item is prefixed with
one of the following labels: `Added`, `Changed`, `Deprecated`,
`Removed`, `Fixed`, `Security`. We also use [Semantic
Versioning](http://semver.org) to manage the versions of this gem so
that you can set version constraints properly.

#### [Unreleased][unreleased]

#### [v4.1.0][v4.1.0] - 2023-05-19

* `Changed`: path scoring algorithm to include leading edge filename scoring & weight

#### [v4.0.0][v4.0.0] - 2023-05-08

* `Added`: multi-line/multi-alternate output
* `Changed`: default behavior to output all possible alternates sorted by score
* `Added`: CLI option to truncate possible alternates down to a specific number
* `Changed`: scoring algorithm to be able to weight filenames different than paths
* `Added`: CLI option to override the scoring algorithms default filename weight
* `Added`: CLI option to override the scoring algorithms default path weight
* `Added`: support for optionally using threads to score in parallel
* `Added`: CLI option to enable parallel scoring
* `Fixed`: bug where scored paths weren't being sorted properly
* `Fixed`: bug where some paths would score as NaN and make sorting off
* `Changed`: Vim example to a NeoVim Lua example as that is what I use

#### [v3.4.0][v3.4.0] - 2021-04-17

* `Added`: support for foo_test.go file types

#### [v3.3.0][v3.3.0] - 2020-06-12

* `Added`: support for Rails monorepo spec & test files
* `Added`: support for JSX specs

#### [v3.2.0][v3.2.0] - 2020-02-18

* `Added`: support for mocha type test files for JS & TypeScript in same dir

#### [v3.1.0][v3.1.0] - 2019-01-26

* `Added`: support for mocha type test files for JS & TypeScript

#### [v3.0.0][v3.0.0] - 2018-01-12

* `Added`: -a option override the default of ignoring hidden directory entries
* `Changed`: walkdir implemantion to ignore::WalkBuilder to gain performance and glob based ignoring
* `Changed`: glob implementation to walkdir to gain performance

#### [v2.4.0][v2.4.0] - 2017-08-10

* `Added`: support for Scala Test/Spec/Suite
* `Added`: man page

#### [v2.3.0][v2.3.0] - 2017-07-10

* `Added`: support for Groovy
* `Changed`: specific extension handling into generic extension handling

#### [v2.2.0][v2.2.0] - 2017-07-08

* `Added`: support for Java Maven JUnit
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

[unreleased]: https://github.com/uptech/alt/compare/v4.0.0...HEAD
[v0.0.1]: https://github.com/uptech/alt/compare/7c9b152...v0.0.1
[v1.0.0]: https://github.com/uptech/alt/compare/v0.0.1...v1.0.0
[v2.0.0]: https://github.com/uptech/alt/compare/v1.0.0...v2.0.0
[v2.0.1]: https://github.com/uptech/alt/compare/v2.0.0...v2.0.1
[v2.1.0]: https://github.com/uptech/alt/compare/v2.0.1...v2.1.0
[v2.1.1]: https://github.com/uptech/alt/compare/v2.1.0...v2.1.1
[v2.2.0]: https://github.com/uptech/alt/compare/v2.1.1...v2.2.0
[v2.3.0]: https://github.com/uptech/alt/compare/v2.2.0...v2.3.0
[v2.4.0]: https://github.com/uptech/alt/compare/v2.3.0...v2.4.0
[v3.0.0]: https://github.com/uptech/alt/compare/v2.4.0...v3.0.0
[v3.1.0]: https://github.com/uptech/alt/compare/v3.0.0...v3.1.0
[v3.2.0]: https://github.com/uptech/alt/compare/v3.1.0...v3.2.0
[v3.3.0]: https://github.com/uptech/alt/compare/v3.2.0...v3.3.0
[v3.4.0]: https://github.com/uptech/alt/compare/v3.3.0...v3.4.0
[v4.0.0]: https://github.com/uptech/alt/compare/v3.4.0...v4.0.0
[v4.1.0]: https://github.com/uptech/alt/compare/v4.0.0...v4.1.0
