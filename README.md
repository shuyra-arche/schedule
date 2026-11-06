# schedule

Shuyra 的 `no_std` 可恢复执行计划库，通过资源注入组合与调度步骤。
本库不依赖具体业务协议，也不提供运行时适配或 Unit server。

`src/plan.rs`、`sequence.rs`、`compose.rs` 描述计划；`graph.rs`、`dispatch.rs`、
`system.rs` 组织执行，`resource.rs` 提供资源接口，`src/lib.rs` 是公共入口。
`tests/host` 是独立工作区，验证机制及源码依赖边界。根库禁用测试入口。

## 工具链与维护

使用 `rust-toolchain.toml` 固定的 `nightly-2026-09-04`，首次运行需要 Rustup
下载工具链及 RV64 目标。依赖版本以 Cargo 清单和锁文件为准。
下列命令从仓库根目录执行；宿主测试不代表真实内核运行验证。

```sh
cargo build --lib --target x86_64-unknown-linux-gnu --locked
cargo build --lib --target riscv64gc-unknown-none-elf --locked
cargo test --manifest-path tests/host/Cargo.toml --target x86_64-unknown-linux-gnu --locked
```

## 贡献与来源

维护与 PR 规则见 [Shuyra 贡献规范](https://github.com/shuyra-arche/vanllia/blob/main/CONTRIBUTING.md)。
来源、作者与许可情况见 [PROVENANCE.md](PROVENANCE.md)。
