# schedule

Resource-injected, resumable execution plans, extracted as a standalone library
from the shuyra repository.

- Source commit: `378d4d826fff70851dfcf08ace8cded4a55d0d2b`
- Package: `schedule` 0.1.0, edition 2024
- No production dependencies, features, build script, or runtime adapter.

Build the library:

```sh
cargo build --lib --target x86_64-unknown-linux-gnu --locked
cargo build --lib --target riscv64gc-unknown-none-elf --locked
cargo build --lib --release --target riscv64gc-unknown-none-elf --locked
```

Run the host-only mechanism tests (37 tests):

```sh
cargo test --manifest-path tests/host/Cargo.toml \
  --target x86_64-unknown-linux-gnu --locked
```

`tests/host` is an independent workspace. See `PROVENANCE.md` for the exact
file mapping from the source tree.
