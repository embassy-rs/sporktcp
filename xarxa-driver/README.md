# xarxa-driver

Device driver trait for the [`xarxa`](https://github.com/embassy-rs/xarxa) TCP/IP network stack.

This crate contains only the [`Device`] trait and supporting types. It is deliberately tiny, and  is versioned independently from `xarxa` itself. 

The goal is that a single major version of `xarxa-driver` can be used by many major versions of `xarxa`. This way, a new `xarxa` release instantly works with the ecosystem without 

Driver crates should depend on `xarxa-driver` and implement [`Device`]. They should *not*
depend on `xarxa`.

## Stability

To keep this crate stable, it deliberately does *not* contain:

- Anything that varies with `xarxa`'s Cargo features. In particular, enums here always have
  all their variants, because adding or removing an enum variant is a breaking change,
  unlike adding a field to a `#[non_exhaustive]` struct.
- Time types. Drivers are not given a timestamp; hardware timestamping is reported through
  [`PacketMeta`] instead.
- Packet parsing, addresses, or anything else from `xarxa::wire`.

## License

This crate is 0-clause BSD licensed, see the `LICENSE-0BSD.txt` file at the root of the
repository.
