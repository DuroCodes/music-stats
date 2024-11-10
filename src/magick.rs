use std::process::Command;

use crate::{
    lastfm::{
        types::{get_image, Album, Image},
        user::LastFmUser,
    },
    themes::{get_theme, Theme},
};

pub fn curl_image(image: &Image, output_path: &str) -> Result<String, String> {
    let output = Command::new("curl")
        .args(&["-o", output_path, &image.url])
        .output();

    match output {
        Ok(_) => Ok(output_path.into()),
        Err(_) => Err("Failed to download image".into()),
    }
}

pub fn text_command(text: &str, pointsize: &str, pos: &str, theme: &Theme) -> Vec<String> {
    let text = match text.len() {
        0..=30 => text.to_string(),
        _ => text.chars().take(27).collect::<String>() + "...",
    };

    vec![
        "-font",
        "Montserrat-Black",
        "-pointsize",
        pointsize,
        "-fill",
        &theme.secondary,
        "-annotate",
        pos,
        &text,
    ]
    .into_iter()
    .map(|s| s.into())
    .collect()
}

pub fn duotone_album_command(albums: &Vec<Album>, theme: &Theme) -> Vec<String> {
    let list_margin = 80;

    albums
        .iter()
        .enumerate()
        .flat_map(|(i, album)| {
            let duotone_image = format!("images/album_{i}_duotone.jpg");

            let number_y = 165 + i * list_margin;
            let number_pos = format!("+480+{number_y}");

            let album_name_y = 148 + i * list_margin;
            let album_name_pos = format!("+540+{album_name_y}");

            let artist_y = 175 + i * list_margin;
            let artist_pos = format!("+540+{artist_y}");

            let album_cover_x = 815 + (i % 2) * 190;
            let album_cover_y = 30 + (i / 2) * 190;
            let album_cover_pos = format!("+{album_cover_x}+{album_cover_y}");

            vec![
                "(",
                &duotone_image,
                "-alpha",
                "set",
                "-channel",
                "A",
                "-evaluate",
                "set",
                "30%",
                ")",
                "-geometry",
                &album_cover_pos,
                "-composite",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .into_iter()
            .chain(text_command(&(i + 1).to_string(), "55", &number_pos, theme))
            .chain(text_command(&album.name, "35", &album_name_pos, theme))
            .chain(text_command(&album.artist.name, "20", &artist_pos, theme))
            .collect::<Vec<String>>()
        })
        .collect::<Vec<String>>()
}

pub fn download_album_covers(images: &Vec<&Image>) -> Vec<Result<String, String>> {
    images
        .iter()
        .enumerate()
        .map(|(i, img)| {
            let output_path = format!("images/album_{i}.jpg");

            match curl_image(img, &output_path) {
                Ok(_) => Ok(output_path),
                Err(e) => Err(e),
            }
        })
        .collect()
}

pub fn download_user_image(image: &Image) -> Result<String, String> {
    let output_path = "images/user.jpg";
    curl_image(image, output_path)
}

pub fn generate_duotone_image(
    image_path: &str,
    size: &str,
    theme: &Theme,
) -> Result<String, String> {
    let image_stem = image_path.split('.').next().unwrap();
    let output_path = format!("{image_stem}_duotone.jpg");

    let output = Command::new("magick")
        .args(&[
            image_path,
            "-colorspace",
            "gray",
            "(",
            "-size",
            "256x1",
            &format!("gradient:{}-{}", &theme.secondary, &theme.primary),
            ")",
            "-clut",
            "-resize",
            size,
            &output_path,
        ])
        .output();

    match output {
        Ok(_) => Ok(output_path),
        Err(_) => Err("Failed to generate duotone image".into()),
    }
}

pub fn generate_gradient(gradient_path: &str, theme: &Theme) -> Result<String, String> {
    let output = Command::new("magick")
        .args(&[
            "-size",
            "400x401",
            &format!("gradient:{}-none", &theme.primary),
            "-alpha",
            "set",
            "-channel",
            "A",
            "-fx",
            "p < 0.7 ? (1 - p / 0.7) : 0",
            "-crop",
            "400x400+0+0",
            gradient_path,
        ])
        .output();

    match output {
        Ok(_) => Ok(gradient_path.into()),
        Err(_) => Err("Failed to generate gradient".into()),
    }
}

pub async fn generate_summary(
    lastfm: LastFmUser,
    theme_name: &str,
    output_path: &str,
) -> Result<String, String> {
    let top_albums = lastfm.get_top_albums().await.unwrap();
    let user_info = lastfm.get_info().await.unwrap();

    let albums = &top_albums.topalbums.album;

    let extralarge_image = get_image(&user_info.user.image, "extralarge").unwrap();
    let user_image = download_user_image(extralarge_image);

    if let Err(e) = user_image {
        return Err(e);
    }

    let theme = get_theme(theme_name, user_image.as_ref().unwrap());

    let duotone_user_image =
        generate_duotone_image(user_image.as_ref().unwrap(), "600x600", &theme);

    if let Err(e) = duotone_user_image {
        return Err(e);
    }

    let album_images = albums
        .iter()
        .map(|album| get_image(&album.image, "extralarge").unwrap())
        .collect();

    let album_downloads = download_album_covers(&album_images);

    if album_downloads.iter().any(|d| d.is_err()) {
        return Err("Failed to download album covers".into());
    }

    let duotone_albums = album_downloads
        .iter()
        .map(|d| generate_duotone_image(d.as_ref().unwrap(), "170x170", &theme))
        .collect::<Vec<Result<String, String>>>();

    if duotone_albums.iter().any(|d| d.is_err()) {
        return Err("Failed to generate duotone images".into());
    }

    let gradient = generate_gradient("images/gradient.png", &theme);

    if let Err(e) = gradient {
        return Err(e);
    }

    let output = Command::new("magick")
        .args(
            vec![
                "-size",
                "1200x600",
                &format!("xc:{}", &theme.primary),
                &duotone_user_image.unwrap(),
                "-geometry",
                "-240+0",
                "-composite",
                "(",
                &user_image.unwrap(),
                "-resize",
                "400x400",
                ")",
                "-geometry",
                "+60+100",
                "-composite",
                &gradient.unwrap(),
                "-geometry",
                "+60+100",
                "-composite",
            ]
            .into_iter()
            .map(|s| s.into())
            .chain(duotone_album_command(albums, &theme))
            .chain(text_command(
                &user_info.user.name.to_uppercase(),
                "35",
                "+70+485",
                &theme,
            ))
            .chain(text_command(
                "MOST LISTENED ALBUMS",
                "50",
                "+380+65",
                &theme,
            ))
            .chain(text_command(
                &lastfm.period.display().to_uppercase(),
                "22",
                "+380+90",
                &theme,
            ))
            .chain([output_path.into()])
            .collect::<Vec<String>>(),
        )
        .output();

    match output {
        Ok(_) => Ok("images/output.jpg".into()),
        Err(_) => Err("Failed to generate summary".into()),
    }
}
