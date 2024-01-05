use std::{env, path::PathBuf, fs};

fn assemble_pages(
  root_index_html_path: &PathBuf,
  root_pages_directory_path: &PathBuf,
  dist_path: &PathBuf,
) {
  let root_template_content = fs::read_to_string(root_index_html_path).expect("Failed to read index.html");
  let page_file_names = fs::read_dir(root_pages_directory_path).expect("Failed to read pages directory")
    .map(|res| res.map(|e| e.path()))
    .collect::<Result<Vec<_>, std::io::Error>>().expect("Failed to collect page file names");

  for page_file_name in page_file_names {
    let page_file_content = fs::read_to_string(&page_file_name).expect("Failed to read page file");
    let page_file_stem = page_file_name.file_stem().expect("Failed to get page file stem");
    let page_file_name_without_extension = page_file_stem
      .to_str().expect("Failed to convert page file name without extension to string");
    let page_file_name_with_html_extension = format!("{}.html", page_file_name_without_extension);
    let output_page_file_path = dist_path.join(page_file_name_with_html_extension);
    let page_file_content = root_template_content.replace("<% PAGE %>", &page_file_content);

    fs::DirBuilder::new().recursive(true).create(dist_path).expect("Failed to create dist directory");
    fs::write(output_page_file_path, page_file_content).expect("Failed to write page file");
  }
}

fn main() {
  let my_path = env::current_dir().expect("Failed to get current directory");
  let root_index_html_path = my_path.join("index.html");
  let root_pages_directory_path = my_path.join("pages");
  let file_exists = root_index_html_path.exists();
  let dist_directory_path = my_path.join("dist");

  if !file_exists {
    panic!("index.html not found in root directory");
  }

  assemble_pages(&root_index_html_path, &root_pages_directory_path, &dist_directory_path);
}
