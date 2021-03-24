extern crate reqwest;
extern crate select;

use std::{thread, time};
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Write;

use select::document::Document;
use select::predicate::{Any, Name};
use string_builder::Builder;

struct Book {
  title: String,
  acts: Vec<u8>
}

fn main() -> std::io::Result<()> {
  let book = Book {
    title: String::from("awws-weww-that-ends-w-weww"),
    acts: vec![3, 5, 7, 5, 3]
  };
  for (act, scene_count) in book.acts.iter().enumerate() {
    let real_act = act + 1;
    create_dir_all(format!("../{}/act{}", book.title, real_act))?;
    for scene in 1..scene_count + 1 {
      let url = &format!("http://shakespeare.mit.edu/allswell/allswell.{}.{}.html", real_act, scene);
      let path = &format!("../{}/act{}/a{}s{}.md", book.title, real_act, real_act, scene);
      parse_scene(url, path);
      let sleepy_time = time::Duration::from_secs(5);
      println!("Sleeping for {} seconds", sleepy_time.as_secs().to_string());
      thread::sleep(sleepy_time)
    }
  }
  Ok(())
}

fn parse_scene(url: &str, path: &str) -> std::io::Result<()> {
  println!("{}", url);
  println!("{}", path);
  let resp = reqwest::blocking::get(url).unwrap();
  assert!(resp.status().is_success());

  let mut builder = Builder::default();

  let document = Document::from_read(resp).unwrap();
  let scene = document.find(Name("h3")).next().unwrap().text();

  // todo abstract into markdown builder
  builder.append("## ");
  builder.append(scene);
  builder.append("\n");

  for node in document.find(Any) {
    let name = node.name();
    if name.is_some() {
      match name.unwrap() {
        "a" => {
          if node.attr("name").is_some() {
            let name_attr = node.attr("name").unwrap();
            if name_attr.chars().all(char::is_numeric) {
              // text block
              builder.append(node.text().trim());
              builder.append("\n\n");
            } else {
              // char block
              builder.append("### ");
              builder.append(node.text().trim());
              builder.append("\n");
            }
          }
        }
        "i" => {
          builder.append("#### ");
          builder.append(node.text().trim());
          builder.append("\n");
        }
        _ => ()
      }
    }
  }
  let mut file = File::create(path)?;
  file.write_all(builder.string().unwrap().as_bytes())?;
  Ok(())
}