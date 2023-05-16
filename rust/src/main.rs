use actix_files;
use actix_web::{App,get,delete,post, HttpServer, Responder, web, HttpRequest, HttpResponse};
use std::*;
use serde::{Serialize, Deserialize};
use serde_json;
use actix_multipart::Multipart;
use futures_util::{StreamExt as _, TryStreamExt};
use tokio::io::AsyncWriteExt;
use tokio::fs;
const UPLOAD_DIR: &str = "./uploads/";

#[derive(Serialize, Deserialize, Debug)]
struct Files{
  files: Vec<String>,
}
#[derive(Deserialize)]
struct del_rtrv_query{
  filename: String
}
async fn readUploadDir() -> String{
  let mut cur_files: Files=Files { files: Vec::new() };
  cur_files.files = Vec::new();
  let mut read_filenames = fs::read_dir(UPLOAD_DIR).await.unwrap();
  while let Some(entry) = read_filenames.next_entry().await.unwrap(){
    let file_name = entry.file_name();
    cur_files.files.push(file_name.to_str().unwrap().to_string());
  }
  let serialized = serde_json::to_string(&cur_files).unwrap();
  return serialized;
}

#[delete("delete")]
async fn deleteRoute(info: web::Query<del_rtrv_query>)-> impl Responder{
  let filename_to_del: String = info.filename.clone();
  if let _ = fs::remove_file(UPLOAD_DIR.to_owned()+&filename_to_del){
    return readUploadDir().await;
  }
  HttpResponse::BadRequest();
  return readUploadDir().await;
}

#[get("/retrieve")]
async fn retrieveRoute(info: web::Query<del_rtrv_query>)-> actix_files::NamedFile{
  let filename_to_rtrv = &info.filename; 
  return actix_files::NamedFile::open(UPLOAD_DIR.to_owned()+&filename_to_rtrv).unwrap();
}

#[get("/sync")]
async fn sync()-> impl Responder{
  return readUploadDir().await;
}

#[post("/upload-file")]
async fn uploadRoute(mut payload: Multipart, req: HttpRequest)-> impl Responder{
    loop{
      if let Ok(Some(mut field)) = payload.try_next().await{
        // field.name() == "upload"

        let full_filename = format!("{}{}",
        UPLOAD_DIR,
        field.content_disposition().get_filename().unwrap()
      );
        let mut saved_file = fs::File::create(full_filename).await.unwrap();
        while let Ok(Some(chunk)) = field.try_next().await{
          let _ = saved_file.write_all(&chunk).await.unwrap();
        }

      } else {
          break;
      }

    }

  return readUploadDir().await;
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| 
        App::new()
        .service(sync)
        .service(deleteRoute)
        .service(retrieveRoute)
        .service(uploadRoute)
        .service(actix_files::Files::new("/", "./dist/").index_file("index.html"))
        )
        .bind(("127.0.0.1", 9001))?
        .run()
        .await
}
