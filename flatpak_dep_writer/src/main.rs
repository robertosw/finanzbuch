use core::panic;
use std::io;
use std::io::BufRead;

// cargo build --release --package flatpak_dep_writer
// usage:
// ldd /lib/x86_64-linux-gnu/libwebkit2gtk-4.0.so.37.67.6  | ../../../target/release/flatpak_dep_writer  > output.yml

fn main()
{
    // pipe the output of ldd to this program
    let stdin = io::stdin();

    for line_res in stdin.lock().lines() {
        match line_res {
            Ok(line) => {
                // ldd line looks like this
                // libbrotlicommon.so.1 => /lib/x86_64-linux-gnu/libbrotlicommon.so.1 (0x00007fc710790000)

                // This is "libbrotlicommon.so.1"
                let lib_name_version = match line.split(" => ").collect::<Vec<&str>>().first() {
                    Some(str) => {
                        let string = String::from(*str);
                        String::from(string.trim_start())
                    }
                    None => panic!("lib name and version could not be extracted."),
                };

                let lib_name = match lib_name_version.split(".").collect::<Vec<&str>>().first() {
                    Some(str) => *str,
                    None => panic!("lib name could not be extracted."),
                };

                print!(
r#"
- name: {lib_name}
  buildsystem: simple
  sources:
    - type: file
      path: /usr/lib/x86_64-linux-gnu/{lib_name_version}
  build-commands:
    - "install -Dm644 {lib_name_version} /app/lib/{lib_name_version}"
"#
                );
            }
            Err(e) => panic!("Error unwrapping stdin line: {:?}", e),
        }
    }

    // for each line, generate a text block, print block
}
