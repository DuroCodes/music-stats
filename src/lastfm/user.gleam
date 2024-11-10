import decode/zero
import gleam/dynamic
import gleam/http/request
import gleam/httpc
import gleam/json
import gleam/list
import gleam/result
import lastfm/types

fn decode_user_info(
  body: String,
) -> Result(types.UserInfoResponse, json.DecodeError) {
  let user_decoder = {
    use name <- zero.field("name", zero.string)
    use images <- zero.field("image", zero.list(types.image_decoder()))
    let assert Ok(image) = images |> list.find(fn(i) { i.size == "extralarge" })
    zero.success(types.UserInfo(name:, image:))
  }

  let response_decoder = {
    use user <- zero.field("user", user_decoder)
    zero.success(types.UserInfoResponse(user:))
  }

  json.decode(body, zero.run(_, response_decoder))
}

pub fn get_user_info(
  api_key: String,
  user: String,
) -> Result(types.UserInfoResponse, dynamic.Dynamic) {
  let url =
    "http://ws.audioscrobbler.com/2.0/?method=user.getinfo&format=json&user="
    <> user
    <> "&api_key="
    <> api_key

  let assert Ok(req) = request.to(url)
  use resp <- result.try(httpc.send(req))

  case decode_user_info(resp.body) {
    Ok(user_info) -> Ok(user_info)
    Error(_) -> Error(dynamic.from(Nil))
  }
}
