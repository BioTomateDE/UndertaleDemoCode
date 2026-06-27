// SPDX-License-Identifier: GPL-2.0-only
mod error;
mod mangle;

use crate::{error::tryy, mangle::mangle_event};
use gm8exe::{
    AssetList, GameAssets,
    asset::{CodeAction, Object, PascalString, Script},
    reader::from_exe,
};
use std::{
    io::ErrorKind,
    path::{Path, PathBuf, absolute},
};

fn logger(message: &str) {
    eprintln!("[LOG] {message}");
}

fn usage() -> ! {
    eprintln!("Usage: gm8decomp <EXE_FILE_PATH> <EXPORT_DIR>");
    std::process::exit(1);
}

fn main() {
    let mut args = std::env::args_os().skip(1);
    let exe_path = match args.next() {
        Some(p) => tryy("Invalid EXE_FILE_PATH", absolute(PathBuf::from(p))),
        None => usage(),
    };
    let export_dir = match args.next() {
        Some(p) => tryy("Invalid EXPORT_DIR", absolute(PathBuf::from(p))),
        None => usage(),
    };
    if args.next().is_some() {
        usage();
    }

    if !exe_path.exists() {
        fail!("Error: game exe not found: {}", exe_path.display());
    }

    let exe_data: Vec<u8> = tryy("Could not read exe file", std::fs::read(exe_path));
    let strict = true;
    let multithread = false;
    let data: GameAssets = tryy(
        "Could not parse GameMaker asset data",
        from_exe(exe_data, Some(logger), strict, multithread),
    );

    mkdir(&export_dir);

    export_scripts(&data.scripts, &export_dir);
    export_objects(&data.objects, &export_dir);

    println!("Exported successfully to {}", export_dir.display());
}

fn export_scripts(scripts: &AssetList<Script>, export_dir: &Path) {
    let dir = export_dir.join("scripts");
    mkdir(&dir);

    for script in scripts.iter().flatten() {
        let name = depascal(&script.name);
        let source = depascal(&script.source);
        validate_name(name);
        let path = dir.join(format!("{name}.gml"));
        tryy(
            "Could not write script GML file",
            std::fs::write(&path, source),
        );
    }
}

fn export_objects(objects: &AssetList<Object>, export_dir: &Path) {
    let base_dir = export_dir.join("objects");
    mkdir(&base_dir);

    for object in objects.iter().flatten() {
        let name = depascal(&object.name);
        validate_name(name);
        let obj_dir = base_dir.join(name);

        for (event_index, events) in object.events.iter().enumerate() {
            for (subtype, actions) in events {
                if actions.is_empty() {
                    continue;
                }

                let event_name = mangle_event(event_index, *subtype, objects);

                mkdir(&obj_dir);

                for (i, action) in actions.iter().enumerate() {
                    let code = action_to_gml(action);
                    let filename = format!("{event_name}_{i}.gml");
                    tryy(
                        "Could not write game object event GML file",
                        std::fs::write(obj_dir.join(&filename), &code),
                    );
                }
            }
        }
    }
}

fn action_to_gml(action: &CodeAction) -> String {
    let fnname = depascal(&action.fn_name);
    if fnname.is_empty() {
        depascal(&action.param_strings[0]).to_owned()
    } else {
        let args = action.param_strings[..action.param_count]
            .iter()
            .map(depascal)
            .collect::<Vec<_>>()
            .join(", ");
        let yap = "This action is a function call and was recreated as GML code.";
        format!("// {yap}\n{fnname}({args});")
    }
}

fn validate_name(name: &str) {
    if name.is_empty()
        || name == "."
        || name.contains("..")
        || name.contains('/')
        || name.contains('\\')
        || name.contains('\0')
    {
        fail!("Name {name:?} will lead to problems in paths");
    }
}

fn depascal(pascal_string: &PascalString) -> &str {
    tryy(
        "String contains invalid UTF-8",
        std::str::from_utf8(&pascal_string.0),
    )
}

fn mkdir(dir_path: &Path) {
    match std::fs::create_dir(dir_path) {
        Ok(()) => {}
        Err(e) if e.kind() == ErrorKind::AlreadyExists => {}
        Err(e) => fail!("Could not create directory {dir_path:?}: {e}"),
    }
}
