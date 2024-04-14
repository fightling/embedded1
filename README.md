# Install espflash

```sh
▶ cargo install espflash
    Updating crates.io index
  Downloaded espflash v3.0.0

[..]

   Compiling espflash v3.0.0
    Finished `release` profile [optimized] target(s) in 46.66s
   Replacing /home/pat/.cargo/bin/espflash
    Replaced package `espflash v1.7.0` with `espflash v3.0.0` (executable `espflash`)
```

# Clone this repository

```sh
▶ git clone https://github.com/schub/embedded1.git
Cloning into 'embedded1'...
remote: Enumerating objects: 29, done.
remote: Counting objects: 100% (29/29), done.
remote: Compressing objects: 100% (18/18), done.
remote: Total 29 (delta 9), reused 25 (delta 5), pack-reused 0
Receiving objects: 100% (29/29), 13.67 KiB | 933.00 KiB/s, done.
Resolving deltas: 100% (9/9), done.
▶ cd embedded1
```

# Build embedded1

```sh
▶ cargo build
info: syncing channel updates for 'nightly-x86_64-unknown-linux-gnu'
info: latest update on 2024-04-14, rust version 1.79.0-nightly (0bf471f33 2024-04-13)
info: component 'rust-src' is up to date
info: downloading component 'rust-std' for 'riscv32imc-unknown-none-elf'
info: installing component 'rust-std' for 'riscv32imc-unknown-none-elf'
    Updating crates.io index
  Downloaded bitflags v2.5.0
[..]
   Compiling embedded1 v0.1.0 (/home/pat/git/embedded1)
    Finished `dev` profile [optimized + debuginfo] target(s) in 20.53s
```
# Flash & run embedded1

```sh
▶ cargo run
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.06s
     Running `espflash flash --monitor target/riscv32imc-unknown-none-elf/debug/embedded1`
✔ Use serial port '/dev/ttyACM0' - USB JTAG/serial debug unit? · yes
✔ Remember this serial port for future use? · no
[2024-04-14T08:52:28Z INFO ] Serial port: '/dev/ttyACM0'
[2024-04-14T08:52:28Z INFO ] Connecting...
[2024-04-14T08:52:28Z INFO ] Using flash stub
Chip type:         esp32c3 (revision v0.3)
Crystal frequency: 40 MHz
Flash size:        4MB
Features:          WiFi, BLE
MAC address:       60:55:f9:23:47:30
App/part. size:    81,200/4,128,768 bytes, 1.97%
[00:00:00] [========================================]      13/13      0x0                                                                                                                                                                                                        [00:00:00] [========================================]       1/1       0x8000                                                                                                                                                                                                     [00:00:00] [========================================]      16/16      0x10000                                                                                                                                                                                                    [2024-04-14T08:52:30Z INFO ] Flashing has completed!
Commands:
    CTRL+R    Reset chip
    CTRL+C    Exit

ESP-ROM:esp32c3-api1-20210207
Build:Feb  7 2021
rst:0x15 (USB_UART_CHIP_RESET),boot:0xf (SPI_FAST_FLASH_BOOT)
Saved PC:0x4038063e
0x4038063e - __EXTERNAL_INTERRUPTS
    at ??:??
SPIWP:0xee
mode:DIO, clock div:2
load:0x3fcd5820,len:0x1714
load:0x403cc710,len:0x968
load:0x403ce710,len:0x2f9c
entry 0x403cc710
I (24) boot: ESP-IDF v5.1.2-342-gbcf1645e44 2nd stage bootloader
I (24) boot: compile time Dec 12 2023 10:50:58
I (25) boot: chip revision: v0.3
I (29) boot.esp32c3: SPI Speed      : 40MHz
I (34) boot.esp32c3: SPI Mode       : DIO
I (38) boot.esp32c3: SPI Flash Size : 4MB
I (43) boot: Enabling RNG early entropy source...
I (48) boot: Partition Table:
I (52) boot: ## Label            Usage          Type ST Offset   Length
I (59) boot:  0 nvs              WiFi data        01 02 00009000 00006000
I (67) boot:  1 phy_init         RF data          01 01 0000f000 00001000
I (74) boot:  2 factory          factory app      00 00 00010000 003f0000
I (82) boot: End of partition table
I (86) esp_image: segment 0: paddr=00010020 vaddr=3c010020 size=022a0h (  8864) map
I (96) esp_image: segment 1: paddr=000122c8 vaddr=40380000 size=00f68h (  3944) load
I (104) esp_image: segment 2: paddr=00013238 vaddr=00000000 size=0cde0h ( 52704) 
I (122) esp_image: segment 3: paddr=00020020 vaddr=42000020 size=03ce8h ( 15592) map
I (127) boot: Loaded app from partition at offset 0x10000
```
