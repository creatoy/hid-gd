## A hidapi extension for godot

* Written by rust with gdext

> How to use it: Download the release archive and decompress it to your godot project root directory.


Example:
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