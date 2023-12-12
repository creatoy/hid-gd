use godot::prelude::*;
use hidapi::HidApi;
use hidapi::HidDevice;
use std::ffi::CString;

#[derive(GodotClass)]
#[class(base=RefCounted)]
struct Hid {
    dev: Option<HidDevice>,

    #[base]
    base: Base<RefCounted>,
}

#[godot_api]
impl IRefCounted for Hid {
    fn init(base: Base<RefCounted>) -> Self {
        Self { dev: None, base }
    }
}

#[godot_api]
impl Hid {
    #[func]
    fn list_devices() -> Array<Dictionary> {
        match HidApi::new() {
            Ok(api) => {
                api.device_list()
                    .map(|info| {
                        let mut dict = Dictionary::new();
                        dict.insert("path", info.path().to_string_lossy().to_string());
                        dict.insert("vid", info.vendor_id());
                        dict.insert("pid", info.product_id());
                        dict.insert("serial_number", info.serial_number().unwrap_or_default());
                        dict.insert("release_number", info.release_number());
                        dict.insert(
                            "manufacturer_string",
                            info.manufacturer_string().unwrap_or_default(),
                        );
                        dict.insert("product_string", info.product_string().unwrap_or_default());
                        dict.insert("usage_page", info.usage_page());
                        dict.insert("usage", info.usage());
                        dict.insert("interface_number", info.interface_number());
                        // dict.insert("bus_type", info.bus_type());
                        dict
                    })
                    .collect()
            }
            Err(err) => {
                godot_error!("Failed to create HidApi: {}", err);
                Array::new()
            }
        }
    }

    #[func]
    fn open(&mut self, vid: u16, pid: u16) -> bool {
        match HidApi::new() {
            Ok(api) => match api.open(vid, pid) {
                Ok(dev) => {
                    self.dev = Some(dev);
                    true
                }
                Err(err) => {
                    godot_error!("Failed to open device: {}", err);
                    false
                }
            },
            Err(err) => {
                godot_error!("Failed to create HidApi: {}", err);
                false
            }
        }
    }

    #[func]
    fn open_serial(&mut self, vid: u16, pid: u16, serial_number: GodotString) -> bool {
        match HidApi::new() {
            Ok(api) => match api.open_serial(vid, pid, serial_number.to_string().as_str()) {
                Ok(dev) => {
                    self.dev = Some(dev);
                    true
                }
                Err(err) => {
                    godot_error!("Failed to open device: {}", err);
                    false
                }
            },
            Err(err) => {
                godot_error!("Failed to create HidApi: {}", err);
                false
            }
        }
    }

    #[func]
    fn open_path(&mut self, path: GodotString) -> bool {
        match HidApi::new() {
            Ok(api) => {
                match api.open_path(CString::new(path.to_string().as_str()).unwrap().as_c_str()) {
                    Ok(dev) => {
                        self.dev = Some(dev);
                        true
                    }
                    Err(err) => {
                        godot_error!("Failed to open device: {}", err);
                        false
                    }
                }
            }
            Err(err) => {
                godot_error!("Failed to create HidApi: {}", err);
                false
            }
        }
    }

    /// The first byte of `data` must contain the Report ID. For
    /// devices which only support a single report, this must be set
    /// to 0x0. The remaining bytes contain the report data. Since
    /// the Report ID is mandatory, calls to `write()` will always
    /// contain one more byte than the report contains. For example,
    /// if a hid report is 16 bytes long, 17 bytes must be passed to
    /// `write()`, the Report ID (or 0x0, for devices with a
    /// single report), followed by the report data (16 bytes). In
    /// this example, the length passed in would be 17.
    /// `write()` will send the data on the first OUT endpoint, if
    /// one exists. If it does not, it will send the data through
    /// the Control Endpoint (Endpoint 0).
    #[func]
    fn write(&mut self, data: PackedByteArray) -> i32 {
        match self.dev {
            Some(ref mut dev) => match dev.write(data.as_slice()) {
                Ok(bytes) => bytes as i32,
                Err(err) => {
                    godot_error!("Failed to write to device: {}", err);
                    -1
                }
            },
            None => {
                godot_error!("Device not open");
                -1
            }
        }
    }

