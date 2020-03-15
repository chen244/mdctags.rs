use std::fs;
use std::process;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("!_TAG_FILE_FORMAT       2       /extended format; --format=1 will not append ;\" to lines/'
!_TAG_FILE_SORTED       0       /0=unsorted, 1=sorted, 2=foldcase/'
!_TAG_PROGRAM_AUTHOR    wsdjeg /wsdkeg@outlook.com/'
!_TAG_PROGRAM_NAME      mdctags        //'
!_TAG_PROGRAM_URL       https://github.com/wsdjeg/mdctags /official site/'
!_TAG_PROGRAM_VERSION   0.1.0   //'
");
    if args.len() == 0 {
        process::exit(0);
    }
    let contents = fs::read_to_string("README.md").expect("");
    let mut in_code = false;
    let mut line_no = 0;
    for line in contents.lines() {
        line_no = line_no + 1;
        if line.starts_with("```") {
            in_code = !in_code;
        }
        if !in_code && line.starts_with("#") {
            let title: String = line.split_whitespace().collect()[1];
            println!("{}", line);
        }
    }
}
