[![Build Status](https://travis-ci.org/cyphactor/alt.svg?branch=master)](https://travis-ci.org/cyphactor/alt)

# Alt

`alt` is a command line utility that attempts to find the "alternate path" for
the provided path. This is primarily intended for developers. It is written in
100% pure ruby, with zero dependencies. Hence, it is able to run on any machine
that has a ruby interpreter installed. This is most machines now days.

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

![Demo](https://raw.github.com/cyphactor/alt/master/resources/demo.gif)

For advanced usage, full reference of the command line interface, and frequently
asked questions please refer to the [Wiki](https://github.com/cyphactor/alt/wiki).

## Installation

To install `alt` simply copy the `alt` file from this repository into your
`~/bin` directory and make sure your `~/bin` directory is in your `PATH`.

## Use with Vim

There's no vim plugin. It may not end up needing one; we'll see. For now, you
can just stick the code below in your `.vimrc` to invoke `alt` with `<leader>.`.
**Note** that `alt` and the Vim Script example below **work** in both the
terminal based Vim and GUI based Vim like MacVim.

```vimscript
" Run a given vim command on the results of alt from a given path.
" See usage below.
function! AltCommand(path, vim_command)
  let l:alternate = system("~/bin/alt " . a:path)
  if empty(l:alternate)
    echo "No alternate file for " . a:path . " exists!"
  else
    exec a:vim_command . " " . l:alternate
  endif
endfunction

" Find the alternate file for the current path and open it
nnoremap <leader>. :w<cr>:call AltCommand(expand('%'), ':e')<cr>
```
