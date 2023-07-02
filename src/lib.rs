use godot::prelude::*;

mod hid;

struct HidLib;

#[gdextension(entry_point=hid_lib_init)]
unsafe impl ExtensionLibrary for HidLib {}
