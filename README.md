# vlc-rs [![Build Status](https://travis-ci.org/garkimasera/vlc-rs.svg?branch=master)](https://travis-ci.org/garkimasera/vlc-rs)

Rust bindings for libVLC media framework.
Fork from [vlc-rs](https://code.videolan.org/videolan/vlc-rs)

## Status

Many missing functions and wrappers.

## Use

Please add the following dependencies to your Cargo.toml.

```Toml
[dependencies]
vlc-rs = "0.3"
```

Or:

```Toml
[dependencies.vlc-rs]
git = "https://github.com/garkimasera/vlc-rs.git"
```

## Example

Play for 10 seconds from a media file.

```Rust
extern crate vlc;
use vlc::{Instance, Media, MediaPlayer};
use std::thread;

fn main() {
    // Create an instance
    let instance = Instance::new().unwrap();
    // Create a media from a file
    let md = Media::new_path(&instance, "path_to_a_media_file.ogg").unwrap();
    // Create a media player
    let mdp = MediaPlayer::new(&instance).unwrap();
    mdp.set_media(&md);

    // Start playing
    mdp.play().unwrap();

    // Wait for 10 seconds
    thread::sleep(::std::time::Duration::from_secs(10));
}
```

Other examples are in the examples directory.

## Building

### Windows

To build `vlc-rs`, you must either build VLC from source or grab one of the pre-built packages from [videolan.org](https://www.videolan.org/vlc/download-windows.html).

If you're building for `x86_64`, then you should grab the download labelled "Installer for 64bit version".
That installer is actually a self-extracting ZIP archive, so we can extract the contents without installing VLC itself.

If you're building for `x86`, then you should either download labelled "7zip package" or the one labelled "Zip package".

Once you've downloaded your chosen package, you should extract it some place such that its path contains no spaces.
To point `vlc-rs` at your VLC package, you should set an appropriate environment variable:

- `VLC_LIB_DIR_WIN`: Directory of the VLC package (preferred when `HAS_PKG_CONFIG=false`)
- `VLC_LIB_DIR`: Directory of the VLC package, any architecture (fallback)
- `VLC_LIB_DIR_X86` : Directory of the VLC package, `x86`-only
- `VLC_LIB_DIR_X86_64` : Directory of the VLC package, `x86_64`-only

You should also add the package to your `PATH` variable if you intend to run the program.
For distribution of an executable program, you should probably copy over the neccessary DLLs, as well as the `plugins` directory.

### Building without pkg-config (all platforms)

By default the build relies on `pkg-config` to discover `libvlc`. If your toolchain
does not have `pkg-config` available (for example when embedding `vlc-rs` in a
bundle that ships its own VLC), you can opt-out by setting:

```bash
HAS_PKG_CONFIG=false
```

When `HAS_PKG_CONFIG` is `false`, the build script will stop probing `libvlc.pc`
and instead look for the library under the following environment variables,
depending on the target OS:

- `VLC_LIB_DIR_MACOS` (macOS builds, falling back to `VLC_LIB_DIR`)
- `VLC_LIB_DIR_LINUX` (Linux builds, falling back to `VLC_LIB_DIR`)
- `VLC_LIB_DIR_WIN` (Windows builds, falling back to the legacy variables listed above)
- `VLC_LIB_DIR` (final fallback for any platform)

Each variable should contain the directory that holds the platform-specific `libvlc`
binary (`.dylib`, `.so`, or `.dll`).

## License

MIT (Examples are licensed under CC0)
