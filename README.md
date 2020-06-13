[![Build Status](https://travis-ci.org/uptech/alt.svg?branch=master)](https://travis-ci.org/uptech/alt)
[![Join the chat at https://gitter.im/uptech/alt](https://badges.gitter.im/uptech/alt.svg)](https://gitter.im/uptech/alt?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

# Alt


`alt` is a command line utility that attempts to find the "alternate path" for
the provided path. This is primarily intended for developers. It is written in
Rust. Hence, it is compiled and distributed as a binary.

It was originally written to alternate files in vim, but has no dependency on
vim at all and can be used in many other scenarios as it is just a command line
utility.

Its interface is as simple as they come.

* Pass it a path as the first argument
* It will print the alternate path to standard out

For example:

```text
$ alt spec/app/queues/fee/user_fee_submission_spec.rb
app/queues/fee/user_fee_submission.rb
```

![Demo](https://raw.github.com/uptech/alt/master/resources/demo.gif)

For advanced usage, full reference of the command line interface, and frequently
asked questions please refer to the [Wiki](https://github.com/uptech/alt/wiki).

## Installation

If you are on a platform other than macOS or Arch Linux you will have to build
your own version from source.

### macOS

To install on macOS we provide a [Homebrew](http://brew.sh) tap which provides
the `alt` formula. You can use it by doing the following:

#### Add the Tap

```
brew tap "uptech/homebrew-oss"
```

#### brew install

```
brew install uptech/oss/alt
```

### Arch Linux

Install from the AUR:

```
git clone https://aur.archlinux.org/alt.git
cd alt
makepkg -si
```

### Build from Source

If you are on another platform you will have to build from source. Given
that `alt` is managed via Rust's Cargo. It can build as follows:

```
$ cargo build --release
```

Once you have built it successfully you should relocate the
`target/release/alt` binary to a location that exists in your `PATH` so
that you can easily use it. You should also relocate the `doc/alt.1` man page
to the appropriate location.

*Note:* The above requires of course that you have [rust](http://rust-lang.org)
and Cargo.

## Use with Vim

There's no vim plugin. It may not end up needing one; we'll see. For now, you
can just stick the code below in your `.vimrc` to invoke `alt` with `<leader>.`.
**Note** that `alt` and the Vim Script example below **work** in both the
terminal based Vim and GUI based Vim like MacVim.

```vimscript
" Run a given vim command on the results of alt from a given path.
" See usage below.
function! AltCommand(path, vim_command)
  let l:alternate = system("alt " . a:path)
  if empty(l:alternate)
    echo "No alternate file for " . a:path . " exists!"
  else
    exec a:vim_command . " " . l:alternate
  endif
endfunction

" Find the alternate file for the current path and open it
nnoremap <leader>. :w<cr>:call AltCommand(expand('%'), ':e')<cr>
```

## Ignoring Things

`alt` by default ignores hidden directory entries, globs defined in a
`.ignore` file and globs defined in your project's `.gitignore` and your
global `.gitignore`. It does this because in our experience that is
generally the behavior you want. If however you want for example to be able to
alternate between hidden files for some reason, you can always use the `-a`
option. If you want to have `alt` ignore some specific paths/files that you
don't want Git to ignore. You can simply define them in the `.ignore` file
at the root of your project.

## Contributing

If you interested at all in contributing. Please do. We are welcoming
group and are willing to provide guidance whenever possible.

Please see [`CONTRIBUTING.md`](./CONTRIBUTING.md) for more details on
contributing.

## License

`alt` is Copyright © 2016 - 2020 UpTech Works, LLC. It is free software, and
may be redistributed under the terms specified in the LICENSE file.

## About <img src="http://upte.ch/img/logo.png" alt="uptech" height="48">

`alt` is maintained and funded by [UpTech Works, LLC][uptech], a a
software design & development agency & consultancy.

We love open source software. See [our other projects][community] or
[hire us][hire] to design, develop, and grow your product.

[community]: https://github.com/uptech
[hire]: http://upte.ch
[uptech]: http://upte.ch
