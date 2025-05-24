# Figura File Format

Figura's `.moon` (`.nbt`) is a gzip-compressed nbt file.

## File structure

`Root`
- `metadata` (tree): stuff from avatar.json
- - `authors` (string): an author as a string (todo: multiple authors)
- - `color` (string): a hex color that will be displayed in the menu
- - `name` (string): avatar name as displayed in the menu
- - `ver` (string): Figura and Minecraft versions in `<figura>+<minecraft>` format
- `models` (`ModelPart`): list of all models as accessible in lua via `models`
- - `chld.(*.name=<name>)` (tree/FiguraModelPart): a model as defined in `<name>.bbmodel`
- `scripts` (tree): all scripts
- - `*` (bytes): lua scripts with leading spaces on each line removed
- `textures` (tree): all textures
- - `src.<name>` (bytes): texture data
- - `data` (list): list of textures
- - - `*.d` (string): texture name

`ModelPart` (tree)
- `name` (string): name of this model part
- `chld` (list\<`ModelPart`>): list of children
- `pt`? (string): (possibly?) display name of this model part
- `rot`? (\[byte|float; 3]): rotation of this part
- `piv`? (\[byte|float; 3]): pivot point of this part
- `cube_data`? (`CubeData`): dimensions of this part
- `f`? (\[byte|float; 3]): part position
- `t`? (\[byte|float; 3]): part position + size

`CubeData` (tree)
- `*` (tree): a face
- - `uv` (\[byte|float; 4]): texture mapping
- - `tex` (int): texture id as defined in `Root.textures` (0-based)

## Expansion rules

- `Root.metadata` is saved as `avatar.json`, but without `ver` field
- `Root.textures.src.<name>` are saved as `<name>.png`
- `Root.scripts.<name>` are saved as `<name>.lua`
- `Root.models.child.(*.name=<name>)` are saved as `<name>.bbmodel`

Afterwards, if found, `stylua` is executed on all `*.lua` files to
restore leading whitespace.

As of now we don't use `ModelPart.pt` field.
