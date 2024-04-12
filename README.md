# beammp-server-beiwagen

This application keeps your **client mods** of the beammp server up to date.

## Motivation

Everybody knows it, everybody hates it; as soon as you have downloaded the mods they are already out-of-date.

## Usage

It can download and update all of your desired mods, defined as environment variables.

### Parameter

| Name               | Description                                                                       | Default value | Example                    |
|--------------------|-----------------------------------------------------------------------------------|---------------|----------------------------|
| BW_CLIENT_MODS_DIR | Mandatory! Folder where BeamMP client mods should be downloaded to.               | `<empty>`     | `/beammp/Resources/Client` |
| BW_MODS            | Mandatory! List of mod ids to download and keep track of. See: How to find mod id | `<empty>`     | `20231,19639,20292`        |
| BW_OUTDATED        | Specify how to handle outdated mods - check explanation below                     | `<empty>`     | `skip`                     |
| BW_UNSUPPORTED     | Specify how to handle unsupported mods - check explanation below                  | `<empty>`     | `delete`                   |

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
