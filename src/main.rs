use std::env;
use std::fs;
use std::io::stdout;
use std::io::Read;
use std::io::Write;
use std::process;

fn main() {
    let argv: Vec<String> = env::args().collect();

    let filename;

    if argv.len() == 2 {
        filename = &argv[1];
    } else {
        eprintln!("Usage:\n\n    {} [brainfrick.bf]\n", &argv[0]);
        process::exit(1);
    }

    let program = fs::read_to_string(filename).expect("Unable to read file");
    let output = execute_program(program);

    println!("\nOutput: \n");
    stdout()
        .write_all(&output)
        .expect("Failed to write output bytes");
}

fn execute_program(program_code: String) -> Vec<u8> {
    let program = program_code.as_bytes();
    let mut memory: [u8; 4096] = [0; 4096];
    let mut output: Vec<u8> = Vec::new();

    let entry: usize = 0;
    let mut pointer: u16 = 0;

    execute_loop(&program, &mut memory, &mut output, entry, &mut pointer);

    return output;
}

fn execute_loop(
    program: &[u8],
    memory: &mut [u8; 4096],
    output: &mut Vec<u8>,
    _ip: usize,
    pointer: &mut u16,
) -> usize {
    let mut ip: usize = _ip;
    let mut iterator: u16 = 0;
    let mut _iterator: Option<u16> = Some(0);

    loop {
        if ip >= program.len() {
            break;
        }

        let instruction: String = byte_to_str(program[ip]);

        let curval = memory[*pointer as usize];
        println!(
            "------ ip:{} ------ i:{} val:{}, ptr:{}",
            ip, instruction, curval, *pointer
        );

        match instruction.as_ref() {
            ">" => {
                *pointer += 1;
            }

            "<" => {
                *pointer -= 1;
            }

            "+" => {
                let mut val: u8 = memory[*pointer as usize];

                println!("add: {}", val);

                if val == 0xff {
                    val = 0x00;
                } else {
                    val += 1;
                }

                println!("... {}", val);

                memory[*pointer as usize] = val;
            }

            "-" => {
                let mut val: u8 = memory[*pointer as usize];

                println!("sub: {}", val);

                if val == 0x00 {
                    val = 0xff;
                } else {
                    val -= 1;
                }

                println!("... {}", val);

                memory[*pointer as usize] = val;
            }

            "." => {
                let byte = memory[*pointer as usize];
                // println!("byte: {}", byte);
                output.push(byte);
            }

            "," => {
                memory[*pointer as usize] = read_byte().expect("Failed to read from stdin") as u8;
            }

            // start loops
            "[" => {
                let curval = memory[*pointer as usize];

                if curval != 0 {
                    ip += 1;
                    println!("Entering loop at ip={}", ip);
                    ip = execute_loop(program, memory, output, ip, pointer);
                } else {
                    println!("Not entering loop as curval is 0 at ip={}", ip);
                    let mut open_count = 1;

                    ip += 1;

                    while ip < program.len() && open_count > 0 {
                        ip += 1;
                        let curinst = byte_to_str(program[ip]);
                        println!("... i:{}, ip:{}, level:{}", curinst, ip, open_count);

                        if curinst == "[" {
                            open_count += 1;
                        } else if curinst == "]" {
                            open_count -= 1;
                        }
                    }

                    println!("... exited loop at ip:{}", ip);
                }
            }

            "{" => {
                match _iterator {
                    None => {
                        iterator = *pointer;
                    }

                    Some(_val) => {}
                };

                let curval = memory[iterator as usize];

                if curval != 0 {
                    *pointer += 1;
                    ip += 1;
                    println!("Entering for loop at ip={}, iter:{}", ip, iterator);
                    ip = execute_loop(program, memory, output, ip, pointer);
                } else {
                    println!(
                        "Not entering for loop as curval is 0 at ip={}, iter:{}",
                        ip, iterator
                    );
                    let mut open_count = 1;

                    iterator -= 1;
                    ip += 1;

                    while ip < program.len() && open_count > 0 {
                        ip += 1;
                        let curinst = byte_to_str(program[ip]);
                        println!("... i:{}, ip:{}, level:{}", curinst, ip, open_count);

                        if curinst == "{" {
                            open_count += 1;
                        } else if curinst == "}" {
                            open_count -= 1;
                        }
                    }

                    println!("... exited for loop at ip:{}", ip);
                }
            }

            // end loops
            "]" => {
                let curval = memory[*pointer as usize];

                println!(
                    "Returning from loop at curval:{}, ip={}, len={}",
                    curval,
                    ip,
                    program.len()
                );

                if _ip == 0 {
                    break;
                }

                ip = _ip - 1 - 1;
            }

            "}" => {
                let curval = memory[iterator as usize];

                println!(
                    "Returning from loop at curval:{}, ip={}, len={}",
                    curval,
                    ip,
                    program.len()
                );

                memory[iterator as usize] -= 1;

                if _ip == 0 {
                    break;
                }

                ip = _ip - 1 - 1;
            }

            // other
            &_ => {
                // ignore instruction; it's a comment
            }
        }

        // post-loop checks

        ip += 1;
        // println!("");
    }

    return ip;

    /*
     *
     *  BUG:::::::
     *
     *  https://github.com/io12/brainfsym/blob/master/src/main.rs#L7
     *
     *  don't check if the curval is 0 at the ], do it at the [ !!!
     *
     *
     *
     *
     * */
}

fn byte_to_str(byte: u8) -> String {
    return String::from_utf8(vec![byte]).expect("Failed to convert byte to str");
}

fn read_byte() -> Option<i32> {
    return std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as i32);
}
