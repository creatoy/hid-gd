use godot::prelude::*;
use hidapi::*;
use std::ffi::CString;

#[derive(GodotClass)]
#[class(base=Object)]
struct Hid {
    dev: Option<HidDevice>,

    #[base]
    base: Base<Object>,
}

#[godot_api]
impl ObjectVirtual for Hid {
    fn init(base: Base<Object>) -> Self {
        Self {
            dev: None,
            base
        }
    }
}

#[godot_api]
impl Hid {
    #[func]
    fn list_devices() -> Array<Dictionary> {
        if let Ok(api) = HidApi::new() {
            api.device_list().map(
                |info| {
                    let mut dict = Dictionary::new();
                    dict.insert("path", info.path().clone().to_string_lossy().to_string());
                    dict.insert("vid", info.vendor_id());
                    dict.insert("pid", info.product_id());
                    dict.insert("serial_number", info.serial_number().unwrap_or_default());
                    dict.insert("release_number", info.release_number());
                    dict.insert("manufacturer_string", info.manufacturer_string().unwrap_or_default());
                    dict.insert("product_string", info.product_string().unwrap_or_default());
                    dict.insert("usage_page", info.usage_page());
                    dict.insert("usage", info.usage());
                    dict.insert("interface_number", info.interface_number());
                    // dict.insert("bus_type", info.bus_type());
                    dict
                }
            ).collect()
        } else {
            godot_error!("Failed to create HidApi");
            Array::new()
        }
    }

    #[func]
    fn get_error_message() -> GodotString {
        if let Err(err) = HidApi::new() {
            err.to_string().into()
        } else {
            "No error".into()
        }
    }

    #[func]
    fn open(&mut self, vid: u16, pid: u16) -> bool {
        if let Ok(api) = HidApi::new() {
            if let Ok(dev) = api.open(vid, pid) {
                self.dev = Some(dev);
                true
            } else {
                godot_error!("Failed to open device");
                false
            }
        } else {
            godot_error!("Failed to create HidApi");
            false
        }
    }

    #[func]
    fn open_serial(&mut self, vid: u16, pid: u16, serial_number: GodotString) -> bool {
        if let Ok(api) = HidApi::new() {
            if let Ok(dev) = api.open_serial(vid, pid, serial_number.to_string().as_str()) {
                self.dev = Some(dev);
                true
            } else {
                godot_error!("Failed to open device");
                false
            }
        } else {
            godot_error!("Failed to create HidApi");
            false
        }
    }

    #[func]
    fn open_path(&mut self, path: GodotString) -> bool {
        if let Ok(api) = HidApi::new() {
            if let Ok(dev) = api.open_path(CString::new(path.to_string().as_str()).unwrap().as_c_str()) {
                self.dev = Some(dev);
                true
            } else {
                godot_error!("Failed to open device");
                false
            }
        } else {
            godot_error!("Failed to create HidApi");
            false
        }
    }

    #[func]
    fn write(&mut self, data: PackedByteArray) -> i32 {
        match self.dev {
            Some(ref mut dev) => {
                if let Ok(bytes) = dev.write(data.as_slice()) {
                    bytes as i32
                } else {
                    godot_error!("Failed to write to device");
                    -1
                }
            },
            None => {
                godot_error!("Device not open");
                -1
            },
        }
    }

    #[func]
    fn read(&mut self, size: u32) -> PackedByteArray {
        match self.dev {
            Some(ref mut dev) => {
                let mut buf = vec![0u8; size as usize];
                if let Ok(bytes) = dev.read(buf.as_mut_slice()) {
                    buf.as_slice()[..bytes].into()
                } else {
                    godot_error!("Failed to read from device");
                    PackedByteArray::new()
                }
            },
            None => {
                godot_error!("Device not open");
                PackedByteArray::new()
            },
        }
    }

    #[func]
    fn read_timeout(&mut self, size: u32, timeout: i32) -> PackedByteArray {
        match self.dev {
            Some(ref mut dev) => {
                let mut buf = vec![0u8; size as usize];
                if let Ok(bytes) = dev.read_timeout(buf.as_mut_slice(), timeout) {
                    buf.as_slice()[..bytes].into()
                } else {
                    godot_error!("Failed to read from device");
                    PackedByteArray::new()
                }
            },
            None => {
                godot_error!("Device not open");
                PackedByteArray::new()
            },
        }
    }

