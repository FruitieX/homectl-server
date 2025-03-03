# homectl

Discord: https://discord.gg/xP2s6EY8rd

🚧 WORK IN PROGRESS 🚧

Note: this project is still in quite early stages. Regardless, I've been using
this as a daily driver for over a year now. This is also my first "real" Rust
project, which brings with it the usual caveats. Luckily refactoring Rust code
is a fairly pleasant experience.

If you're not ready to get your hands dirty with Rust code, I would suggest
trying out other alternatives for now.

### Quick start

- Install the Rust toolchain using [`rustup`](https://rustup.rs/)
- Clone this repository
- Run `RUST_LOG=homectl_server=info cargo run`

You should now have a demo/dummy homectl environment running.

To control your home automation systems, you need to edit the generated
configuration file in `/Settings.toml`. See below sections for
configuration instructions and examples.

## Description

This project aims to unify home automation (HA) systems from different
brands, and does so by assuming complete control over the individual systems.
It brings some features that I felt are missing from consumer HA systems,
and also other similar solutions to homectl:

- A common interface for configuring everything in one place (plaintext config file for now).

  - (Hopefully) no more figuring out obscure schedule/rule/condition/action
    configuration that vary per HA supplier. (Instead you have homectl's
    obscure configuration file format for now, but this will be improved upon
    later! :-))

- Allow complete control of actions performed when sensors/buttons are triggered.

  - Because homectl only reads sensor values from HA systems, we are not
    limited by what actions can be programmed into the individual HA system
    controllers.

  - For example, you can put your computer to sleep/wake when you turn off/on
    the lights to your office.

  - Or you could start a robot vacuum when leaving your home between certain
    times of the day.

  - You can also control other manufacturers devices than the one that made the
    light switch you pressed

- Don't trust that the HA systems always do what you want

  - Some HA systems are not as reliable as you would hope, and may for example
    miss a command that you send them.

  - Or a device might simply forget its state due to an accidental power cycle.

  - Due to this, homectl will keep track of the expected state for each device,
    and actively poll devices for their current state, automatically correcting
    any incorrect state it might find.

- An advanced scenes system allow controlling a large amount of devices to preset states.

  - Because homectl keeps track of a device's active scene, we can perform
    certain actions only when a device is in a certain scene. For example, we
    can bind a light switch to multiple scenes and cycle between the scenes.

  - Scenes may "link" state from other devices: "go look up what the state of
    this device is and copy the state from there".

  - These devices can be "virtual" devices, such as a device that returns the
    approximate color of the sky.

  - Combine these facts and you can e.g. have your lights smoothly follow a
    circadian rhythm. These transitions will be so smooth that you won't
    notice them. Every time homectl polls your lights their expected state is
    calculated and compared against the actual state. If the difference is
    large enough (still imperceptibly small), then homectl will update the
    lights to match the expected state.

## Setup

### Environment variables (optional):

- `DATABASE_URL`: Used by the backend to connect to a PostgreSQL database. If
  not provided, functionalities requiring a database connection will be
  disabled.

  For example: `DATABASE_URL=postgres://user:password@localhost/homectl`

### Database setup (optional)

- Install PostgreSQL.
- Set `DATABASE_URL` according to above instructions.
- Run migrations:
  - `cargo install sqlx-cli`
  - `cd backend`
  - `sqlx database create`
  - `sqlx migrate run`

## Sample configs for supported integrations:

You can refer to the [sample config](/Settings.toml.example) for an
example on how to put these concepts together.

### MQTT

Sample configuration:

```
[integrations.example]
plugin = "mqtt"
host = "mqtt.example.org"
port = 1883

# Note the + sign, which acts as wildcard in MQTT topics.
# This will subscribe to both light and sensor updates.
topic = "home/+/example/{id}"
topic_set = "home/lights/example/{id}/set"
```

Example of a supported MQTT message using the default settings:

```
{
  "id": "fd2a1757-931b-4e44-b8f9-2fc8981626c1",
  "name": "Kitchen table",
  "power": true,
  "brightness": 0.252,
  "color": {
    "x": 0.5574,
    "y": 0.3919
  },
  "capabilities": {
    "xy": true,
    "ct": {
      "start": 2000,
      "end": 6535
    }
  }
}
```