    /// Input reports are returned to the host through the 'INTERRUPT IN'
    /// endpoint. The first byte will contain the Report number if the device
    /// uses numbered reports.
    #[func]
    fn read(&mut self, size: u32) -> PackedByteArray {
        match self.dev {
            Some(ref mut dev) => {
                let mut buf = vec![0u8; size as usize];
                match dev.read(buf.as_mut_slice()) {
                    Ok(bytes) => buf.as_slice()[..bytes].into(),
                    Err(err) => {
                        godot_error!("Failed to read from device: {}", err);
                        PackedByteArray::new()
                    }
                }
            }
            None => {
                godot_error!("Device not open");
                PackedByteArray::new()
            }
        }
    }

    #[func]
    fn read_timeout(&mut self, size: u32, timeout: i32) -> PackedByteArray {
        match self.dev {
            Some(ref mut dev) => {
                let mut buf = vec![0u8; size as usize];
                match dev.read_timeout(buf.as_mut_slice(), timeout) {
                    Ok(bytes) => buf.as_slice()[..bytes].into(),
                    Err(err) => {
                        godot_error!("Failed to read from device: {}", err);
                        PackedByteArray::new()
                    }
                }
            }
            None => {
                godot_error!("Device not open");
                PackedByteArray::new()
            }
        }
    }

    /// Send a Feature report to the device.
    /// Feature reports are sent over the Control endpoint as a
    /// Set_Report transfer.  The first byte of `data` must contain the
    /// 'Report ID'. For devices which only support a single report, this must
    /// be set to 0x0. The remaining bytes contain the report data. Since the
    /// 'Report ID' is mandatory, calls to `send_feature_report()` will always
    /// contain one more byte than the report contains. For example, if a hid
    /// report is 16 bytes long, 17 bytes must be passed to
    /// `send_feature_report()`: 'the Report ID' (or 0x0, for devices which
    /// do not use numbered reports), followed by the report data (16 bytes).
    /// In this example, the length passed in would be 17.
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
            }
            None => {
                godot_error!("Device not open");
                false
            }
        }
    }

    /// Set the first byte of `buf` to the 'Report ID' of the report to be read.
    /// Upon return, the first byte will still contain the Report ID, and the
    /// report data will start in `buf[1]`.
    #[func]
    fn get_feature_report(&mut self, report_id: u8) -> PackedByteArray {
        match self.dev {
            Some(ref mut dev) => {
                let mut buf = vec![report_id, 1];
                match dev.get_feature_report(buf.as_mut_slice()) {
                    Ok(bytes) => buf.as_slice()[1..bytes].into(),
                    Err(err) => {
                        godot_error!("Failed to get feature report: {}", err);
                        PackedByteArray::new()
                    }
                }
            }
            None => {
                godot_error!("Device not open");
                PackedByteArray::new()
            }
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
            }
            None => {
                godot_error!("Device not open");
                false
            }
        }
    }

    /// Get [`DeviceInfo`] from a HID device.
    #[func]
    fn get_device_info(&self) -> Dictionary {
        if let Some(ref dev) = self.dev {
            match dev.get_device_info() {
                Ok(dev) => {
                    let mut dict = Dictionary::new();
                    dict.insert("path", dev.path().to_string_lossy().to_string());
                    dict.insert("vid", dev.vendor_id());
                    dict.insert("pid", dev.product_id());
                    dict.insert("serial_number", dev.serial_number().unwrap_or_default());
                    dict.insert("release_number", dev.release_number());
                    dict.insert(
                        "manufacturer_string",
                        dev.manufacturer_string().unwrap_or_default(),
                    );
                    dict.insert("product_string", dev.product_string().unwrap_or_default());
                    dict.insert("usage_page", dev.usage_page());
                    dict.insert("usage", dev.usage());
                    dict.insert("interface_number", dev.interface_number());
                    // dict.insert("bus_type", dev.bus_type());
                    return dict;
                }
                Err(err) => {
                    godot_error!("Failed to get device info: {}", err);
                }
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
