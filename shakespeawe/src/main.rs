extern crate reqwest;
extern crate select;

use std::fs::create_dir_all;
use std::fs::File;
use std::io::Write;

use select::document::Document;
use select::predicate::{Any, Name};
use string_builder::Builder;

fn main() -> std::io::Result<()> {
  let act = 3;
  let scene = 5;
  let book = "awws-weww-that-ends-w-weww";
  create_dir_all(format!("../{}/act{}", book, act))?;
  parse_scene(&format!("http://shakespeare.mit.edu/allswell/allswell.{}.{}.html", act, scene), &format!("../{}/act{}/a{}s{}.md", book, act, act, scene))
}

fn parse_scene(url: &str, path: &str) -> std::io::Result<()> {
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