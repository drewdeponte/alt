# Alt

`alt` is a command line utility that finds the "alternate paths" for the
provided path.

**alt** [OPTIONS] PATH

**alt** finds alternate file paths for the given `PATH` based on a similarity
ranking. For example, if you were in a Ruby project and ran
`alt spec/app/queues/fee/user_fee_submission_spec.rb` the output would include
ranked at the top `app/queues/fee/user_fee_submission.rb`. In this case the
alternate for the test file is the implementation file. It is important to
understand that alternate files are simply filenames and paths that rank high
in similarity.

**alt** by default outputs all possible alternate paths in ranked order. In
older major versions the behavior was to only output the highest ranked file
path. To retain this behavior simply use the `truncate` option with a value of
1.

This is primarily intended for developers. It is written in
[Rust][]. Hence, it is compiled and distributed as a
binary.

It was originally written to alternate files in vim, but has no dependency on
vim at all and can be used in many other scenarios as it is just a command line
utility.

Its interface is as simple as they come.

* Pass it a path as the first argument
* It will print the alternate paths in ranked order on separate lines via standard out

For example:

```text
$ alt spec/app/queues/fee/user_fee_submission_spec.rb
app/queues/fee/user_fee_submission.rb
app/bar/car/user_goal.rb
app/queues/modification/vehicle.rb
```

![Demo](https://raw.github.com/drewdeponte/alt/master/resources/demo-with-telescope.gif)

For advanced usage & full reference of the command line interface, please refer
to the man page via `man alt`.

## Installation

If you are on a platform other than macOS or Arch Linux you will have to build
your own version from source.

### macOS

To install on macOS we provide a [Homebrew](http://brew.sh) tap which provides
the `alt` formula. You can use it by doing the following:

#### Add the Tap

```
brew tap "drewdeponte/oss"
```

#### brew install

```
brew install drewdeponte/oss/alt
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

*Note:* The above requires of course that you have [rust][] and Cargo.

## Use with NeoVim

There's no NeoVim or Vim plugin. It may not end up needing one; we will see.
The snippet below is a basic setup I use in my NeoVim to tie **alt** into
[telescope][] so that I can fuzzy select the ranked alternate paths. For now,
you can just stick the code below in your `init.lua` to invoke **alt** with
`<leader>.` Note that **alt** and the NeoVim Lua example below work in both the
terminal based NeoVim and GUI based NeoVim.

```lua
-- ----------------------------------------------
-- Alternate File Switching
-- ----------------------------------------------
local pickers = require "telescope.pickers"
local finders = require "telescope.finders"
local conf = require("telescope.config").values

local alternates_picker = function(alternates, opts)
  opts = opts or {}
  pickers.new(opts, {
	prompt_title = "alternates",
	finder = finders.new_table {
	  results = alternates
	},
	sorter = conf.generic_sorter(opts),
  }):find()
end

function alt(path)
  local function isempty(s)
	return s == nil or s == ''
  end

  -- This is where you can configure it with CLI options so
  -- it behaves how you want it to.
  local alternates = vim.fn.system("some/path/alt " .. path)
  if isempty(alternates) then
	return nil
  else
	local alternates_table = {}
	for s in alternates:gmatch("[^\r\n]+") do
	  table.insert(alternates_table, s)
	end
	return alternates_table
  end
end

function alt_command(path, alt_handler)
  local current_file_path = vim.fn.expand('%')
  local alternate_file_paths = alt(current_file_path)
  if alternate_file_paths == nil then
	print("No alternate files found for " .. current_file_path .. "!")
  else
	alt_handler(current_file_path, alternate_file_paths)
  end
end

function alt_handler(current_file_path, alternate_file_paths)
  alternates_picker(alternate_file_paths)
end

vim.keymap.set('n', '<leader>.', function()
  alt_command(vim.fn.expand('%'), alt_handler)
end)
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

If you interested at all in contributing. Please do. We are a welcoming group
and are willing to provide guidance whenever possible.

Please see [`CONTRIBUTING.md`](./CONTRIBUTING.md) for more details on
contributing.

## License

`alt` is free software, and may be redistributed under the terms specified in
the LICENSE file.

## About

`alt` is maintained and funded by [Drew De Ponte][drewdeponte].

[Rust]: https://www.rust-lang.org
[rust]: https://www.rust-lang.org
[telescope]: https://github.com/nvim-telescope/telescope.nvim
