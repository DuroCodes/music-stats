use std::process::Command;

use regex::Regex;

#[derive(Debug)]
pub struct Theme {
    pub primary: String,
    pub secondary: String,
}

pub fn get_theme(theme: &str, image_path: &str) -> Theme {
    match theme.to_lowercase().as_str() {
        "midnight" => Theme {
            primary: "#F7396F".into(),
            secondary: "#16006F".into(),
        },
        "forest" => Theme {
            primary: "#00D574".into(),
            secondary: "#1A2A56".into(),
        },
        "ocean" => Theme {
            primary: "#80f8f8".into(),
            secondary: "#7a004f".into(),
        },
        "strawberry" => Theme {
            primary: "#D7FD31".into(),
            secondary: "#EA1264".into(),
        },
        "bumblebee" => Theme {
            primary: "#ffea00".into(),
            secondary: "#141209".into(),
        },
        "crimson" => Theme {
            primary: "#dc2c2c".into(),
            secondary: "#000000".into(),
        },
        "aqua" => Theme {
            primary: "#68ebc1".into(),
            secondary: "#0239d8".into(),
        },
        "lavender" => Theme {
            primary: "#C9A7E7".into(),
            secondary: "#5F4B8B".into(),
        },
        "emerald" => Theme {
            primary: "#50C878".into(),
            secondary: "#0B3D3B".into(),
        },
        "cherry" => Theme {
            primary: "#FF4C94".into(),
            secondary: "#C70039".into(),
        },
        "twilight" => Theme {
            primary: "#7A5C8D".into(),
            secondary: "#1C1B29".into(),
        },
        "flame" => Theme {
            primary: "#FF4500".into(),
            secondary: "#6A240E".into(),
        },
        "moss" => Theme {
            primary: "#8A9A5B".into(),
            secondary: "#4F4D40".into(),
        },
        "catppuccin" => Theme {
            primary: "#CBA6F7".into(),
            secondary: "#1E1E2E".into(),
        },
        "horizon" => Theme {
            primary: "#D55170".into(),
            secondary: "#1C1E26".into(),
        },
        _ => extract_theme(image_path).unwrap_or(Theme {
            primary: "#000000".into(),
            secondary: "#FFFFFF".into(),
        }),
    }
}

pub fn extract_theme(image_path: &str) -> Result<Theme, String> {
    let theme_path = "images/theme.jpg";

    let theme_output = Command::new("magick")
        .args(&[
            image_path,
            "-geometry",
            "16x16",
            "-colorspace",
            "HSB",
            "-colors",
            "5",
            "-unique-colors",
            "-scale",
            "4000%",
            theme_path,
        ])
        .output();

    if theme_output.is_err() {
        return Err("Failed to extract theme".into());
    }

    let histogram_output = Command::new("magick")
        .args(&[theme_path, "-format", "%c", "histogram:info:-"])
        .output();

    if histogram_output.is_err() {
        return Err("Failed to extract theme".into());
    }

    let theme_hex = String::from_utf8_lossy(&histogram_output.unwrap().stdout)
        .lines()
        .map(|line| {
            let re = Regex::new(r"#([A-Fa-f0-9]{6})").unwrap();
            let hex = re.captures(line).unwrap().get(0).unwrap().as_str();
            hex.to_string()
        })
        .collect::<Vec<String>>();

    Ok(Theme {
        primary: theme_hex[3].clone(),
        secondary: theme_hex[0].clone(),
    })
}
