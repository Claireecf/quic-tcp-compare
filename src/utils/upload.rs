use rocket::Data;

#[post("/upload", format = "plain", data = "<data>")]
fn upload(data: Data) -> Result<String, std::io::Error> {
    data.stream_to_file("/tmp/upload.txt").map(|n| n.to_string())
}