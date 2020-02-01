extern crate x11rb;

use std::error::Error;
use std::mem::size_of;

use x11rb::connection::Connection;
use x11rb::generated::xproto;
use x11rb::xcb_ffi::XCBConnection;

// Maximum buffer size to allocate for retrieved x properties. Equivalent to 256 UTF8/ASCII chars.
const MAX_XPROP_BYTE_SIZE: u32 = 64;
// Size in bytes of x window ids
const WINDOW_ID_SIZE: usize = size_of::<xproto::WINDOW>();

pub fn get_focused_window_info() -> Result<super::FocusedWindowInfo, Box<dyn Error>> {
    // Initialize an XCB Connection and get focused window
    let (xcb_conn, _screen_id) = XCBConnection::connect(None)?;
    // Get x root window
    // TODO: Support checking multiple display roots? Can't see that being that necessary...
    let x11_setup = xcb_conn.setup();
    let x11_root = x11_setup.roots[0].root;
    // Parse the active window id -- bytes must be copied into a fixed-size array to parse into a u32 window id.
    let active_window_bytes = get_xprop_bytes(&xcb_conn, x11_root, b"_NET_ACTIVE_WINDOW")?;
    let mut active_window_bytes_arr: [u8; WINDOW_ID_SIZE] = [0; WINDOW_ID_SIZE];
    for i in 0..WINDOW_ID_SIZE {
        active_window_bytes_arr[i] = active_window_bytes[i];
    }
    let active_window_id = xproto::WINDOW::from_ne_bytes(active_window_bytes_arr);
    // Input focus probably makes more sense for this application, but apps don't request it consistently -- and some never do
    // let focused_window_id = xproto::get_input_focus(&xcb_conn)?.reply()?.focus;

    return Ok(super::FocusedWindowInfo {
        class: get_xprop_strings(&xcb_conn, active_window_id, b"WM_CLASS")?,
        name: get_xprop_strings(&xcb_conn, active_window_id, b"WM_NAME")?,
    });
}

// Gets an X Window Property as a vector of safe strings (split on null bytes)
fn get_xprop_strings(xcb_conn: &XCBConnection, window: xproto::WINDOW, prop_name: &[u8]) -> Result<Vec<String>, Box<dyn Error>> {
    let atom_result = xproto::intern_atom(xcb_conn, true, prop_name)?.reply()?;
    let prop_result = xproto::get_property(xcb_conn, false, window, atom_result.atom, 0, 0, MAX_XPROP_BYTE_SIZE)?.reply()?;
    let mut prop_vec = Vec::new();
    for split_val in prop_result.value.split(|&x| x == 0) {
        prop_vec.push(String::from_utf8(split_val.to_vec())?);
    }
    return Ok(prop_vec);
}

// Gets an xprop as a raw byte vector
fn get_xprop_bytes(xcb_conn: &XCBConnection, window: xproto::WINDOW, prop_name: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let atom_result = xproto::intern_atom(xcb_conn, true, prop_name)?.reply()?;
    let prop_result = xproto::get_property(xcb_conn, false, window, atom_result.atom, 0, 0, MAX_XPROP_BYTE_SIZE)?.reply()?;
    return Ok(prop_result.value);
}