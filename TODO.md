# TODO

## Old Systems To Update

- `src/save/beta/shiny_list.rs`
  - Replace `new_party` / `new_box` with `new(gvas_file, storage_type)` using the storage wrapper macro.
  - Add or update mutable wrapper constructor to use `StorageType` for party and boxes.

- `src/save/beta/pokemon_classes.rs`
  - Replace `new_party` / `new_box` with `new(gvas_file, storage_type)` using the storage wrapper macro.
  - Keep class-specific methods as normal explicit methods.

- `src/save/beta/pokemon_gender.rs`
  - Replace `new_party` constructors with `new(gvas_file, storage_type)` using the storage wrapper macro.
  - Add box support if the save has `BoxNPokemonGender` properties.

- `src/save/beta/iv_struct.rs`
  - Replace `new_party` / unfinished `new_box` with `new(gvas_file, storage_type)` using the storage wrapper macro.
  - Remove unused `box_number` / `storage_type` fields if the wrapper only needs `property`.
  - Implement `get_iv_at`, `get_iv_at_mut`, and the remaining IV mutation/read paths.

- `src/save/beta/pokemon_info.rs`
  - Finish converting constructors to the storage wrapper macro.
  - Keep `PokemonInfo::new(gvas_file, StorageType::PARTY)` and `PokemonInfo::new(gvas_file, StorageType::BOXES(n))` as the public API.
  - Add `InfoSnapshot::new(gvas_file, storage_type, index)` or remove the TODO if snapshots are not part of the intended API.

- `src/save/beta/row_id.rs`
  - Consider whether `new(gvas_file, box_number)` should stay box-only or move to `StorageType` for consistency.

- `src/save/beta/slot_id.rs`
  - Consider whether `new(gvas_file, box_number)` should stay box-only or move to `StorageType` for consistency.

- `src/save/beta/macros.rs`
  - Keep wrapper macros private/internal unless path imports are needed.
  - If using `#[macro_use] mod macros;`, do not import macros with `use crate::...` in child modules.

## Code TODOs To Finish

- `src/traits.rs`
  - Implement `NamespacedValue for StructProperty::get_namespaced_value_mut`.

- `src/save/beta/pokemon/natures.rs`
  - Implement `From<BetaEnumStr> for Nature` or change it to `TryFrom<BetaEnumStr>` so invalid enum strings can fail safely.

- `src/utils/custom_struct.rs`
  - Add tests for `get_struct_property_at_idx_mut`.

## Files That Need Tests

- `src/save/beta/pokemon_classes.rs`
  - Read party and box class paths.
  - Test `parse_class` against real class paths.
  - Test wrapper constructor with `StorageType::PARTY` and `StorageType::BOXES(1)`.

- `src/save/beta/pokemon_gender.rs`
  - Read party gender values.
  - Mutate gender values on a cloned save.
  - Add box tests if box gender data exists.

- `src/save/beta/iv_struct.rs`
  - Read all IVs for a party Pokemon.
  - Read one IV with `get_iv_at`.
  - Mutate one IV with `get_iv_at_mut` / `set_iv_at` on a cloned save.
  - Add box IV coverage through `StorageType::BOXES(1)`.

- `src/save/beta/pokemon_info.rs`
  - Update wrapper tests to call `PokemonInfo::new(gvas_file, StorageType::PARTY)`.
  - Add wrapper tests for `StorageType::BOXES(1)`.
  - Test `PokemonInfoMut::new(gvas_file, StorageType::PARTY)` and box mutation if supported.
  - Add tests for `InfoSnapshot::new` if implemented.

- `src/save/beta/pokemon/natures.rs`
  - Test every known nature enum number.
  - Test invalid enum numbers if conversion becomes fallible.

- `src/traits.rs`
  - Test immutable and mutable namespaced value lookup on `StructProperty`.

- `src/utils/custom_struct.rs`
  - Test mutable struct lookup updates the underlying `GvasFile` clone.

- `src/save/beta/row_id.rs`
  - Add box row ID read tests.

- `src/save/beta/slot_id.rs`
  - Add box slot ID read tests.

## Cleanup After Conversion

- Remove old `new_party` / `new_box` constructors once all callers and tests use `StorageType`.
- Remove unused imports like `GvasFile` or `StorageType` from modules after macro conversion if they are no longer referenced directly.
- Run `cargo fmt`.
- Run `cargo test`.
