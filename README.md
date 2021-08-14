# Shared Files
Share Files using ARCropolis API

## Requirements:

- [ARCropolis 1.1.3 or higher](https://github.com/Raytwo/ARCropolis/releases/latest)

## Installation:

- Place the plugin in `sd:/atmosphere/contents/01006A800016E000/romfs/skyline/plugins/`

## Usage:

Create a file called `share.toml` in either `sd:/ultimate/mods/[Mod Folder]/` (UMM Path) or `rom:/arc/` (ARC Path)

Inside the `share.toml` file, create a array called `files`, then enter the file you want to be used and give it the value of an array with the files you'll share

For example, this
```toml
[files]
"fighter/mario/model/body/c00/def_mario_001_col.nutexb" = [
    "fighter/mario/model/body/c01/def_mario_001_col.nutexb",
    "fighter/mario/model/body/c02/def_mario_001_col.nutexb"
]
```

will make `def_mario_001_col` in c02 and c01 load the `c00` file.

## Important Note

The file that will be shared needs to exist in the folder that the `share.toml` goes to