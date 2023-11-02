use godot::prelude::*;

mod hid;

struct HidLib;

#[gdextension(entry_point=hid_ext_init)]
unsafe impl ExtensionLibrary for HidLib {}
