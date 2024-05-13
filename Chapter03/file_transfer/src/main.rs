// Test it with the following commands:
// curl -X DELETE http://localhost:8080/datafile.txt
// curl -X GET http://localhost:8080/datafile.txt
// curl -X PUT http://localhost:8080/datafile.txt -d "File contents."
// curl -X POST http://localhost:8080/data -d "File contents."
// curl -X GET http://localhost:8080/a/b

use std::io::Write;

use actix_web::{
    web::{self, Path},
    App, Error, HttpResponse, HttpServer,
};
use futures::{
    future::{ok, Future},
    StreamExt, TryStreamExt,
};

fn flush_stdout() {
    std::io::stdout().flush().unwrap();
}

fn upload_new_file(
    payload: web::Payload,
    info: Path<(String,)>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let filename_prefix = info.0.clone();
    print!("Uploading file \"{}*.txt\" ... ", filename_prefix);
    flush_stdout();

    payload
        .map_err(Error::from)
        .fold(web::BytesMut::new(), move |mut body, chunk| {
            body.extend_from_slice(&chunk);
            Ok::<_, Error>(body)
        })
        .and_then(move |contents| {
            let mut rng = rand::thread_rng();
            let mut attempts = 0;
            let mut file;
            let mut filename;
            const MAX_ATTEMPTS: u32 = 100;

            loop {
                attempts += 1;
                if attempts > MAX_ATTEMPTS {
                    println!(
                        "Failed to create new file with prefix \"{}\", \
                         after {} attempts.",
                        filename_prefix, MAX_ATTEMPTS
                    );
                    return ok(HttpResponse::NotFound().into());
                }

                // Generate a 3-digit pseudo-random number.
                // and use it to create a file name.
                filename = format!("{}{:03}.txt", filename_prefix, rng.gen_range(0, 1000));

                // Create a not-yet-existing file.
                file = OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&filename);

                // If it was created, exit the loop.
                if file.is_ok() {
                    break;
                }
            }

            // Write the contents into it synchronously.
            if file.unwrap().write_all(&contents).is_err() {
                println!("Failed to write file \"{}\"", filename);
                return ok(HttpResponse::NotFound().into());
            }

            println!("Uploaded file \"{}\"", filename);
            ok(HttpResponse::Ok().content_type("text/plain").body(filename))
        })
}
fn main() -> std::io::Result<()> {
    let server_address = "127.0.0.1:8080";
    println!("Listening at address {}", server_address);
    // HttpServer::new(|| {}).bind(server_address).run()
    Ok(())
}
