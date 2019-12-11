#[allow(unused_imports)]
use rusoto_core::credential::{AwsCredentials, DefaultCredentialsProvider, ProvideAwsCredentials};
use rusoto_core::{ByteStream, Region};
use rusoto_s3::util::{PreSignedRequest, PreSignedRequestOption};
use rusoto_s3::{GetObjectRequest, PutObjectRequest, S3Client, S3};
use std::fs::File;

fn main() {
    #[allow(unused_variables)]
    let client = build_client();
    // print_object(&client, "sample1.mp4".to_owned());
    // put_object(&client);
    let url = presigned_url("sample1.mp4".to_owned());
    println!("{:?}", url);
}

fn build_client() -> S3Client {
    S3Client::new(build_region())
}

fn build_region() -> Region {
    Region::Custom {
        name: "minio".to_owned(),
        endpoint: "http://localhost:9001".to_owned(),
    }
}

fn presigned_url(key: String) -> String {
    let get_obj_req = GetObjectRequest {
        bucket: "videos".to_owned(),
        key: key,
        ..Default::default()
    };

    let option = PreSignedRequestOption {
        expires_in: ::std::time::Duration::from_secs(60),
    };

    let region = build_region();

    let credentials = AwsCredentials::new("minio", "minio123", None, None);

    get_obj_req.get_presigned_url(&region, &credentials, &option)
}

#[allow(dead_code)]
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
