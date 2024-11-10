import gleam/dict
import gleam/erlang
import gleam/io
import gleam/list
import gleam/string
import images
import lastfm/albums
import lastfm/user

pub fn main() {
  let color_schemes =
    dict.from_list([
      #("midnight", #("#F7396F", "#16006F")),
      #("forest", #("#00D574", "#1A2A56")),
      #("ocean", #("#80f8f8", "#7a004f")),
      #("strawberry", #("#D7FD31", "#EA1264")),
      #("bumblebee", #("#ffea00", "#141209")),
      #("crimson", #("#dc2c2c", "#000000")),
      #("aqua", #("#68ebc1", "#0239d8")),
      #("lavender", #("#C9A7E7", "#5F4B8B")),
      #("emerald", #("#50C878", "#0B3D3B")),
      #("cherry", #("#FF4C94", "#C70039")),
      #("twilight", #("#7A5C8D", "#1C1B29")),
      #("flame", #("#FF4500", "#6A240E")),
      #("moss", #("#8A9A5B", "#4F4D40")),
    ])

  io.println("Available color schemes:")

  color_schemes
  |> dict.to_list
  |> list.map(fn(scheme) { " - " <> scheme.0 |> string.capitalise })
  |> string.join("\n")
  |> io.println
  io.println(" - Auto (based on your profile picture)")

  let assert Ok(color_scheme) = erlang.get_line("Enter a color scheme: ")
  let color_scheme = color_scheme |> string.trim |> string.lowercase

  let #(colors, extract_colors) = case color_scheme {
    "auto" -> #(#("#FFFFFF", "#000000"), True)
    _ -> {
      case dict.get(color_schemes, color_scheme) {
        Ok(colors) -> #(colors, False)
        Error(_) -> {
          io.println("Invalid color scheme; defaulting to auto")
          #(#("#FFFFFF", "#000000"), True)
        }
      }
    }
  }

  let assert Ok(lastfm_user) = erlang.get_line("Enter your Last.fm username: ")
  let lastfm_user = lastfm_user |> string.trim |> string.lowercase

  let assert Ok(api_key) = erlang.get_line("Enter your Last.fm API key: ")
  let api_key = api_key |> string.trim

  let assert Ok(top_albums) = albums.get_top_albums(api_key, lastfm_user)
  let albums = top_albums.top_albums.albums

  let assert Ok(user_info) = user.get_user_info(api_key, lastfm_user)
  let user_info = user_info.user
  images.generate_image(albums, user_info, colors, extract_colors)
}
