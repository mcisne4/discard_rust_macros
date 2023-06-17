mod dev_ver_01;
mod dev_ver_02;
mod dev_ver_03;

use dev_ver_01::run_version_01;
use dev_ver_02::run_version_02;
use dev_ver_03::run_version_03;

fn main() {
    run_version_01(false);
    run_version_02(false);
    run_version_03(false);

    format!("{}{}{{}}{{{}}}", 1, 2, 3);

    count_braces("{} lorem {}", 2);
    count_braces("{input}{{", 0);
    count_braces("{{} {}} {}{}", 2);

    let mut soup = String::new();
    soup += "{}";

    println!("SOUP: {}", soup);

    use std::fmt::Write;

    // let mut fmt = String::new();
    // let x = write!(&mut fmt, "Hello {}", "world", "!!");
    // let fmt = std::fmt::Formatter::;
    // let x = std::fmt::Display::fmt(&self, f)
}

fn count_braces(input: &str, expected: usize) {
    let mut count = 0_usize;
    let mut skip_count = 0_usize;

    for (idx, chr) in input.chars().enumerate() {
        if skip_count > 0 {
            skip_count -= 1;
            continue;
        }

        if chr == '{' {
            if let Some(next_chr) = input.chars().nth(idx + 1) {
                match next_chr {
                    '{' => skip_count = 1,
                    // '}' => {
                    //     let next_next_chr = input.chars().nth(idx + 2);
                    //     match next_next_chr {
                    //         Some(chr)=> match chr {
                    //             '}' => skip_count
                    //         }
                    //     }

                    //     if let Some(next_next_chr) = input.chars().nth(idx + 2) {
                    //         match next_next_chr {
                    //             '}' => skip_count = 2,
                    //             _ => {
                    //                 skip_count = 1;
                    //                 count += 1;
                    //             }
                    //         }
                    //     }
                    // }
                    _ => skip_count = 1,
                }
            }
        }
    }

    let pass = count == expected;
    println!(
        "String: '{}':\nExpected Count: {}\nActual Count: {}\nPass: {}\n",
        input, expected, count, pass
    );
}
