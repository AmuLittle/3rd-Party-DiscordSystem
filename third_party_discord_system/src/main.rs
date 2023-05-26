/*
3rd (Party) Discord System: A discord client for the 3ds,
Copyright (C) 2023  SmolFoxyBoi

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
mod api;
mod ui;

use ctru::prelude::*;

fn main() {
    ctru::use_panic_handler();

    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");

    /* cfg_if::cfg_if! {
        if #[cfg(all(feature = "romfs", romfs_exists))] { // Unused, broken and I have no idea why */
    
    let _romfs = ctru::services::romfs::RomFS::new().expect("unable to start romfs"); // just start it
    while apt.main_loop() {
        hid.scan_input();
    
        if hid.keys_down().contains(KeyPad::START) {
            break;
        }
        
        gfx.wait_for_vblank();
    }
}
