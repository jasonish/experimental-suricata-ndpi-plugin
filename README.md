# Experimental Suricata NDPI Plugin

## Building

First initialized sub-modules to pull down the nDPI source:

```
git submodule update --init --recursive
```

Make sure you have Suricata headers installed. From your Suricata 8.0 source directory, run:

```
make install-headers
```

Then to build:

```
SURICATA_INCLUDE_DIR=/usr/include/suricata cargo build --release
```

For example, if you build and installed Suricata with `--prefix=/opt/suricata/8.0.4` then you might
use `SURICATA_INCLUDE_DIR=/opt/suricata/8.0.4-dev/include/suricata`.

## Install

Copy ./target/release/libndpi.so somewhere and let Suricata know about it in your Suricata
configuration file:

```yaml
plugins:
  - /usr/lib/suricata/plugins/libndpi.so
```
