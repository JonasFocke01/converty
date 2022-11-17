use std::{env};

use colored::*;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    if args.len() == 0 {
        help(String::from("Too few arguments"));
    }
    match args.remove(0).as_str() {
        "-radix" => match radix(args) {
                        Ok(m) => print!("{}\n", m),
                        Err(error) => help(String::from(error))
                    },
        "-color" => match color_system(args) {
                        Ok(m) => print!("{}\n", m),
                        Err(error) => help(String::from(error))
                    },
        flag => help(String::from("of flag: ") + flag),
    }
}

fn help(flag: String) {
    print!("\n");
    print!("{} {}\n","Spawned help dialog because".bright_red(), flag.bright_red());
    print!("\n");
    print!("{}\n", "===================== CONVERTY ====================".green());
    print!("{}\n" , "Helpcenter. Here are some Converty modi you can use".bright_green());
    print!("\n");
    print!("{}\n", "-------------------NUMBER SYSTEM-------------------".green());
    print!("FLAG: -radix\n");
    print!("\n");
    print!("{}\n", "This needs a second flag\nindicating the given numbertype".bright_green());
    print!("\n");
    print!("Hexadecimal: -x\n");
    print!("Decimal: -d\n");
    print!("Binary: -b\n");
    print!("\n");
    print!("{}\n", "Possible command should look like this:".bright_green());
    print!("converty -radix -d 250\n");
    print!("converty -radix -x fa\n");
    print!("converty -radix -b 11111010\n");
    print!("\n");
    print!("{}\n", "-------------------COLOR SYSTEM--------------------".green());
    print!("FLAG: -color\n");
    print!("\n");
    print!("{}\n", "This needs a second flag\nindicating the given colortype".bright_green());
    print!("\n");
    print!("RGB: -rgb\n");
    print!("HEX: -hex\n");
    print!("\n");
    print!("{}\n", "Possible command should look like this:".bright_green());
    print!("converty -color -rgb 255,10,17\n");
    print!("converty -color -hex faffaa (note that you have to leave out the #)\n");

    std::process::exit(0);
}

fn radix(mut args: Vec<String>) -> Result<String, String> {
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
                    let input_as_number: u64 = match input_as_string.as_str().parse::<u64>() {
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

fn color_system(mut args: Vec<String>) -> Result<String, String> {
    if args.len() < 2 {
        return Err(String::from("Too few arguments"))
    } else if args.len() > 2 {
        return Err(String::from("Too many arguments"))
    }

    match args.remove(0).as_str() {
        "-rgb" => {
                    let input_as_string: String = args.remove(0);
                    let input_as_vec: Vec<&str> = input_as_string.split(",").collect();
                    let mut result_string = String::from("Conversion result:\n");

                    result_string.push_str(" RGB: ");
                    // ? rgb
                    for i in 0..3 {
                        result_string.push_str(input_as_vec[i]);
                        if i < 2 {
                            result_string.push_str(", ");
                        }
                    }

                    result_string.push('\n');

                    // ? hex
                    result_string.push_str(" HEX: #");
                    for i in 0..3 {
                        result_string.push_str(format!("{:x}", String::from(input_as_vec[i]).parse::<u64>().unwrap()).as_str());
                    }

                    Ok(
                        result_string
                    )
                },
        "-hex" => {
                    let input_as_string: String = args.remove(0);
                    let mut input_as_vec: Vec<&str> = vec!();
                    let mut result_string: String = String::from("Conversion result:\n");

                    input_as_vec.push(&input_as_string[..2]);
                    input_as_vec.push(&input_as_string[2..4]);
                    input_as_vec.push(&input_as_string[4..6]);

                    let mut input_as_number_vec = vec!();

                    for i in 0..3 {
                        input_as_number_vec.push(match read_hex_to_number(&mut String::from(input_as_vec[i])) {
                            Ok(n) => n,
                            Err(m) => return Err(m)
                        });
                    }

                    result_string.push_str(" RGB: ");
                    // ? rgb
                    for i in 0..3 {
                        result_string.push_str(input_as_number_vec[i].to_string().as_str());
                        if i < 2 {
                            result_string.push_str(", ");
                        }
                    }

                    result_string.push('\n');

                    // ? hex
                    result_string.push_str(" HEX: #");
                    for i in 0..3 {
                        result_string.push_str(format!("{:x}", input_as_number_vec[i]).as_str());
                    }

                    Ok(
                        result_string
                    )
                },
        arg => return Err(String::from("Found wrong argument: ") + arg)
    }
}