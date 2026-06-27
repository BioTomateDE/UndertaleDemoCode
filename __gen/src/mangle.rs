// SPDX-License-Identifier: GPL-2.0-only
mod key;
mod mouse;
mod other;

use gm8exe::{AssetList, asset::Object};

use crate::{
    depascal, fail,
    mangle::{key::mangle_key, mouse::mangle_mouse, other::mangle_other},
    validate_name,
};

pub fn mangle_event(event_index: usize, subtype: u32, objects: &AssetList<Object>) -> String {
    match event_index {
        0 => "create".to_owned(),
        1 => "destroy".to_owned(),
        2 => format!("alarm{subtype}"),
        3 => match subtype {
            0 => "step",
            1 => "step_begin",
            2 => "step_end",
            _ => fail!("Invalid step subtype {subtype}"),
        }
        .to_owned(),
        4 => {
            let Some(Some(object)) = objects.get(subtype as usize) else {
                fail!("Collision event refers to non-existent game object {subtype}");
            };
            let target_name = depascal(&object.name);
            validate_name(target_name);
            format!("collision_{target_name}")
        }
        5 => format!("keyboard_{}", mangle_key(subtype).to_ascii_lowercase()),
        6 => format!("mouse_{}", mangle_mouse(subtype).to_ascii_lowercase()),
        7 => format!("other_{}", mangle_other(subtype).to_ascii_lowercase()),
        8 => match subtype {
            0 => "draw",
            64 => "draw_gui",
            65 => "resize",
            72 => "draw_begin",
            73 => "draw_end",
            74 => "draw_gui_begin",
            75 => "draw_gui_end",
            76 => "draw_pre",
            77 => "draw_post",
            _ => fail!("Invalid draw subtype {subtype}"),
        }
        .to_owned(),
        9 => format!("keypress_{}", mangle_key(subtype).to_ascii_lowercase()),
        10 => format!("keyrelease_{}", mangle_key(subtype).to_ascii_lowercase()),
        11 => "trigger".to_owned(),
        _ => fail!("Invalid event index {event_index}"),
    }
}