    #[func]
    fn send_feature_report(&mut self, report: PackedByteArray) -> bool {
        match self.dev {
            Some(ref mut dev) => {
                if let Ok(_) = dev.send_feature_report(report.as_slice()) {
                    true
                } else {
                    godot_error!("Failed to send feature report");
                    false
                }
            },
            None => {
                godot_error!("Device not open");
                false
            },
        }
    }

    #[func]
    fn get_feature_report(&mut self, report_id: u8) -> PackedByteArray {
        match self.dev {
            Some(ref mut dev) => {
                let mut buf = vec![report_id, 1];
                if let Ok(bytes) = dev.get_feature_report(buf.as_mut_slice()) {
                    buf.as_slice()[..bytes].into()
                } else {
                    godot_error!("Failed to get feature report");
                    PackedByteArray::new()
                }
            },
            None => {
                godot_error!("Device not open");
                PackedByteArray::new()
            },
        }
    }

    #[func]
    fn set_blocking_mode(&mut self, blocking: bool) -> bool {
        match self.dev {
            Some(ref mut dev) => {
                if let Ok(_) = dev.set_blocking_mode(blocking) {
                    true
                } else {
                    godot_error!("Failed to set blocking mode");
                    false
                }
                
            },
            None => {
                godot_error!("Device not open");
                false
            },
        }
    }

    #[func]
    fn get_device_info(&self) -> Dictionary {
        if let Some(ref dev) = self.dev {
            if let Ok(dev) = dev.get_device_info() {
                let mut dict = Dictionary::new();
                dict.insert("path", dev.path().clone().to_string_lossy().to_string());
                dict.insert("vid", dev.vendor_id());
                dict.insert("pid", dev.product_id());
                dict.insert("serial_number", dev.serial_number().unwrap_or_default());
                dict.insert("release_number", dev.release_number());
                dict.insert("manufacturer_string", dev.manufacturer_string().unwrap_or_default());
                dict.insert("product_string", dev.product_string().unwrap_or_default());
                dict.insert("usage_page", dev.usage_page());
                dict.insert("usage", dev.usage());
                dict.insert("interface_number", dev.interface_number());
                // dict.insert("bus_type", dev.bus_type());
                return dict;
            } else {
                godot_error!("Failed to get device info");
            }
        } else {
            godot_error!("Device not open");
        }
        return Dictionary::new();
    }

    #[func]
    fn get_manufacturer_string(&self) -> GodotString {
        if let Some(ref dev) = self.dev {
            if let Ok(manufacturer_string) = dev.get_manufacturer_string() {
                if let Some(manufacturer_string) = manufacturer_string {
                    return manufacturer_string.into();
                } else {
                    return "".into();
                }
            } else {
                godot_error!("Failed to get manufacturer string");
            }
        } else {
            godot_error!("Device not open");
        }
        return "".into();
    }

    #[func]
    fn get_product_string(&self) -> GodotString {
        if let Some(ref dev) = self.dev {
            if let Ok(product_string) = dev.get_product_string() {
                if let Some(product_string) = product_string {
                    return product_string.into();
                } else {
                    return "".into();
                }
            } else {
                godot_error!("Failed to get product string");
            }
        } else {
            godot_error!("Device not open");
        }
        return "".into();
    }

    #[func]
    fn get_serial_number_string(&self) -> GodotString {
        if let Some(ref dev) = self.dev {
            if let Ok(serial_number_string) = dev.get_serial_number_string() {
                if let Some(serial_number_string) = serial_number_string {
                    return serial_number_string.into();
                } else {
                    return "".into();
                }
            } else {
                godot_error!("Failed to get serial number string");
            }
        } else {
            godot_error!("Device not open");
        }
        return "".into();
    }

    #[func]
    fn get_indexed_string(&self, index: i32) -> GodotString {
        if let Some(ref dev) = self.dev {
            if let Ok(indexed_string) = dev.get_indexed_string(index) {
                if let Some(indexed_string) = indexed_string {
                    return indexed_string.into();
                } else {
                    return "".into();
                }
            } else {
                godot_error!("Failed to get indexed string");
            }
        } else {
            godot_error!("Device not open");
        }
        return "".into();
    }
}
