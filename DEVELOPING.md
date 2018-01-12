## Developing

### Building Debug

```text
$ cargo build
```

### Building Release

```text
$ cargo build --release
```

### Benchmarking

We have a script setup that runs benchmarks for common scenarios we would like to see. It runs them against two coppies of `alt`, the version installed in `/usr/local/bin/alt` and the version sitting in `target/release/alt`. So, conceptually it is comparing the latest builds benchmarks with the latest stable installed version.

To prepare the benchmarking you need to first make sure that you have the proper fixtures. Currently, we manage one of our fixtures using git submodules. So, you have to run the following to get setup with the submodule to be able to properly run the benchmarks.

```text
$ git submodule update --init --recursive
```

Once you have pulled the submodule for the fixtures you are ready to run the benchmarks using the following.

```text
$ ruby benchmark.rb
```

The above produces output that should look something like the following:

```text
/usr/local/bin/alt - aa_zz_possibles.txt fixture
user     system      total        real
For impl. file:   0.090000   0.390000  10.630000 (  9.779738)
For test file:    0.090000   0.430000  12.290000 ( 11.470875)

target/release/alt - aa_zz_possibles.txt fixture
user     system      total        real
For impl. file:   0.090000   0.430000   9.160000 (  8.385433)
For test file:    0.100000   0.450000  11.120000 ( 10.383350)

/usr/local/bin/alt - ruby_on_rails_discourse_possibles.txt fixture
user     system      total        real
For impl. file:   0.100000   0.470000  21.540000 ( 20.189522)
For test file:    0.100000   0.460000  22.650000 ( 21.306799)

target/release/alt - ruby_on_rails_discourse_possibles.txt fixture
user     system      total        real
For impl. file:   0.090000   0.460000  18.610000 ( 17.320507)
For test file:    0.100000   0.460000  20.150000 ( 18.897057)

/usr/local/bin/alt - With cd spec/fixtures/discourse using Glob/WalkDir
user     system      total        real
For impl. file:   0.130000   0.510000 300.660000 (304.231532)
For test file:    0.130000   0.510000 301.810000 (303.974687)

target/release/alt - With cd spec/fixtures/discourse using Glob/WalkDir
user     system      total        real
For impl. file:   0.110000   0.500000  86.560000 ( 88.260965)
For test file:    0.120000   0.500000  88.820000 ( 90.348901)

/usr/local/bin/alt - With cd spec/fixtures/discourse using find
user     system      total        real
For impl. file:   0.100000   0.480000 195.200000 (189.660669)
For test file:    0.120000   0.500000 210.510000 (203.112697)

target/release/alt - With cd spec/fixtures/discourse using find
user     system      total        real
For impl. file:   0.120000   0.510000 209.580000 (201.857882)
For test file:    0.110000   0.480000 196.160000 (187.753652)
```

### Releasing

#### Prep alt Repo Release

1. Bump the version in the `Cargo.toml` and in the `CHANGELOG.md` and build it using `cargo build --release` to make sure that `Cargo.lock` is up to date. Then commit it as a version bump commit and push it up to master.
2. Tag the version bump commit at the appropriate version tag and push the tags up.
3. Define a new Release on GitHub for that version tag and copy the changes for this version from the `CHANGELOG.md` into the release notes.

#### Update alt Homebrew Formula in homebrew-oss

1. Open the `homebrew-oss/Formula/alt.rb` file in your editor.
2. Copy the url from GitHub for the `tar.gz` of release that you made in the previous step and paste it in place of the current `url` in the formula.
3. Update the `version` in the formula to match this new version.
4. Download the `tar.gz` of this release from GitHub
5. Generate the sha256 of the downloaded `tar.gz` by running, `shasum -a 256 the.tar.gz`.
6. Copy the sha256 sum and paste it in place of the current `sha256`.

#### Test out installing from the new Formula

```
brew edit alt
```

Make the same changes as we just did in the above section and save them.

```
brew uninstall alt
```

Make sure all of the version are uninstalled.

```
brew install -s --build-bottle alt
```

#### Bottle It Up

Make sure you are in the `alt.rb` formula directory in `homebrew-oss`.

```
brew bottle alt
```

The above will produce a file looking something like `alt-3.0.0.high_sierra.bottle.tar.gz` and output a new DSL example for the bottle.

Merge the DSL example with the one already in `alt.rb` appropriately.

#### Release It

Stage both the `alt-3.0.0.high_sierra.bottle.tar.gz` and the changes to the formula and commit them and push them.

Thats it.
