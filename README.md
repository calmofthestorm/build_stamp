# Introduction

At compile time, writes attributes such as the git revision, whether the
working directory is clean, git status, build time, etc to a generated file.
This is then available to the binary, for example in the version information.

Due to the integration with git itself, albeit in a simple way, it is hard for
me to include an example of how to use it in this repository. Instead, I refer
to https://github.com/calmofthestorm/eseb as a simple binary which uses this.

The short of it is that you need to add `build_stamp` to both `dependencies`
and `build-dependencies` in `Cargo.toml` as so:

```
[dependencies]
build_stamp = "1"

[build-dependencies]
build_stamp = "1"
```

and then add the following include for the generated code to `lib.rs`,
`main.rs`, or a submodule thereof:

```
include!(concat!(env!("OUT_DIR"), "/generated_stamp.rs"));
```

You can then access the `BUILD_STAMP` constant. The below example is using `clap` to define a command-line interface that includes build information in the version:

```
let matches = App::new("eseb")
    .name(env!("CARGO_PKG_NAME"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .version(format!("{} ({})", env!("CARGO_PKG_VERSION"), BUILD_STAMP.git_revision_cleanness()).as_ref())
```

# Limitations

Be aware that Cargo may not always run `build.sh` when you might like. For
example, after you commit, you would expect that if your working directory is
clean, a build would be tagged as clean. If you haven't changed any files,
however, Cargo will continue to use the previously generated stamp.

You can fix this by touching `build.rs` manually, or using the (included) post
commit hook. There may be other similar edge cases I haven't thought of.

## Security

I don't recommend relying on something this basic for any sensitive
application, such as anything related to attestation or integrity verification.
My purpose in creating this is simply to have a better understanding of my own
binaries that are here and there when there is an error or similar. As such,
"probably approximately good enough" is good enough for me:-)
