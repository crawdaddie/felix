pub const HELP: &str = "# fm
A simple TUI file manager with vim-like keybind.

## Usage

`fm` => Show items in current directory.
`fm <directory path>` => Show items in the path.
Both relative and absolute available.

### Manual

j / Key Up            :Go up.
k / Key Down          :Go down.
h / Key Left          :Go to parent directory if exists.
l / Key Right / Enter :Open file or change directory.
gg                    :Go to the top.
G                     :Go to the bottom.
dd                    :Delete and yank item.
yy                    :Yank item.
p                     :Put yanked item in the current directory.
V                     :Switch to select mode.
  - d                 :In select mode, delete and yank selected items.
  - y                 :In select mode, yank selected items.
Ctrl+c                :Copy file name to clipboard.
t                     :Toggle sort order (name <-> modified time).
:                     :Switch to shell mode.
c                     :Switch to rename mode.
/                     :Switch to filter mode.
E                     :Empty the trash directory.
:q / ZZ               :Exit the program.
Esc                   :Return to normal mode.
H                     :Show help.

## Configuration

config file    : $XDG_CONFIG_HOME/fm/config.toml
trash directory: $XDG_CONFIG_HOME/fm/trash

for more detail, visit:
https://github.com/kyoheiu/fm
";
