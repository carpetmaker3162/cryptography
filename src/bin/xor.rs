use std::io::Write;

fn main() {
    println!("Welcome to the XOR encryptor by EarthTraveller1!");
    print!("Enter the file you would like to encrypt / decrypt: ");
    
    let mut stdout = std::io::stdout();
    stdout.flush().unwrap();
    
    let stdin = std::io::stdin();
    let mut file_name = String::new();
    stdin.read_line(&mut file_name).expect("io error or something idk");
    file_name = file_name.trim().to_string();
    
    let contents = std::fs::read_to_string(file_name.as_str()).unwrap_or_else(|_| {
        println!("Can't read or access {}. Sorry about that.", file_name);
        std::process::exit(-1);
    });
    
    print!("Now, enter a key (must be between 0 and 255): ");
    stdout.flush().unwrap();
    
    let mut key = String::new();
    stdin.read_line(&mut key).expect("io error or something idk");
    key = key.trim().to_string();
    
    let key = key.parse::<u8>().unwrap_or_else(|_| {
        println!("'{}' is either too large or not a valid integer, my friend.", key);
        std::process::exit(-1);
    });
    
    let encrypted_contents = contents.bytes().map(|byte| byte ^ key).collect::<Vec<u8>>();
    
    let mut output = std::fs::File::create(file_name.as_str()).unwrap_or_else(|_| {
        println!("Can't open {} for writing.", file_name);
        std::process::exit(-1);
    });
    
    output.write_all(&encrypted_contents[..]).unwrap_or_else(|_| {
        println!("Can't write to {}.", file_name);
        std::process::exit(-1);
    });
    
    println!("The file has been successfully encrypted / decrypted!");
}