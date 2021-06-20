// Stick
// Copyright © 2017-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your option (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).  This file may not be copied,
// modified, or distributed except according to those terms.

use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fmt::Write;

const LINUX: &str = "./sdb/linux/";
const _MACOS: &str = "./sdb/macos/";
const _WINDOWS: &str = "./sdb/windows/";
const _WEB: &str = "./sdb/web/";
const _USB: &str = "./sdb/usb/";

#[derive(Deserialize)]
struct Map {
    name: String,
    r#type: String,
    remap: HashMap<String, toml::value::Value>,
}

fn name_to_hex(name: &str) -> &str {
    match name {
        "None" => "00",
        "Exit" => "01",
        "ActionA" => "02",
        "ActionB" => "03",
        "ActionC" => "04",
        "ActionH" => "05",
        "ActionV" => "06",
        "ActionD" => "07",
        "MenuL" => "08",
        "MenuR" => "09",
        "Joy" => "0A",
        "Cam" => "0B",
        "BumperL" => "0C",
        "BumperR" => "0D",
        "TriggerL" => "0E",
        "TriggerR" => "0F",
        "Up" => "10",
        "Down" => "11",
        "Left" => "12",
        "Right" => "13",
        "HatUp" => "14",
        "HatDown" => "15",
        "HatLeft" => "16",
        "HatRight" => "17",
        "MicUp" => "18",
        "MicDown" => "19",
        "MicLeft" => "1A",
        "MicRight" => "1B",
        "PovUp" => "1C",
        "PovDown" => "1D",
        "PovLeft" => "1E",
        "PovRight" => "1F",
        "JoyX" => "20",
        "JoyY" => "21",
        "JoyZ" => "22",
        "CamX" => "23",
        "CamY" => "24",
        "CamZ" => "25",
        "Slew" => "26",
        "Throttle" => "27",
        "ThrottleL" => "28",
        "ThrottleR" => "29",
        "Volume" => "2A",
        "Wheel" => "2B",
        "Rudder" => "2C",
        "Gas" => "2D",
        "Brake" => "2E",
        "MicPush" => "2F",
        "Trigger" => "30",
        "Bumper" => "31",
        "ActionL" => "32",
        "ActionM" => "33",
        "ActionR" => "34",
        "Pinky" => "35",
        "PinkyForward" => "36",
        "PinkyBackward" => "37",
        "FlapsUp" => "38",
        "FlapsDown" => "39",
        "BoatForward" => "3A",
        "BoatBackward" => "3B",
        "AutopilotPath" => "3C",
        "AutopilotAlt" => "3D",
        "EngineMotorL" => "3E",
        "EngineMotorR" => "3F",        
        "EngineFuelFlowL" => "40",        
        "EngineFuelFlowR" => "41",
        "EngineIgnitionL" => "42",        
        "EngineIgnitionR" => "43",
        "SpeedbrakeBackward" => "44",
        "SpeedbrakeForward" => "45",
        "ChinaBackward" => "46",
        "ChinaForward" => "47",
        "Apu" => "48",
        "RadarAltimeter" => "49",
        "LandingGearSilence" => "4A",
        "Eac" => "4B",
        "AutopilotToggle" => "4C",
        "ThrottleButton" => "4D",
        "MouseX" => "4E",
        "MouseY" => "4F",
        "Mouse" => "50",
        "PaddleLeft" => "0x51",
        "PaddleRight" => "0x52",
        "PinkyLeft" => "0x53",
        "PinkyRight" => "0x54",
        "Context" => "0x55",
        "Dpi" => "0x56",
        "ScrollX" => "0x57",
        "ScrollY" => "0x58",
        "Scroll" => "0x59",
        "TrimUp" => "0x5A",
        "TrimDown" => "0x5B",
        "TrimLeft" => "0x5C",
        "TrimRight" => "0x5D",
        _unknown => panic!("Unknown: {}", _unknown),
    }
}

pub(super) fn main() {
    let mut out = String::new();

    println!("Loading Linux TOML Controller Mappings…");
    for file in std::fs::read_dir(LINUX).expect("Missing database").flatten() {
        let path = file.path();
        let file = std::fs::read_to_string(&path).expect("Open file failed");
        let file: Map = toml::from_str(&file).unwrap();

        // ID of Controller
        out.push_str(&path.as_path().file_name().unwrap().to_str().unwrap()[..16]);

        // Name of Controller.
        out.push_str(&file.name);
        out.push('\t');

        // Type of controller
        let ctlr_type = match file.r#type.as_str() {
            "xbox" => 'x',
            "playstation" => 'p',
            "nintendo" => 'n',
            "flight" => 'f',
            _type => panic!("Unknown type: {}", _type),
        };
        out.push(ctlr_type);

        // Add remappings
        let mut kv = Vec::new();
        for (key, value) in file.remap {
            kv.push((key, value));
        }
        kv.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
        for (key, value) in kv {
            if let Ok(number) = key.parse::<u8>() {
                write!(&mut out, "{:02X}", number | 0x80).unwrap();
            } else {
                out.push_str(name_to_hex(key.as_str()));
            }
            match value {
                toml::value::Value::String(event) => {
                    out.push_str(name_to_hex(event.as_str()));
                    out.push(';');
                }
                toml::value::Value::Table(table) => {
                    if let Some(event) = table.get("event") {
                        out.push_str(name_to_hex(event.as_str().unwrap()));
                    } else {
                        out.push_str(name_to_hex("None"));
                    }
                    if let Some(max) = table.get("max") {
                        let max = max.as_integer().unwrap();
                        out.push('a');
                        write!(&mut out, "{}", max).unwrap();
                    }
                    if let Some(min) = table.get("min") {
                        let min = min.as_integer().unwrap();
                        out.push('i');
                        write!(&mut out, "{}", min).unwrap();
                    }
                    if let Some(scale) = table.get("scale") {
                        let scale = scale.as_float().unwrap();
                        out.push('s');
                        write!(&mut out, "{}", scale).unwrap();
                    }
                    if let Some(deadzone) = table.get("deadzone") {
                        let deadzone = deadzone.as_float().unwrap();
                        out.push('d');
                        write!(&mut out, "{}", deadzone).unwrap();
                    }
                    out.push(';');
                }
                _map => panic!("invalid mapping: {:?}", _map),
            }
        }
        out.pop();

        // Newline to separate controllers.
        out.push('\n');
    }
    out.pop();

    std::fs::write("./stick/remap_linux.sdb", out).unwrap();
    // std::fs::write("./stick/sdlgc.sdb", out).unwrap();
}