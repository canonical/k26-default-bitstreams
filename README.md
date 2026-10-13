# k26-default-bitstreams

This repo is for a snap application which connects with fpgad to load a bitstream to the fpga0 device.
The contained bitstream is a fan controller for the `K*26*` series of Xilinx boards.

If running on target device:
```shell
snapcraft
sudo snap install k26-default-bitstreams..._arm64.snap
sudo snap connect k26-default-bitstreams:fpgad-dbus fpgad:daemon-dbus
```

NB: the `fpgad:daemon-dbus` is external to this repo so may be subject to change. Check [fpgad's snapcraft.yaml](https://github.com/canonical/fpgad/blob/main/snap/snapcraft.yaml) for changes if this command fails.

## snapcraft.yaml explained

### plugs and slots

The `plugs` entry here allows the connection to be made between this snap and the fpgad daemon
```yaml
plugs:
  fgpad-dbus:
    interface: dbus
    bus: system
    name: com.canonical.fpgad
```
but it must also be added to the application:
```yaml
apps:
  k26-default-bitstreams:
    command: bin/k26-default-bitstreams
    daemon: oneshot
    plugs:
      - fpgad-dbus
```
here `daemon: oneshot` means "run once on startup and then it is finished".

The `slots` entry here allows the snap to provide bitstreams and content to the system and other snaps via the `k26-default-bitstreams:provided-content` slot:
```yaml
slots:
  provided-content:
    interface: content
    content: provider-content
    source:
      read:
        - $SNAP/data/k26-starter-kits
```
This should be connected to fpgad by calling `snap connect fpgad:provider-content k26-default-bitstreams:provided-content` after installation. Note that all files within the specified directory will be made available to the consumer snap, so the `override-build` step in the `bitstream-data` part must ensure that only the intended files are copied to the `$SNAPCRAFT_PART_INSTALL/data/k26-starter-kits/` directory.

It is mandatory that the `content:` key matches with FPGAd's definition i.e. `content: provider-content` cannot be changed. The only exception is in the case that the name of the slot is `provider-content:` and content is not specified - if not specified, the `content:` value defaults to the slot name.

### parts 

The parts section describes how to form the snap package:

```yaml

parts:
  version:
    plugin: nil
    source: .
    build-snaps:
      - jq
    override-pull: |
      craftctl default
      cargo_version=$(cargo metadata --no-deps --format-version 1 | jq -r .packages[0].version)
      craftctl set version="$cargo_version+git$(date +'%Y%m%d').$(git describe --always --exclude '*')"
  k26-default-bitstreams:
    plugin: rust
    source: .
    rust-path:
      - k26-default-bitstreams
  bitstream-data:
    plugin: dump
    source: https://github.com/Xilinx/kria-base-firmware
    source-type: git
    build-packages:
      - xilinx-bootgen
    override-build: |
      bootgen -image k26_starter_kits/k26_starter_kits.bif -arch zynqmp -o k26_starter_kits/k26_starter_kits.bit.bin -w
      mkdir -p $SNAPCRAFT_PART_INSTALL/data/k26-starter-kits
      cp k26_starter_kits/k26_starter_kits.bit.bin $SNAPCRAFT_PART_INSTALL/data/k26-starter-kits/
      cp LICENSE-BINARIES $SNAPCRAFT_PART_INSTALL/data/k26-starter-kits/
```
Here `version` just runs a simple script to generate a unique version string, `k26-default-bitstreams` part defines how to build the rust package which creates the `bin/k26-default-bitstreams` used in the app section and `bitstream-data` clones a remote repository, builds the bitstream files, and makes the `k26_starter_kits.bit.bin` and `LICENSE-BINARIES` files available from the snap root at `$SNAP/data`. See [the snapcraft docs on package versioning](https://documentation.ubuntu.com/snapcraft/stable/how-to/crafting/configure-package-information/) for more information. The content interface (described above) is used to make the bitstream file available to fpgad and other snaps, but the `k26-default-bitstreams` app can also access it directly from the snap's own data directory if needed.

## fancontrol

The fan controller in the example bitstream does not do anything on its own; instead, a userspace application must interact with it by setting the PWM duty cycle using the Linux `hwmon` interface. In order to demonstrate how this works and to effectively test the bitstream, `fancontrol` is bundled with this snap.

`fancontrol` adjusts the fan's PWM duty cycle according to the ZynqMP's AMS temperature sensor through Linux's hwmon interface. The snap runs `usr/sbin/fancontrol` as a simple daemon, initially disabled.

The daemon requires the `zynqmp-hwmon` plug (`interface: custom-device`) to be connected to a matching slot in the Ubuntu Core gadget snap that permits reading the temperature sensor and accessing the fan's PWM control. Once connected, the service will be enabled automatically.

Connect the interface with the following command on Ubuntu Core:

```shell
snap connect k26-default-bitstreams:zynqmp-hwmon amd-kria:zynqmp-hwmon-slot
```

#### Configuration

In order to make testing more flexible, the `fancontrol` settings can be changed with `snap set`. All values are integers; temperatures are in degrees Celsius and PWM values range from 0 to 255.

| Setting | Description |
| --- | --- |
| `interval`| Time between temperature checks, in seconds (at least 1). |
| `mintemp` | At or below this temperature, use `minpwm`. |
| `maxtemp` | At or above this temperature, use `maxpwm`. |
| `minstart`| PWM applied briefly to start a stopped fan when cooling is needed. |
| `minstop` | Minimum running PWM used as the start of the temperature-based ramp. |
| `minpwm`  | PWM used at low temperatures; the default leaves this fan off. |
| `maxpwm`  | Maximum PWM used at high temperatures. |

Between `mintemp` and `maxtemp`, PWM increases linearly from `minstop` to `maxpwm`. 

The snap configure hook validates and updates the configuration automatically, then restarts the service if it is already running. For example:

```shell
sudo snap set k26-default-bitstreams interval=5 mintemp=40 maxtemp=70
```

Refer to the `fancontrol` man page for more information on these parameters.

# Licenses

The source code here is distributed under the GPL-3.0-only licence provided in the repository root's `LICENSE` file.

The bitstream packaged in the snap is distributed under a binary only license, provided in the `$SNAP/data/k26-starter-kits/LICENSE-BINARIES` file.
