/// Merger:
/// Merge two or more yaml files
///
/// $ cat foo.yml
/// foo: bar
/// foobar: bar
/// $ cat bar.yml
/// foo: foo
/// bar:
///   foo: test
///   list:
///     - foo
///     - bar
///     - foobar
///
/// $ merger foo.yml bar.yml
/// foobar: bar
/// foo: foo
/// bar:
///   foo: test
///   list:
///     - foo
///     - bar
///     - foobar
///
/// This is particularly useful when you use shared pipelines and the configuration is yaml based
/// so you can override values per env or repo, used in conjunction with another tool like gomplate
/// takes this approach to another level.
///
/// Tests:
/// $ cargo test
///   Compiling merger v0.1.0 (/home/kainlite/Webs/merger)
///    Finished test [unoptimized + debuginfo] target(s) in 0.58s
///     Running target/debug/deps/merger-27e528d977271ee0
///
/// running 0 tests
///
/// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
///
///      Running target/debug/deps/merger-d8ef74b95964bf7e
///
/// running 2 tests
/// test tests::open_file_test ... ok
/// test tests::merge_test ... ok
///
/// test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
///
///    Doc-tests merger
///
/// running 0 tests
///
/// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
use merge_yaml_hash::MergeYamlHash;
use std::env;
use std::io::prelude::*;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let mut files = vec![];

    let doc = r#"
Merger: merges yaml files recursively and prints the
result back to the terminal, it's useful for CI pipelines

Usage:
merger file-1.yml file-2.yaml file-n.yaml
"#;

    if args.len() > 1 {
        for filename in args.split_off(1) {
            let contents = open_file(filename);
            files.push(String::from(contents));
        }

        let result = merge(files);
        println!("{}", result)
    } else {
        println!("{}", doc)
    }
}

fn open_file(filename: String) -> String {
    let mut file = std::fs::File::open(filename).expect("Couldn't find or open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Couldn't read the contents of the file");
    contents
}

fn merge(files: Vec<String>) -> String {
    let mut hash = MergeYamlHash::new();

    for file in files {
        hash.merge(&file);
    }

    hash.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_file_test() {
        let str = open_file(String::from("foo.yml"));
        assert_eq!(str, "foo: bar\nfoobar: bar\n");
    }

    #[test]
    fn merge_test() {
        let str = open_file(String::from("foo.yml"));
        let str2 = open_file(String::from("bar.yml"));

        let files = vec![str, str2];

        let result = merge(files);
        let expected_result =
            "foobar: bar\nfoo: foo\nbar:\n  foo: test\n  list:\n    - foo\n    - bar\n    - foobar";
        assert_eq!(result, expected_result);
    }
}
