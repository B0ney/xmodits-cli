 
## Supported Tracker Formats
| Extension | Format | 
| - | - |
| ``.it`` | Impulse Tracker* |
| ``.xm`` | Extended Module | 
| ``.s3m`` | Scream Tracker 3 |
| ``.mod`` | Amiga Pro Tracker |
| ``.mptm`` | ModPlug Tracker module* |
| ``.umx`` | Unreal Music Package (Containing above) |

\* Ripping from OpenMPT trackers is not pefect 

## Supported Exports
| Extension | Format |
|-|-|
|``.wav``| Microsoft Wave|
|``.aiff``| Audio Interchange File Format |
|``.its``| Impulse Tracker 2 sample |
|``.s3i``| Scream Tracker 3 Instrument |
|``.8svx``| 8-Bit Sampled Voice |
|``.raw``| Headerless pcm |

## How to Use

```
xmodits {ARGUMENTS} <module path> [destination folder]

e.g:
    xmodits ./test.s3m ~/Music/Samples/

    xmodits ./test.s3m

    xmodits ./mod1.mod ./mod2.it ./mod3.s3m [destination folder]

    xmodits --fmt="8svx" ./*.mod ~/Downloads/samples/8svx

```
Note: If the destination is not provided, xmodits will place the ripped samples in a self contained folder in the current working directory.

# Download
The command line version of xmodits can be downloaded [here](https://github.com/B0ney/xmodits-cli/releases).

Please have a look at [running on older systems](#running-on-older-systems).

### Additional Arguments
If you want to customize how ripped samples are named, the following arguments can be used:

|short| long| Description|
|--|--|--|
|-s |--strict| ``Enabled by default.`` Only allow files with the supported file extensions: [``it``, ``xm``, ``s3m``, ``mod``, ``umx``, ``mptm``]|
|-i |--index-only| Ripped samples will only be named with an index.|
|-r |--index-raw| Preserve internal sample indexing.|
|-p |--index-padding| Pad sample index with preceding zeros. 0-1 will disable padding.|
|-n |--no-folder| Do not create a new folder for ripped samples.|
|-u |--upper| Name samples in upper case. |
|-l |--lower| Name samples in lower case. |
|-g |--prefix| Prefix samples with the tracker's filename. |
|-f|--fmt| Export samples to the following formats: [ ``wav (default)``, ``aiff``, ``8svx``,``its``,``s3i``, ``raw`` ]|
||--info| Print information about a tracker module. |

## Building

```shell
cargo build --release
```

Compile with build metadata embedded in the binary (enables ``--Meta`` flag), **This will take longer to compile**:
```shell
cargo build --release --features="with_metadata"
```

(**Experimental**) Compile with multi-threading:
```shell
cargo build --release --features="rayon"
```

## Running on older systems
<!--Unlike its [big brother](https://github.com/B0ney/xmodits), xmodits-cli will always run in a single thread
(unless you compile with ``--features="rayon"``).-->

### Windows
Windows 7 is the minimum OS that could run xmodits. I tried to get Windows XP working sorry :(

xmodits is compiled with vcruntime (msvc only) embedded, so it should run out of the box.

### Linux
**It is recommended that you download the ``*-musl`` variant.**

Older linux sytems may have glibc versions too old to run the program.

You'll most likely get an error like this if you attempt to run the ``-gnu`` variant:
```
libc.so.6: version `GLIBC_2.18` not found (required by ./xmodits)
```

## Licence
xmodits-cli is licensed under the LGPLv3

