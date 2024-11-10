import decode/zero
import gleam/dynamic
import gleam/http/request
import gleam/httpc
import gleam/json
import gleam/list
import gleam/result
import lastfm/types

fn decode_top_albums(
  body: String,
) -> Result(types.TopAlbumsResponse, json.DecodeError) {
  let artist_decoder = {
    use name <- zero.field("name", zero.string)
    zero.success(types.Artist(name:))
  }

  let album_decoder = {
    use artist <- zero.field("artist", artist_decoder)
    use name <- zero.field("name", zero.string)
    use play_count <- zero.field("playcount", zero.string)
    use images <- zero.field("image", zero.list(types.image_decoder()))
    let assert Ok(image) = images |> list.find(fn(i) { i.size == "extralarge" })
    zero.success(types.Album(artist:, name:, play_count:, image:))
  }

  let top_albums_decoder = {
    use albums <- zero.field("album", zero.list(album_decoder))
    zero.success(types.TopAlbums(albums:))
  }

  let response_decoder = {
    use top_albums <- zero.field("topalbums", top_albums_decoder)
    zero.success(types.TopAlbumsResponse(top_albums:))
  }

  json.decode(body, zero.run(_, response_decoder))
}

pub fn get_top_albums(
  api_key: String,
  user: String,
) -> Result(types.TopAlbumsResponse, dynamic.Dynamic) {
  let url =
    "http://ws.audioscrobbler.com/2.0/?method=user.gettopalbums&format=json&limit=6&period=1month&user="
    <> user
    <> "&api_key="
    <> api_key

  let assert Ok(req) = request.to(url)
  use resp <- result.try(httpc.send(req))

  case decode_top_albums(resp.body) {
    Ok(top_albums) -> Ok(top_albums)
    Error(_) -> Error(dynamic.from(Nil))
  }
}
