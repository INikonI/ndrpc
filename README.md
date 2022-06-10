# ndrpc
Light Crossplatform Discord Rich Presence Client.

## Configuration

### Config
config is `.yaml` file with following content
```yaml
# application id from discord developers portal.
app_id: 983347731823210567

type: "CustomStatic" # CustomStatic | CustomDynamic | SystemInfo

# name of preset file for CustomStatic presence type.
static_preset_name: "some_preset"

# names of preset files for CustomDynamic presence type.
# presets swaps in the same order, from top to bottom.
dynamic_preset_names:
  - "other_preset"
  - "some_preset"

# show elapsed time from start in presence or not.
with_elapsed_time: true # true | false
```
#### ndrpc has 3 presence types:
- `CustomStatic` - Full custom presence which sets preset once
- `CustomDynamic` - Full custom presence which changes preset every 4 seconds
- `SystemInfo` - Presence which updates every 4 secs with system information and with assets and buttons from static preset

### Presets
Preset is `.yaml` file in `/presets` folder with following content
```yaml
details: "Some details" # text
state: "Some state" # text

assets:
  large_image: "ndrpc_logo" # url or image key
  large_text: "Text of large image" # text
  small_image: "ndrpc_logo" # url or image key
  small_text: "Text of small image" # text

# presence can have maximum 2 buttons
buttons:
  - label: "Github" # text
    url: "https://github.com/INikonI/ndrpc/" # url
  - label: "Discord" # text
    url: "https://discord.com" # url
```
In presets you can specify only you need, for example
```yaml
details: "Rich Presence Client"

assets:
  large_image: "https://media.giphy.com/media/gQ2ctJip0lURTLvrLt/giphy.gif"
  large_text: "Rumia flex"

buttons:
  - label: "Github" # max length is 32
    url: "https://github.com/INikonI/ndrpc/"
```

## Usage
After configuration your folder must be looks like this (In the archive with ndrpc there is already a config and basic presets)

![image](https://user-images.githubusercontent.com/47792796/172809673-c0ba5373-c6be-4ba3-a777-d19314844742.png)

And stays just run `ndrpc` file.
