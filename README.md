## A hidapi extension for godot

* Written by rust with gdext

### How to use it:
Download the release archive and decompress it to your godot project root directory.


### Example:
```
func _ready():
    # List all connected HID devices.
    var devices = Hid.list_devices()
    print(devices)

    var hid = Hid.new()
    # Open by vender id and product id
    hid.open(vid, pid)
    # Or open by device path
    #hid.open_path(path)
    # Or open by serial number
    #hid.open_serial(vid, pid, serial_number)

    # Then you can read HID report data from device.
    var report_recv = hid.read(64)
    #var report_recv = hid.read_timeout(64, 10)
    # And write report data to HID
    hid.write(report_send)
```

### Or you need to compile the extension by yourself
1. Install [Rust](https://www.rust-lang.org/tools/install) on your computer.
2. Clone the repo and compile it.
```sh
git clone https://github.com/creatoy/hid-gd.git
cd hid-gd
cargo build
# or
# cargo build --release
```
> The "libhid_ext.so/dll/dylib" file will be created in *./target/*
3. Create the "hid.extension" file (On Linux x86_64 for example)
```
[configuration]
entry_symbol = "hid_ext_init"
compatibility_minimum = 4.1

[libraries]
linux.x86_64 = "libhid_ext.so"
```
4. Copy the two files to your godot project's *addon* dir.
