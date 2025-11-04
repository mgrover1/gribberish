# gribberish

## Publishing on this branch

To publish a new version:
1. Update version numbers in Cargo.toml files
2. Commit and push changes
3. Go to https://github.com/mgrover1/gribberish/releases/new
4. Create a new release (e.g., tag v0.23.0)
5. The workflows will automatically build and publish to crates.io and PyPI

This gives you more control - no accidental publishes from just pushing tags!

## More Docs/Info

Read [GRIB 2](https://en.wikipedia.org/wiki/GRIB) files with Rust.

See [`gribberish`](gribberish/README.md) for the core library

See [`python`](python/README.md) for usage with `python` and `xarray`

See [`node`](node/README.md) for usage with `nodejs`

## License

[MIT](LICENSE) -  2023 Matthew Iannucci
