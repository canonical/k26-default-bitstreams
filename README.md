# k26-default-bitstreams

examples and tests for fpgad snaps

If running on target device:
```shell
snapcraft
sudo snap install k26-default-bitstreams..._arm64.snap
sudo snap connect k26-default-bitstreams:fpgad-dbus fpgad:dbus-daemon
```

NB: the `fpgad:dbus-daemon` is external to this repo so may be subject to change. Check [fpgad's snapcraft.yaml](https://github.com/canonical/fpgad/blob/main/snap/snapcraft.yaml) for changes if this command fails.

## snapcraft.yaml explained

The `plugs` entry here allows the connection to be made between this snap and the fpgad daemon
```
plugs:
  fgpad-dbus:
    interface: dbus
    bus: system
    name: com.canonical.fpgad
    default-provider: fpgad
```
but it must also be added to the application:
```yaml
apps:
  k26-default-bitstreams:
    command: bin/k26-default-bitstreams
    daemon: simple
    install-mode: enable
    plugs:
      - fgpad-dbus 
    environment:
      RUST_LOG: trace <- allows printing of all trace information, remove to use "info" default
``` 
where here the `daemon: simple` means "run on startup". and `install-mode: enable` means it is enabled as a startup service at time of install. 

The parts section describes how to form the snap package
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
    source: ./data/
    source-type: local
    organize:
      default-bitstreams: data/k26-starter-kits
```
Here `version` just runs a simple script, `default-bitstream` defines how to build the rust package which creates the `bin/default-bitstream` used in the app section and `bitstream-data` makes a copy of the data folder available from the snap root. 