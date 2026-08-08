# TODO

## Remaining Code TODOs

- `src/traits.rs`
  - Implement `NamespacedValue for StructProperty::get_namespaced_value_mut`.

- `src/save/beta/iv_struct.rs`
  - Implement or remove the unused `get_iv_at`, `get_iv_at_mut`, and `new_box` stubs.
  - Keep the tested `IV::new(gvas_file, StorageType::...)` and `IVMut::new(gvas_file, StorageType::...)` API.

## Remaining Test TODOs

- `src/save/beta/pokemon_info.rs`
  - Add read-wrapper tests for `PokemonInfo::new(gvas_file, StorageType::BOXES(1))`.
  - Add tests for `InfoSnapshot::new` if implemented.

- `src/traits.rs`
  - Test immutable and mutable namespaced value lookup on `StructProperty`.

## Cleanup

- Remove stale TODO comments from files after the corresponding TODOs are completed.
- Remove unused imports like `GvasFile` or `StorageType` from modules after macro conversion if they are no longer referenced directly.
- Run `cargo fmt`.
- Run `cargo clippy --all-targets --all-features`.
- Run `cargo test`.