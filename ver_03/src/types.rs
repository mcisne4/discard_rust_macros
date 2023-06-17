#[derive(Debug, deluxe::ExtractAttributes, Default)]
#[deluxe(attributes(parent))]
#[deluxe(default)]
pub struct ParentAttributes {
    pub a: u8,
    pub b: u8,
}

#[derive(Debug, deluxe::ExtractAttributes, Default)]
#[deluxe(attributes(child))]
#[deluxe(default)]
pub struct ChildAttributes(String);
impl ChildAttributes {
    fn needed_fields(&self) -> usize {
        let val = self.0.as_str();

        let mut count = 0_usize;
        let mut skip_next = false;

        for (idx, chr) in val.chars().enumerate() {
            if skip_next {
                skip_next = false;
                continue;
            }

            if let Some(next_chr) = val.chars().nth(idx + 1) {
                match next_chr {
                    '{' => skip_next = true,
                    _ => count += 1,
                }
            } else {
                count += 1;
            }
        }

        count
    }
}

pub struct VariantData {
    ident: String,
    to_str: String,
}

fn count_braces_0(input: &str) -> usize {
    let mut count = 0;
    let mut skip_next = false;

    for (i, c) in input.chars().enumerate() {
        if skip_next {
            skip_next = false;
            continue;
        }

        if c == '{' {
            if let Some(next_char) = input.chars().nth(i + 1) {
                if next_char == '{' {
                    skip_next = true;
                } else {
                    count += 1;
                }
            } else {
                count += 1;
            }
        }
    }

    count
}

fn count_braces(input: &str) -> usize {
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
                    '}' => {
                        if let Some(next_next_chr) = input.chars().nth(idx + 2) {
                            match next_next_chr {
                                '}' => skip_count = 2,
                                _ => {
                                    skip_count = 1;
                                    count += 1;
                                }
                            }
                        }
                    }
                    _ => skip_count = 1,
                }
            }
        }
    }

    count
}
