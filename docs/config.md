# Configuration Reference

key-overlay-rs uses TOML for configuration. All fields are optional and will fall back to sensible defaults if omitted.

## Full Example

```toml
[general]
height = 700
keySize = 70
barSpeed = 600
backgroundColor = "0,0,0,255"
margin = 25
outlineThickness = 5
fading = true
counter = true
fps = 60
logToFile = false

[[key]]
name = "Z"
color = "255,0,0,255"
size = 1.0

[[key]]
name = "X"
color = "0,255,255,255"
size = 1.0
```

## General Settings

The `[general]` section controls overall overlay appearance and behavior.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `height` | number | `700` | Overlay window height in pixels |
| `keySize` | number | `70` | Base size of each key indicator in pixels |
| `barSpeed` | number | `600` | Animation speed for press bars (higher is faster) |
| `backgroundColor` | string | `"0,0,0,255"` | Background color as RGBA (0-255) |
| `margin` | number | `25` | Margin around the overlay in pixels |
| `outlineThickness` | number | `5` | Thickness of key outlines in pixels |
| `fading` | boolean | `true` | Enable fade-out animation after key release |
| `counter` | boolean | `true` | Show press count on each key |
| `fps` | number | `60` | Target frame rate for rendering |
| `logToFile` | boolean | `false` | Enable writing logs to rotating files under `logs/`; when `false`, logs are printed to console |

### Color Format

Colors are specified as comma-separated RGBA values in the range 0-255:

```
"red,green,blue,alpha"
```

Examples:
- `"0,0,0,255"` - opaque black
- `"255,255,255,255"` - opaque white
- `"255,0,0,128"` - semi-transparent red
- `"0,255,0,255"` - solid green

## Key Definitions

The `[[key]]` sections define which keys to monitor and how to display them. You can define as many keys as needed.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | string | required | Key name to monitor (case-sensitive) |
| `color` | string | `"255,255,255,255"` | Color for this key (RGBA format) |
| `size` | number | `1.0` | Size multiplier (1.0 = base size) |

### Supported Key Names

Key names depend on your input system and locale. Common names include:

- **Letters**: A, B, C, ..., Z
- **Numbers**: 0, 1, 2, ..., 9
- **Function keys**: F1, F2, ..., F12
- **Modifiers**: Shift, Control, Alt, Super
- **Navigation**: Left, Right, Up, Down, Home, End, PageUp, PageDown
- **Mouse**: Mouse1 (left), Mouse2 (middle), Mouse3 (right), Mouse4, Mouse5
- **Special**: Space, Enter, Tab, Escape, Backspace, Delete, Insert

If you're unsure of a key's name, start the application and press the key. Check the logs for the detected name.

## Key Configuration Examples

### Single Key

```toml
[[key]]
name = "Space"
color = "255,200,0,255"
size = 1.5
```

### Multiple Keys

```toml
[[key]]
name = "Z"
color = "255,0,0,255"
size = 1.0

[[key]]
name = "X"
color = "0,255,255,255"
size = 1.0

[[key]]
name = "C"
color = "0,0,255,255"
size = 1.0
```

### Mouse Buttons

```toml
[[key]]
name = "Mouse1"
color = "255,128,0,255"
size = 0.8

[[key]]
name = "Mouse3"
color = "0,255,128,255"
size = 0.8
```

### With Custom Size Multiplier

```toml
[[key]]
name = "Q"
color = "255,0,0,255"
size = 1.2

[[key]]
name = "E"
color = "255,0,0,255"
size = 1.2

[[key]]
name = "R"
color = "255,0,0,255"
size = 1.2

[[key]]
name = "F"
color = "255,0,0,255"
size = 0.8
```

## Default Configuration

If no configuration file exists, key-overlay-rs will create one with these defaults:

```toml
[general]
height = 700.0
keySize = 70.0
barSpeed = 600.0
backgroundColor = "0,0,0,255"
margin = 25.0
outlineThickness = 5.0
fading = true
counter = true
fps = 60
logToFile = false

[[key]]
name = "Z"
color = "255,0,0,255"
size = 1.0

[[key]]
name = "X"
color = "0,255,255,255"
size = 1.0
```

## Hot-Reload

The application watches the configuration file and automatically reloads changes without restarting. Edit your config file and save, and the overlay will update immediately.

## Validation

- `barSpeed` must be positive. If set to zero or negative, the default value (600) will be used with a warning.
- Colors must be in valid RGBA format with values 0-255.
- All keys must have a `name` field.

## Notes

- Key names are case-sensitive on most platforms.
- Some key names may vary by operating system or keyboard layout.
- Empty key lists are valid but will result in an empty overlay.
