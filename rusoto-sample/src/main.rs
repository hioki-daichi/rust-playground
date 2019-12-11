use rusoto_core::{ByteStream, Region};
use rusoto_s3::{GetObjectRequest, PutObjectRequest, S3Client, S3};
use std::fs::File;

fn main() {
    let client = build_client();
    // print_object(&client, "sample1.mp4".to_owned());
    put_object(&client);
}

fn build_client() -> S3Client {
    S3Client::new(Region::Custom {
        name: "minio".to_owned(),
        endpoint: "http://localhost:9001".to_owned(),
    })
}

fn put_object(client: &S3Client) {
    let mut file = File::open("sample1.mp4").unwrap();
    let mut buf = Vec::new();
    use std::io::Read;
    let _ = file.read_to_end(&mut buf);
    let put_obj_req = PutObjectRequest {
        bucket: "videos".to_owned(),
        key: "sample1.mp4".to_owned(),
        body: Some(ByteStream::from(buf)),
        content_type: Some("video/mp4".to_owned()),
        ..Default::default()
    };
    client.put_object(put_obj_req).sync().unwrap();
}

#[allow(dead_code)]
fn print_object(client: &S3Client, key: String) {
    let get_obj_req = GetObjectRequest {
        bucket: "videos".to_owned(),
        key: key,
        ..Default::default()
    };

    let result = client.get_object(get_obj_req).sync().unwrap();
    let mut stream = result.body.unwrap().into_blocking_read();
    let mut body = Vec::new();
    use std::io::Read;
    stream.read_to_end(&mut body).unwrap();
    println!("{:?}", body);
}
