use std::path::Path;
use prost_build::Config;
use std::{io::Result, fs};

// blatantly yoinked from @harscoet https://github.com/tokio-rs/prost/issues/173#issuecomment-509004188
fn rename_prost_generated_filenames(dir: &Path) -> Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let file_stem_renamed = &path
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace(".", "_");

                let extension = path.extension().unwrap().to_str().unwrap();
                fs::rename(&path, dir.join(format!("{}.{}", file_stem_renamed, extension))).unwrap();
            }
        }
    }

    Ok(())
}

fn main() {
    let _skip_protobuf_compilation = match std::env::var("SKIP_PROTOBUF") {
        Ok(_env_var) => return,
        _ => {}
    };

    let input_files = &["src/protobuf/untis.proto"];
    let output_directory = Path::new("src/protobuf");

    let mut prost = Config::new();
    prost.out_dir(output_directory)
        .compile_protos(input_files, &[""])
        .unwrap();

    rename_prost_generated_filenames(&output_directory).unwrap();
}

