// Test it with the following commands:
// curl -X DELETE http://localhost:8080/datafile.txt
// curl -X GET http://localhost:8080/datafile.txt
// curl -X PUT http://localhost:8080/datafile.txt -d "File contents."
// curl -X POST http://localhost:8080/data -d "File contents."
// curl -X GET http://localhost:8080/a/b

use actix_web::{delete, Error};
use actix_web::{web, web::Path, App, HttpRequest, HttpResponse, HttpServer, Responder};
use futures::{
    future::{ok, Future},
    Stream,
};
use rand::prelude::*;
use std::fs::{File, OpenOptions};
use std::io::Write;

fn flush_stdout() {
    std::io::stdout().flush().unwrap();
}

#[delete()]
async fn delete_file(info: Path)

fn main() {}
