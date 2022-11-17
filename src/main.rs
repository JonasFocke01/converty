use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    if args.len() == 0 {
        help(String::from("Too few arguments"));
    }
    match args.remove(0).as_str() {
        "-n" => match number_system(args) {
                        Ok(m) => print!("{}\n", m),
                        Err(error) => help(String::from(error))
                    },
        flag => help(String::from("of flag: ") + flag),
    }
}

fn help(flag: String) {
    print!("\n");
    print!("Spawned help dialog because {}\n", flag);
    print!("\n");
    print!("CONVERTY supports the following modi:\n");
    print!("\n");
    print!("------------NUMBER SYSTEM------------\n");
    print!("FLAG: -n\n");
    print!("\n");
    print!("This needs a second flag indicating the given numbertype\n");
    print!("\n");
    print!("Hexadecimal: -x\n");
    print!("Decimal: -d\n");
    print!("Binary: -b\n");
    print!("\n");
    print!("A possible command should look like this:\n");
    print!("converty -n -d 17\n");

    std::process::exit(0);
}

fn number_system(mut args: Vec<String>) -> Result<String, String> {
    if args.len() < 2 {
        return Err(String::from("Too few arguments"))
    } else if args.len() > 2 {
        return Err(String::from("Too many arguments"))
    }

    match args.remove(0).as_str() {
        "-x" => {
                    let mut input_as_string: String = args.remove(0);
                    let input_as_number: u64 = match read_hex_to_number(&mut input_as_string) {
                        Ok(n) => n,
                        Err(m) => return Err(m)
                    };
                    Ok(
                        String::from("Conversion result: ") +
                        "\n Hex: " + format!("{:x}", input_as_number).as_str() +
                        "\n Bin: " + format!("{:b}", input_as_number).as_str() + " (" + format!("{:b}", input_as_number).chars().count().to_string().as_str() + " digits)" +
                        "\n Dec: " + input_as_number.to_string().as_str())
                },
        "-d" => {
                    let input_as_string: String = args.remove(0);
                    let input_as_number: u64 = match str::trim(input_as_string.as_str()).parse::<u64>() {
                        Ok(n) => n,
                        Err(_) => return Err(String::from("error while parsing ") + input_as_string.as_str())
                    };
                    Ok(
                        String::from("Conversion result: ") +
                        "\n Hex: " + format!("{:x}", input_as_number).as_str() +
                        "\n Bin: " + format!("{:b}", input_as_number).as_str() + " (" + format!("{:b}", input_as_number).chars().count().to_string().as_str() + " digits)" +
                        "\n Dec: " + input_as_number.to_string().as_str())
                },
        "-b" => {
                    let mut input_as_string: String = args.remove(0);
                    let input_as_number: u64 = match read_bin_to_number(&mut input_as_string) {
                        Ok(n) => n,
                        Err(m) => return Err(m)
                    };
                    Ok(
                        String::from("Conversion result: ") +
                        "\n Hex: " + format!("{:x}", input_as_number).as_str() +
                        "\n Bin: " + format!("{:b}", input_as_number).as_str() + " (" + format!("{:b}", input_as_number).chars().count().to_string().as_str() + " digits)" +
                        "\n Dec: " + input_as_number.to_string().as_str())
                },
        arg => return Err(String::from("Found wrong argument: ") + arg)
    }
}

fn read_bin_to_number(input: &mut String) -> Result<u64, String> {

    let mut input_as_bytes = input.to_string().into_bytes();
    let mut result = 0;
    let mut weight = 1;

    for _ in 0..input_as_bytes.len() {
        let current_byte = match  input_as_bytes.pop() {
            None => return Err(String::from("could not convert binary string into bytes")),
            m => m.unwrap()
        };
        if current_byte == 48 || current_byte == 49 {
            result += (current_byte as u64 - 48) * weight;
        } else {
            return Err(String::from("could not convert binary string into u64"));
        }
        weight *= 2;
    }

    Ok(result)
}

fn read_hex_to_number(input: &mut String) -> Result<u64, String> {

    let mut input_as_bytes = input.to_string().into_bytes();
    let mut result = 0;
    let mut weight = 1;

    for _ in 0..input_as_bytes.len() {
        let current_byte = match  input_as_bytes.pop() {
            None => return Err(String::from("could not convert hex string into bytes")),
            m => m.unwrap()
        };
        print!("{}\n", current_byte);
        if current_byte ==  48 ||
           current_byte ==  49 ||
           current_byte ==  50 ||
           current_byte ==  51 ||
           current_byte ==  52 ||
           current_byte ==  53 ||
           current_byte ==  54 ||
           current_byte ==  55 ||
           current_byte ==  56 ||
           current_byte ==  57 {
           result += (current_byte as u64 - 48) * weight;
        } else if current_byte ==  97 ||
           current_byte ==  98 ||
           current_byte ==  99 ||
           current_byte == 100 ||
           current_byte == 101 ||
           current_byte == 102 {
            result += (current_byte as u64 - 87) * weight;
        } else {
            return Err(String::from("could not convert hex string into u64"));
        }
        weight *= 16;
    }

    Ok(result)
}