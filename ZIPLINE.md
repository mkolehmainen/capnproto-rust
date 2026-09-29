# The `zipline` branch of `mkolehmainen/capnproto-rust`

This branch is ZPR's fork of capnproto-rust. It exists for one feature:
**passing Unix file descriptors over Cap'n Proto RPC**. `ph-cli` uses it to
hand `ph` the packet-capture file. That work is upstream PR
[capnproto/capnproto-rust#666](https://github.com/capnproto/capnproto-rust/pull/666)
by emilazy, and it isn't merged. This branch holds that PR plus the fixes ZPR
needs on top.

It was forked from `emilazy/capnproto-rust` on 2026-09-29 so the PR's commits
can't be lost to a force-push on that side. An earlier pin, `cfbcb9b`, was
orphaned that way.

## Who uses it

`zl-zpr-core` patches `capnp`, `capnp-futures`, `capnp-rpc` and `capnpc` to
this repository, by `rev`, in its root `Cargo.toml` `[patch.crates-io]`. The
FD-passing code in core is gated on `cfg(all(unix, feature = "capnp-ancillary"))`.
Cargo applies a patch on every target, so **this branch must build on every
target, Windows included**, even though FD passing is unix-only.

The Cap'n Proto version is lockstep across `zl-zpr-common`, `zl-zpr-compiler`,
`zl-zpr-visaservice` and `zl-zpr-core`, because `zl-zpr-common` exports the
generated code. A patch only replaces requirements it is semver-compatible
with, so **this branch's minor version sets the minor for all four repos.**
Today that's 0.26.

## What the branch carries

Base: `capnp-v0.26.2` + emilazy's 13 FD-passing commits, ending at `cb619b2e`
(emilazy `push-xstmntksusmk`, 2026-09-29). On top of that:

| Commit | Kind | What |
|---|---|---|
| `5b027f86` | ours | Non-unix FD stubs compile. This fixes the Windows build (E0637, E0004 ×2). |
| `cf1bf04e` | ours | rustix `"time"` feature. Without it, rustix 1.1.5 fails `tokio-unix-fd-stream` (E0433 ×6). |
| `ebe2377d` | ours | `eof_mid_passthrough_run` test ported to `AsyncFdReadExt`. |
| `9bc9d826` | ours | `FdHooks` gated on `alloc`. This fixes the `--no-default-features` builds. |
| `d66f6c62` | upstream `9eb4b9a3` | Bool `PrimitiveElement` bounds check (capnp 0.27.0). |
| `24019c0b` | upstream `081b742d` | `get_data_field` bounds-check overflow (capnp 0.27.2). |
| `b293800e` | upstream `1ed0d0e5` | Peer-triggered panic re-sending a twice-imported capability (capnp-rpc 0.26.2). The conflict with the FD work was resolved by hand. |
| `c9a2764d` | upstream `f7269c6e` | Regression test for `24019c0b`. |

As of 2026-09-29, every security-relevant upstream fix up to capnp 0.27.2 is
on this branch. Add a row here for every commit you add.

## Rules

- **Only fast-forward `zipline`. Never force-push.** Consumers pin SHAs on this
  branch, and rewriting history orphans them.
- **Don't bump crate versions.** They stay `capnp` 0.26.2, `capnp-rpc` 0.26.1,
  `capnp-futures` 0.26.1 and `capnpc` 0.26.0. Consumers write `"0.26"`
  requirements. A minor bump (0.27) stops the patch matching them at all, and
  a patch-level bump only disguises which upstream release we're based on.
  A version lower than crates.io's is fine: Cargo prefers a matching patch
  when it resolves, as long as the lockfile is re-resolved (see below).
- **Take upstream fixes with `git cherry-pick -x <sha>`.** If you resolve a
  conflict, add a bracketed `[zipline: …]` note to the commit message that says
  what you changed and why. Bring the upstream regression test along, and
  check that it fails without the fix.
- **Our own fixes** get a commit message that quotes the error they fix.
  Offer them to emilazy on #666 when that makes sense.

## Remotes

```sh
git remote add emilazy https://github.com/emilazy/capnproto-rust.git    # PR #666
git remote add upstream https://github.com/capnproto/capnproto-rust.git
git fetch emilazy && git fetch upstream --tags
```

To see what upstream has shipped that this branch doesn't:
`git log --oneline --no-merges $(git merge-base HEAD upstream/master)..upstream/master`.

## Verification before every push

The test build scripts compile schemas that import `/capnp/stream.capnp`, which
comes from `libcapnp-dev` (`sudo apt install capnproto libcapnp-dev`, the same
packages upstream CI installs). Without it, the builds fail with
"'/capnp/stream.capnp' is not found". For the Windows checks, run
`rustup target add x86_64-pc-windows-msvc` first. Then run:

```sh
cargo fmt --all -- --check
cargo test --workspace
cargo test -p capnp-futures -p capnp-rpc -p capnp-rpc-test -p capnp-futures-test \
    --features capnp-futures/tokio-unix-fd-stream          # includes pass_fd
(cd capnp && for f in "--no-default-features" \
                      "--no-default-features --features alloc" \
                      "--no-default-features --features std" \
                      "--features sync_reader" "--features unaligned"; do
    sh -c "cargo test $f" || exit 1; done)
W=x86_64-pc-windows-msvc
cargo check --target $W -p capnp -p capnp-futures -p capnp-rpc -p capnpc
cargo check --target $W -p capnp -p capnp-futures -p capnp-rpc -p capnpc \
    --features capnp-futures/tokio-unix-fd-stream
cargo check --target $W -p capnp -p capnp-futures -p capnp-rpc --tests
cargo check --target $W -p capnp --no-default-features
```

These clippy and rustc warnings come from upstream or emilazy code and are
expected. Don't add new ones:
- `manual checked division` in `capnp/src/primitive_list.rs`
- `iterate on a map's values` in `capnp-rpc/src/rpc.rs`
- `unused variable: fmt` in `capnp/src/lib.rs`, with `--no-default-features`
- `value assigned to set_cloexec is never read` in
  `capnp-futures/src/io/tokio/unix_fd_stream.rs`. This one is harmless: on
  Linux, received FDs get close-on-exec atomically through `CMSG_CLOEXEC`.

After pushing, bump the `rev` in `zl-zpr-core`'s `[patch.crates-io]`, all four
lines, in a PR that runs core's netns tier. In that PR:

```sh
cargo update -p capnp -p capnp-futures -p capnp-rpc -p capnpc
```

An existing lockfile keeps whatever it already has. If crates.io `capnp-rpc`
0.26.3 is locked, it stays, and Cargo only warns `patch … was not used in the
crate graph`, which rustc's `-D warnings` doesn't catch. Check two things:
- every `capnp*` entry in `Cargo.lock` has a `git+https://github.com/mkolehmainen/capnproto-rust.git` source;
- there is no `[[patch.unused]]` section.

## When to retire the fork

When #666, or a successor, is released on crates.io:
1. Check the release builds for Windows and with `alloc` off. Those are the
   breaks `5b027f86` and `9bc9d826` fix; if they're still broken, send the
   fixes upstream first.
2. Delete core's `[patch.crates-io]`.
3. Move the four repos to that release in one round.
4. Archive this repository.
