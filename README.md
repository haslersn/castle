# castle

A door lock controller to be installed on devices with a connected GPIO
expander.

## Build

```bash
$ cargo build
```

### With Nix

```bash
$ nix-build
```

With Nix you can also easily cross-compile castle:

```bash
$ nix-build '<nixpkgs>' \
    --arg crossSystem '{ config = "aarch64-unknown-linux-gnu"; }' \
    --arg overlays '[ (self: super: { castle = super.callPackage ./. {}; }) ]' \
    -A castle
```

## Configuration

In the working directory where castle is executed, there must be a
`castle.toml` configuration file.
A good start is to copy the `castle.toml.example` from this repository.

### top-level keys

#### `expander_device =`

Path to the expander device. This is usually: `"/dev/spidev0.0"`

### [output_pins] section

#### `green_leds =`

List of output pin numbers where green LEDs are connected.
See [#leds](#leds).

#### `red_leds =`

List of output pin numbers where red LEDs are connected.
See [#leds](#leds).

#### `lock =`

Output pin number where the lock is connected.

### [input_pins] section

#### `hinge =`

Input pin number where the hinge is connected.
The hinge affects the visual feedback on the LEDs; see [#leds](#leds).

### [server] section

#### `mount_point =`

Prefix to the paths where the [REST endpoints](#rest-endpoints) are mounted.

#### `port =`

The port on which the [REST endpoints](#rest-endpoints) are served.

## REST endpoints

castle provides two rest endpoints: `/lock` and `/hinge`.
For example, with `mount_point = "/castle"` and `port = 8020`,
they're accessible at `http://localhost:8020/castle/lock` and
`http://localhost:8020/castle/hinge`, respectively.
We use these configuration values in the further examples.

We have a subsection for each type of supported request:

### `GET` request to the `/lock` endpoint

```bash
$ curl -X GET http://localhost:8020/castle/lock
{"state":"Locked","last_change":1546297200}
```

Hereby `"state"` is one of `"Locked"` and `"Unlocked"`.
`"last_change"` is the timestamp where that state changed lastly.

### `PUT` request to the `/lock` endpoint

Only putting the `"state"` field is supported.

```bash
$ curl -X PUT http://localhost:8020/castle/lock -d '{ "state": "Unlocked" }'
```

### `POST` request to the `/lock?toggle` endpoint

This toggles the `"state"`.

``` bash
$ curl -X POST http://localhost:8020/castle/lock?toggle
```

### `GET` request to the `/hinge` endpoint

```bash
$ curl -X GET http://localhost:8020/castle/hinge
{"state":"Closed"}
```

Hereby `"state"` is one of `"Closed"` and `"Open"`.

## LEDs

* If the lock is `"Unlocked"`, then only the green LEDs are on.
* If the lock is `"Locked"`:
    * If the hinge is `"Closed"`, then only the red LEDs are on.
    * If the hinge is `"Open"`, then the red and green LEDs alternate.
