<img src="resources/icons/logos/mpui_icon.svg" width="125">

# MPUI
_At some point, you need to bake your own bread._

```rust
use mpui::ui::{MWindow, MText}
use mpui::types::{MColor}

fn main() {
    let window = MWindow {
        title: "MPUI Window",
        background: MColor::from_hex("#ffffff")
    }
    let text = MText {
        label: "Hello, World!"
    }
    window.set_primary_widget(text);
    window.show();
}
```

We created MPUI because Rust needs an end-all, cross-platform, GPU-accelerated UI framework.

## Overview

> **All one language**

MPUI user interfaces can be created all in native rust. No macros, no extra ui-related files, it's all purely one language.

> **Huge library**

MPUI not only includes a widget for that one specific thing (`mpui::ui`), it includes types (`mpui::types`) for various tools too! Colors, fonts, threading, and more can be used.
