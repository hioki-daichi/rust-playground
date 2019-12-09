use rusoto_core::Region;
use rusoto_s3::{GetObjectRequest, S3Client, S3};
use std::io::Read;

fn main() {
    let client = S3Client::new(Region::Custom {
        name: "minio".to_owned(),
        endpoint: "http://localhost:9001".to_owned(),
    });

    let get_obj_req = GetObjectRequest {
        bucket: "videos".to_owned(),
        key: "sample1.mp4".to_owned(),
        ..Default::default()
    };

    let result = client.get_object(get_obj_req).sync().unwrap();
    let mut stream = result.body.unwrap().into_blocking_read();
    let mut body = Vec::new();
    stream.read_to_end(&mut body).unwrap();
    println!("{:?}", body);

    // TODO: put_object
    // TODO: get presigned url
}
