# BlockBench Model Format

`.bbmodel` is raw JSON text.

## File structure

`Root`
- `meta` (tree): metadata
- - `format_version` (string): format version, assumed `"4.9"`
- - `model_format` (string): unknown, assumed `"free"`
- - `box_uv`? (bool): whether box uv is used, assumed `false`
- `name`? (string): name of the model
- `model_identifier`? (string): unknown
- `visible_box`? (list\<number>): unknown
- `variable_placeholders`? (list): unknown
- `variable_placeholder_buttons`? (list): unknown
- `timeline_setups`? (list): unknown
- `unhandled_root_fields`? (tree): unknown
- `resolution`? (\[number; 2]): (possibly?) texture resolution
- `elements` (list\<`Part`>): list of parts
- `outliner` (`Outliner`): folder tree
- `textures` (list): list of all textures
- - `uuid` (string): uuid of this texture
- - `name` (string): texture name, assumed relative path
- - `source` (string): base64 (`data:image/png;base64,<data>`) png data
- - `folder` (string): unknown, assumed `""`
- - `id` (string): (potentially unused) id of this texture, assumed element index
- - `relative_path` (string): (unused) relative path to texture
- - `path` (string): absolute path to texture
- - `width` (number): texture width
- - `uv_width` (number): texture width for uvs, assumed `width` cuz with it nothing breaks
- - `height` (number): texture height
- - `uv_height` (number): texture height for uvs, assumed `height` cuz with it nothing breaks

`Part`
- `name` (string): name of this part
- `box_uv`? (bool): whether box uv is used
- `rescale`? (bool): unknown
- `locked`? (bool): unknown
- `light_emission`? (number): unknown
- `render_order`? (string): unknown, defaults to `"default"`
- `allow_mirror_modeling`? (bool): unknown, defaults to `true`
- `type`? (string): part type, defaults to `"cube"`
- `color`? (number): unknown, defaults to random
- `autouv`? (number): unknown
- `uuid` (string): uuid of this part
- `origin` (\[number; 3]): shift of the pivot from the part center
- `from` (\[number; 3]): starting position of this part
- `rotation`? (\[number; 3]): rotation of this part
- `to` (\[number; 3]): end position of this part (start + size)
- `faces` (tree): faces of this part
- - `*` (`Face`): a face
- - - `texture` (number): texture index as defined in `Root.textures`
- - - `uv` (\[number; 4]): uv texture mapping

`Outliner`
- `name` (string): name of the folder
- `uuid` (string): uuid of the folder
- `origin` (\[number; 3]): shift of the pivot from the part center
- `isOpen`? (bool): whether the folder is expanded in the editor
- `children` (list\<string|`Outliner`>): folder contents, parts are referenced via their uuids as strings
- `color`? (number): unknown
- `export`? (bool): unknown, defaults to `true`
- `mirror_uv`? (bool): unknown
- `locked`? (bool): unknown
- `visiblity`? (bool): whether a part is visible, defaults to `true`
- `autouv`? (number): unknown, defaults to `0`
- `selected`? (bool): unknown
