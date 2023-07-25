 
## Supported Tracker Formats
| Extension | Format | 
| - | - |
| ``.it`` | Impulse Tracker* |
| ``.xm`` | Extended Module | 
| ``.s3m`` | Scream Tracker 3 |
| ``.mod`` | Amiga Pro Tracker |
| ``.mptm`` | ModPlug Tracker module* |
| ``.umx`` | Unreal Music Package (Containing above) |

## Supported Exports
| Extension | Format |
|-|-|
|``.wav``||
|``.aiff``||
|``.its``||
|``.s3i``||
|``.8svx``||
|``.raw``||

## How to Use

```
xmodits {ARGUMENTS} <module path> [destination folder]

e.g:
    xmodits ./test.s3m ~/Music/Samples/

    xmodits ./test.s3m

    xmodits ./mod1.mod ./mod2.it ./mod3.s3m [destination folder]

    xmodits --fmt="8svx" ./*.mod ~/Downloads/samples/8svx

```
If the destination is not provided, xmodits will place the ripped samples in a self contained folder in the current working directory.

### Additional Arguments
If you want to customize how ripped samples are named, the following arguments can be used:

|short| long| Description|
|--|--|--|
|-s |--strict| ``Enabled by default.`` Only allow files with the supported file extensions: [it, xm, s3m, mod, umx, mptm]|
|-i |--index-only| Ripped samples will only be named with an index.|
|-r |--index-raw| Preserve internal sample indexing.|
|-p |--index-padding| Pad sample index with preceding zeros. 0-1 will disable padding.|
|-n |--no-folder| Do not create a new folder for ripped samples.|
|-u |--upper| Name samples in upper case. |
|-l |--lower| Name samples in lower case. |
|-g |--prefix| Prefix samples with the tracker's filename. |
|-f|--fmt| Export samples to the following formats: [ ``wav (default)``, ``aiff``, ``8svx``, ``raw`` ]|
||--info| Print information about a tracker module. |

## Licence
xmodits-cli is licensed under the LGPLv3
