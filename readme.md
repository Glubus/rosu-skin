# rosu-skin

A Rust library for parsing osu! skin.ini files.

## Overview

This library provides a complete parser for osu! skin configuration files (skin.ini), supporting all sections and properties defined in the [osu! skinning documentation](https://osu.ppy.sh/wiki/en/Skinning/skin.ini).

## Features

- ✅ **General Section**: Complete support for all general skin settings
  - Cursor settings (centre, expand, rotate, trail rotate)
  - Slider settings (ball flip, ball tint)
  - Combo burst settings
  - Spinner settings
  - Animation framerate
  - Hit circle overlay settings
  - Layered hit sounds

- ✅ **Colours Section**: Full RGB and RGB(a) color support
  - Combo colors (Combo1-8)
  - Slider colors (ball, border, track override)
  - Menu glow, input overlay text
  - Song select text colors
  - Spinner background, star break additive

- ✅ **Fonts Section**: Font configuration
  - Hit circle prefix and overlap
  - Score prefix and overlap
  - Combo prefix and overlap

- ✅ **CatchTheBeat Section**: osu!catch specific settings
  - HyperDash colors (with fallback support)
  - HyperDashFruit colors
  - HyperDashAfterImage colors

- ✅ **Mania Section**: Complete osu!mania support
  - Multi-keycount support (1-18 keys)
  - Column configuration (spacing, width, line width)
  - Position settings (hit, light, score, combo)
  - Style settings (special style, combo burst style, note body style)
  - Upside-down mode support
  - Interface settings (split stages, stage separation)
  - Per-column colors and images
  - Stage images, visual effects, hit effects

## Usage

```rust
use rosu_skin::structs::*;
use std::path::PathBuf;

// Parse General section
let general = General::from_path(&PathBuf::from("path/to/skin.ini"));

// Parse Colours section
let colours = Colours::from_path(&PathBuf::from("path/to/skin.ini"));

// Parse Fonts section
let font = Font::from_path(&PathBuf::from("path/to/skin.ini"));

// Parse CatchTheBeat section
let ctb = CatchTheBeat::from_path(&PathBuf::from("path/to/skin.ini"));

// Parse Mania section (supports multiple keycounts)
let mania = Mania::from_path(&PathBuf::from("path/to/skin.ini"));
// Access specific keycount: mania.key_configs.get(&4) // 4K config
```

## Structure

The library is organized into modules:

- `structs/`: Data structures for each section
  - `common.rs`: Common types (Colour, ColourAlpha)
  - `general.rs`: General section
  - `colours.rs`: Colours section
  - `font.rs`: Fonts section
  - `gamemode/`: Game mode specific sections
    - `ctb.rs`: CatchTheBeat
    - `mania.rs`: Mania (with sub-structs for organization)

- `decode/`: Parsing implementations
  - Each section has its own decoder module
  - Modular parsing functions for maintainability

## Color Support

The library supports both RGB and RGB(a) color formats:
- RGB: `"255,192,0"` → `Colour { red: 255, green: 192, blue: 0 }`
- RGB(a): `"255,192,0,128"` → `ColourAlpha { color: ..., alpha: 128 }`
- RGB(a) with default alpha: `"255,192,0"` → `ColourAlpha { color: ..., alpha: 255 }`

## Mania Multi-Keycount Support

The Mania parser correctly handles multiple `[Mania]` sections with different `Keys` values:

```ini
[Mania]
Keys: 4
ColumnStart: 290
...

[Mania]
Keys: 5
ColumnStart: 270
...
```

Each keycount configuration is stored in `mania.key_configs` as a `HashMap<u8, ManiaKeyConfig>`.

## Error Handling

The parser uses graceful error handling:
- Missing sections return default values
- Invalid values are skipped (with defaults used)
- File read errors return empty/default structures

## Dependencies

- `rust-ini`: For basic INI file parsing (used for non-Mania sections)
- `serde`: For serialization support (optional, for future JSON export)

## License

[Add your license here]

## Contributing

Contributions are welcome! Please ensure all sections of the osu! skin.ini specification are properly supported.

## References

- [osu! skin.ini documentation](https://osu.ppy.sh/wiki/en/Skinning/skin.ini)