Optionally, you can change the shape of read/written MQTT messages by setting
the following fields to valid [JSON
pointers](https://datatracker.ietf.org/doc/html/rfc6901):

```
[integrations.example]

...

id_field = "/id"
name_field = "/name"
color_field = "/color"
power_field = "/power"
brightness_field = "/brightness"
sensor_value_field = "/sensor_value"
transition_field = "/transition"
capabilities_field = "/capabilities"
```

### Neato

```
[integrations.neato]
plugin = "neato"
email = "example@gmail.com"
password = "your-password-here"
cleaning_days = ["Mon", "Wed", "Fri"]
cleaning_time_start = "08:00"
cleaning_time_end = "19:00"

# Set to true for debugging
dummy = false

# Example routine that starts your Neato robot(s)
[routines.leave_home]
name = "Leave home"
rules = [
  { integration_id = "hue1", name = "Entryway switch button 4", state = { value = true } }
]
actions = [
  { action = "IntegrationAction", integration_id = "neato", payload = "clean_house" },
]
```

### Wake on LAN

```
# Wake up your PC:s by MAC address, and optionally put them to sleep!
# Make sure you've set up Wake on LAN on your PC.
# (optional) Set up sleep on lan: https://github.com/SR-G/sleep-on-lan
[integrations.wol]
plugin = "wake_on_lan"
machines = [
  { id = "office_pc", mac = "DE:AD:BE:EF:12:34", sleep_on_lan = "http://192.168.1.123:8009/sleep" }
]

# Example of a scene that turns on PC via WOL
[scenes.office]
name = "Office devices"

  [scenes.office.devices.wol]
  office_pc = { power = true }
```

## Configuration tips / "recipes"

### Group lights to control multiple lights at once:

```
[groups.entryway]
name = "Entryway"
devices = [
  { integration_id = "hue", name = "Entryway spot 1" },
  { integration_id = "hue", name = "Entryway spot 2" },
  { integration_id = "lifx", name = "Entryway table lamp" },
]
```

### Combine groups into larger groups:

```
[groups.downstairs]
name = "Downstairs"
groups = [
  { group_id = "entryway" },
  { group_id = "kitchen" },
  { group_id = "office" },
]
```

I would suggest creating at least an "All" group containing all your devices.

### Create scenes for setting lights to preset states:

```
[scenes.normal_downstairs]
name = "Normal (downstairs only)"

  # Add as many groups as you want
  [scenes.normal_downstairs.groups]
  downstairs = { power = true, color = { h = 42, s = 1.0 }}
  outdoor = { power = true, color = { r = 255, g = 255, b = 255 }, brightness = 0.75 }

  # You can add devices directly per integration and device name, too
  [scenes.normal_downstairs.devices.hue]
  "Hue lightstrip" = { power = true, color = { h = 263, s = 1.0 } }
```

### Combine scenes into larger scenes:

```
[scenes.normal]
name = "Normal"

  # You can make a scene group/device use the state that it would have in
  # another scene
  [scenes.normal.groups]
  downstairs = { scene_id = "normal_downstairs" }
  upstairs = { scene_id = "normal_upstairs" }
  outdoor_spots = { scene_id = "outdoor" }
```

This is a bit of a niche feature, but I use it to create a scene for the entire
house without needing to duplicate the config of contained scenes.

### Make lights follow a fake circadian rhythm:

```
[integrations.circadian]
plugin = "circadian"
device_name = "Circadian rhythm"
day_color = { h = 25, s = 0.35 }
day_fade_start = "04:00"
day_fade_duration_hours = 4
night_color = { h = 20, s = 0.95 }
night_fade_start = "18:00"
night_fade_duration_hours = 3

[scenes.normal]
name = "Normal"

  # You can make a scene group/device use the state of another device
  [scenes.normal.groups]
  all = { integration_id = "circadian", device_id = "color" }
```

### Make a light switch activate a scene:

```
[routines.arrive_home]
name = "Arrive home"
rules = [
  { integration_id = "hue", name = "Entryway switch button 1", state = { value = true } }
]
actions = [
  { action = "ActivateScene", scene_id = "normal_downstairs" },
]
```

### Make a light switch cycle through a list of scenes:

```
[routines.favorite_scenes]
name = "Cycle through favorite scenes"
rules = [
  { integration_id = "hue1", name = "Living room switch button 1", state = { value = true } }
]
actions = [
  { action = "CycleScenes", scenes = [ { scene_id = "normal" }, { scene_id = "bright" } ] },
]
```

### Make a light switch dim/brighten lights:

```
# Brighten
[routines.brighten]
name = "Brighten"
rules = [
  { integration_id = "hue1", name = "Living room switch button 2", state = { value = true } }
]
actions = [
  { action = "DimAction", step = -0.1 },
]

# Dim
[routines.dim]
name = "Dim"
rules = [
  { integration_id = "hue1", name = "Living room switch button 3", state = { value = true } }
]
actions = [
  { action = "DimAction", step = 0.1 },
]
```

### Temporarily disable a motion detector when leaving the house:

```
[integrations.entryway_timer]
plugin = "timer"
device_name = "Entryway timer"

# This routine is triggered and turns on the lights when a motion sensor detects
# movement, and `entryway_timer` is not running.
[routines.entryway]
name = "Entryway"
rules = [
  { integration_id = "hue1", name = "Entryway motion sensor", state = { value = true } },
  { integration_id = "entryway_timer", name = "Entryway timer", state = { value = false } },
]
actions = [
  { action = "ActivateScene", scene_id = "normal_downstairs" },
  { action = "ActivateScene", scene_id = "outdoor" }
]

# This routine is triggered when switching off all lights via a switch located
# in the entryway. It activates a scene that turns off all lights, and starts
# `entryway_timer` with a timeout of 300 seconds.
[routines.leave_home]
name = "Leave home"
rules = [
  { integration_id = "hue1", name = "Entryway switch button 4", state = { value = true } }
]
actions = [
  { action = "ActivateScene", scene_id = "leave" },
  { action = "IntegrationAction", integration_id = "entryway_timer", payload = "300000" }

  # Clean the house too!
  { action = "IntegrationAction", integration_id = "neato", payload = "clean_house" },
]
```

### Enable a motion sensor only when lights controlled by it are in a certain state:

```
# Activates `normal_upstairs` scene when motion is detected in staircase only if
# all upstairs lights are off
[routines.staircase_upstairs]
name = "Staircase (upstairs)"
rules = [
  { integration_id = "hue1", name = "Staircase motion sensor", state = { value = true } },
  { group_id = "upstairs", power = false }
]
actions = [
  { action = "ActivateScene", scene_id = "normal_upstairs" },
]

# Activates `normal_downstairs` scene when motion is detected in staircase only
# if all downstairs lights are off
[routines.staircase_downstairs]
name = "Staircase (downstairs)"
rules = [
  { integration_id = "hue1", name = "Staircase motion sensor", state = { value = true } },
  { group_id = "downstairs", power = false }
]
actions = [
  { action = "ActivateScene", scene_id = "normal_downstairs" }
]
```

My motivation for this setup is that the only task my motion sensor should
perform is turning on the lights if they were previously off.

For instance if I have manually enabled another scene, I don't want that scene
overwritten every time someone uses the stairs. Or if I'm setting the colors of
my lights through the homectl UI, I don't want the changes to be lost whenever I
walk past a motion detector.

### Development notes

You can test features without access to physical hardware with configs such as:

```
[integrations.mqtt]
plugin = "mqtt"
host = "localhost"
port = 1883
topic = "home/devices/adb/{id}"
topic_set = "home/devices/adb/{id}/set"

[integrations.dummy]
plugin = "dummy"

[integrations.dummy.devices.sensor]
name = "Test sensor"
init_state = { Sensor = { OnOffSensor = { value = false } } }

[routines.test]
name = "Test routine"
rules = [
  { integration_id = "dummy", name = "Test sensor", state = { value = true } }
]
actions = [{ action = "Custom", integration_id = "mqtt", payload = '{ "topic": "home/devices/adb/android-tv/set", "json": "{ \"power\": false }" }' }]
```

Now you can test the `Test routine` by toggling the dummy sensor on/off over HTTP:

```
xh PUT localhost:45289/api/v1/devices/sensor id=sensor name="Test sensor" integration_id=dummy state:='{ "Sensor": { "OnOffSensor": { "value": true }}}'
```

```
xh PUT localhost:45289/api/v1/devices/sensor id=sensor name="Test sensor" integration_id=dummy state:='{ "Sensor": { "OnOffSensor": { "value": false }}}'
```