# Angle - a Figura avatar extractor

> the export system is for recovering everything that isnt the
> .bbmodel and for CEM since the resourcepacks use the .moon file

\- a figura mod

Angle allows you to extract a full avatar from a .moon file. If
you have an avatar uploaded to the cloud, but lost the sources,
you can use `/figura export avatar avatar` to get the ~~.moon~~ .nbt file and then run
`$ angle avatar.nbt avatars/avatar` to get it back.

Keep in mind that we cannot restore what is not stored (i.e. blockbench settings are
lost).

(p.s. you should also install [Stylua](https://github.com/JohnnyMorganz/StyLua)
for formatting. It will be executed automatically if found)

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

Or you can run `$ cargo install --path .` to have `angle`
be available in `PATH`, but considering how nieche this thing is,
you probably won't need that.

(note, `$ cargo run ...` can be used for a shorthand for a building and running)

## Feature support

Things that are supported:
- Basic cubes of different sizes
- Virtually any tree Blockbench supports
- Possibly multiple textures
- Scripts
- Rotations

Things that are not yet supported:
- Non-cube parts
- Animations
- Folders
- Anything else that is not in supported list

Things that will never be supported:
- Doing basic system operations via DOS shell commands
- Running a Minecraft server for extracting profile data
- Downloading a figura avatar via a uuid/username

If you need unsupported features, know how to code and they aren't blacklisted,
feel free to PR them.

(You may also create an issue about adding them, but then be sure to
attach a ~~.moon~~ .nbt file as well an original blockbench model).
