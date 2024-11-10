import decode/zero

pub type Artist {
  Artist(name: String)
}

pub type Image {
  Image(size: String, text: String)
}

pub type Album {
  Album(artist: Artist, name: String, play_count: String, image: Image)
}

pub type TopAlbums {
  TopAlbums(albums: List(Album))
}

pub type TopAlbumsResponse {
  TopAlbumsResponse(top_albums: TopAlbums)
}

pub type UserInfo {
  UserInfo(name: String, image: Image)
}

pub type UserInfoResponse {
  UserInfoResponse(user: UserInfo)
}

pub fn image_decoder() {
  use size <- zero.field("size", zero.string)
  use text <- zero.field("#text", zero.string)
  zero.success(Image(size:, text:))
}
