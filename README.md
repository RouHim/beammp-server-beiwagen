# beammp-server-beiwagen

This application keeps your **client mods** of the beammp server up to date.

## Motivation

Everybody knows it, everybody hates it; as soon as you have downloaded the mods they are already out-of-date.

### Configuration

The configuration can either be done via command line arguments, environment variables or a configuration file.

The order of precedence is as follows:

1. Command line arguments
2. Environment variables
3. Configuration file

The mods can be defined as a list of mod ids or URLs.

#### Command line arguments

Use the `--help` argument to get a list of all available command line arguments.

#### Environment variables

| Name               | Description                                                                       | Example                                                                             |
|--------------------|-----------------------------------------------------------------------------------|-------------------------------------------------------------------------------------|
| BW_CLIENT_MODS_DIR | Mandatory! Folder where BeamMP client mods should be downloaded to.               | `/beammp/Resources/Client`                                                          |
| BW_MODS            | Mandatory! List of mod ids to download and keep track of. See: How to find mod id | `20231,19639,https://www.beamng.com/resources/ibishu-pessima-awd-turbo.30372/,6546` |
| BW_OUTDATED        | Specify how to handle outdated mods - check explanation below                     | `skip`                                                                              |
| BW_UNSUPPORTED     | Specify how to handle unsupported mods - check explanation below                  | `delete`                                                                            |

#### Configuration file

The configuration file is a simple `toml` file.
The file must be named `beiwagen.toml` and must be located in the same directory as the executable.
This is an example of a configuration file:

```toml
client_mods_dir = "/path/to/BeamNG.drive/client-mods"
outdated = "skip"
unsupported = "delete"
mods = [
    "https://www.beamng.com/resources/sic_igct-powertrain-kit.30373/",
    "https://www.beamng.com/resources/used-car-generator.30414/",
    "https://www.beamng.com/resources/ibishu-pessima-awd-turbo.30372/",
    "65165",
    "9082",
]
```

### How to use

Each mod on [beamng.com/resources](https://beamng.com/resources) can be marked if it is **unsupported** or **outdated**.
The following logic can affect the automation behavior - For the parameters `BW_OUTDATED` and `BW_UNSUPPORTED` the
following values are available:

* `<empty>` - Nothing special will happen with outdated or unsupported mods
* `skip` - Skip the download of an outdated or unsupported mod
* `delete` - Skip the download of an outdated or unsupported mod and delete it locally

### How to find mod id

1. Navigate to any mod on [beamng.com/resources](https://beamng.com/resources)
2. Copy the last number of the URL e.g.: `https://www.beamng.com/resources/1jz-gte-vvti-swap-for-the-ibishu-saga.20231`
   . **20231** is your mod id.

## Known problems

If a mod maker produces a new version of his mod, and updates the new version only online on the beamng resource page,
but not in the zip file that is downloaded, you have to download the mod again every time.
