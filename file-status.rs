#[derive (Debug)]

struct File {
    name: String,
    data: Vec<u8>,
}

fn main()( {
    let f = File (
        name: String::from("file-status.rs"),
        data: Vec::new(),
    };

    let f_name = &f.name;
    let f_length = &f.data.len();

    println!("{:?}", f);
    println!("{} is {} bytes long", f_name, f_length);
}

