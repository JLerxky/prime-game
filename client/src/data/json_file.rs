use bytes::BytesMut;
use protocol::data::tile_map_data::TileMapData;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

pub async fn read_file() -> io::Result<TileMapData> {
    let mut f = File::open("foo.json").await?;
    let mut buffer = BytesMut::new();

    f.read_buf(&mut buffer).await?;
    let _data: TileMapData = serde_json::from_slice(&buffer[..])?;
    Ok(_data)
}
