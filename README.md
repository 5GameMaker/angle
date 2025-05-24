# Angle - a Figura .moon extractor

> the export system is for recovering everything that isnt the
> .bbmodel and for CEM since the resourcepacks use the .moon file

\- a figura mod

Angle allows you to extract a full profile from a .moon file. If
you have a profile loaded on server, but you have lost the sources,
you can use `/figura export` to get the .moon file and then do
`$ angle avatar.moon path/to/figura/avatars/avatar` to get them back.

Keep in mind that we cannot restore what is not stored (i.e. some of
the blockbench settings are lost), and this tool only has support for
things I personally use.

(p.s. you can also install [Stylua](https://github.com/JohnnyMorganz/StyLua)
for formatting. A formatter will be executed automatically if found)

## Usage notes

Be sure to not move the avatar from the directory it was extracted into.
Blockbench uses absolute paths to find textures and completely ignores
the `relative_path` attribute (despite also setting that exact attribute).

## Building

Download [Rust](https://rustup.rs/).

> If you're on a POSIX system, you can do  
> `$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`  
> and then follow the prompts. Default configuration should be
> sufficient.

Do `$ cargo build`. Your binary should be at `target/debug/angle`
(`angle.exe` for Windows.)

Additionally, you can do `$ cargo install --path .` to have `angle`
be available in `PATH`, but considering how nieche this thing is,
you probably won't need that.

(note, `$ cargo run ..` can be used for a shorthand for a building
and .)

## Feature support

Things that are supported:
- Basic cubes of different sizes
- Virtually any tree Blockbench supports
- Possibly multiple textures
- Scripts

Things that are not supported:
- Rotations
- Animations
- Folders
- Anything else that is not in supported list

Things that will never be supported:
- Doing basic system operations via DOS shell commands
- Running a Minecraft server for extracting profile data

If you need those features and know how to code, feel free to PR them.

(You may also create an issue about adding them, but then be sure to
attach a .moon file as well an original blockbench model).

## What this tool is not

This is *not* a Figura Avatar Downloader. It doesn't download anything
from anywhere and any PR adding that funcionality will be rejected.
Think of this as an unarchiver rather than a downloader.

However, this section in particular is an SEO thing so you can actually
find it in Google :>
