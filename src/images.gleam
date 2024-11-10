import gleam/dict
import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/string
import lastfm/types
import shellout

fn download_image(image: types.Image, img_path: String) -> String {
  let _ =
    shellout.command(
      run: "curl",
      with: ["-o", img_path, image.text],
      in: ".",
      opt: [],
    )
  img_path
}

fn download_album_covers(album_images: List(types.Image)) -> List(String) {
  album_images
  |> list.index_map(fn(img, idx) {
    let img_path = "images/album_" <> int.to_string(idx) <> ".jpg"
    let _ = download_image(img, img_path)
    img_path
  })
}

fn download_user_image(user_image: types.Image) -> String {
  let img_path = "images/user.jpg"
  let _ = download_image(user_image, img_path)
  img_path
}

pub fn extract_palette(img_path: String) -> #(String, String) {
  let palette_path = "images/palette.png"

  let _ =
    shellout.command(
      run: "magick",
      with: [
        img_path,
        "-geometry",
        "16x16",
        "-colorspace",
        "HSB",
        "-colors",
        "5",
        "-unique-colors",
        "-scale",
        "4000%",
        palette_path,
      ],
      in: ".",
      opt: [],
    )

  let assert Ok(color_palette) =
    shellout.command(
      run: "magick",
      with: [palette_path, "-format", "%c", "histogram:info:-"],
      in: ".",
      opt: [],
    )

  let colors =
    color_palette
    |> string.split("\n")
    |> list.map(fn(line) {
      line
      |> string.split(" ")
      |> list.filter(string.starts_with(_, "#"))
      |> string.join(" ")
    })
    |> list.sized_chunk(3)
    |> list.map(fn(c) {
      let assert Ok(first) = list.first(c)
      first
    })
    |> list.take(2)
    |> list.reverse

  case colors {
    [color1, color2] -> #(color1, color2)
    _ -> #("#FFFFFF", "#000000")
  }
}

fn generate_duotone_image(
  img_path: String,
  duotone_path: String,
  size: String,
  colors: #(String, String),
) -> String {
  let _ =
    shellout.command(
      run: "magick",
      with: [
        img_path,
        "-colorspace",
        "gray",
        "(",
        "-size",
        "256x1",
        "gradient:" <> colors.1 <> "-" <> colors.0,
        ")",
        "-clut",
        "-resize",
        size <> "x" <> size,
        duotone_path,
      ],
      in: ".",
      opt: [],
    )

  duotone_path
}

fn text_command(
  text: String,
  pointsize: String,
  coords: String,
  colors: #(String, String),
) -> List(String) {
  let text = case string.length(text) > 28 {
    True -> text |> string.slice(0, 28) <> "..."
    False -> text
  }

  [
    "-font",
    "Montserrat-Black",
    "-pointsize",
    pointsize,
    "-fill",
    colors.1,
    "-annotate",
    coords,
    text,
  ]
}

fn album_command(
  albums: List(types.Album),
  colors: #(String, String),
) -> List(String) {
  albums
  |> list.index_map(fn(album, i) {
    let list_margin = 80

    let number_y = 165 + i * list_margin |> int.to_string
    let number_coords = "+480+" <> number_y

    let name_y = 148 + i * list_margin |> int.to_string
    let name_coords = "+540+" <> name_y

    let artist_y = 175 + i * list_margin |> int.to_string
    let artist_coords = "+540+" <> artist_y

    let cover_x = 815 + { i % 2 } * 190 |> int.to_string
    let cover_y = 30 + { i / 2 } * 190 |> int.to_string
    let cover_coords = "+" <> cover_x <> "+" <> cover_y

    [
      [
        "(",
        "album_" <> int.to_string(i) <> ".jpg_duotone.jpg",
        "-alpha",
        "set",
        "-channel",
        "A",
        "-evaluate",
        "set",
        "30%",
        ")",
        "-geometry",
        cover_coords,
        "-composite",
      ],
      text_command(int.to_string(i + 1), "55", number_coords, colors),
      text_command(album.name, "35", name_coords, colors),
      text_command(album.artist.name, "20", artist_coords, colors),
    ]
    |> list.flatten
  })
  |> list.flatten
}

fn generate_gradient(colors: #(String, String), gradient_path: String) -> String {
  let _ =
    shellout.command(
      run: "magick",
      with: [
        "-size",
        "400x401",
        "gradient:" <> colors.0 <> "-none",
        "-alpha",
        "set",
        "-channel",
        "A",
        "-fx",
        "p < 0.7 ? (1 - p / 0.7) : 0",
        "-crop",
        "400x400+0+0",
        gradient_path,
      ],
      in: ".",
      opt: [],
    )
    |> result.map(with: fn(output) {
      io.print(output)
      0
    })
    |> result.map_error(with: fn(detail) {
      let #(status, message) = detail
      let style =
        shellout.display(["bold", "italic"])
        |> dict.merge(from: shellout.color(["red"]))
      message
      |> shellout.style(with: style, custom: [])
      |> io.print_error
      status
    })

  gradient_path
}

pub fn generate_image(
  albums: List(types.Album),
  user_info: types.UserInfo,
  colors: #(String, String),
  extract_colors: Bool,
) -> Nil {
  download_user_image(user_info.image)
  let colors = case extract_colors {
    True -> extract_palette("images/user.jpg")
    False -> colors
  }

  albums
  |> list.map(fn(album) { album.image })
  |> download_album_covers
  |> list.map(fn(a) {
    generate_duotone_image(a, a <> "_duotone.jpg", "170", colors)
  })

  let album_args = album_command(albums, colors)
  generate_duotone_image(
    "images/user.jpg",
    "images/user_duotone.jpg",
    "600",
    colors,
  )
  generate_gradient(colors, "images/gradient.png")

  let _ =
    shellout.command(
      run: "magick",
      with: [
        [
          "-size",
          "1200x600",
          "xc:" <> colors.0,
          "user_duotone.jpg",
          "-geometry",
          "-240+0",
          "-composite",
          "(",
          "user.jpg",
          "-resize",
          "400x400",
          ")",
          "-geometry",
          "+60+100",
          "-composite",
          "gradient.png",
          "-geometry",
          "+60+100",
          "-composite",
        ],
        album_args,
        text_command(
          user_info.name |> string.uppercase,
          "35",
          "+70+485",
          colors,
        ),
        text_command("MOST LISTENED ALBUMS", "50", "+380+65", colors),
        text_command("LAST 30 DAYS", "22", "+380+90", colors),
        ["output.jpg"],
      ]
        |> list.flatten,
      in: "images/",
      opt: [],
    )

  let _ =
    shellout.command(run: "open", with: ["images/output.jpg"], in: ".", opt: [])

  Nil
}
