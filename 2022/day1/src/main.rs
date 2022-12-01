use std::io;

struct Elf {
    foods: Vec<u32>,
}

enum ElfLoadError {
    IOError,
    BadInteger,
}

enum ElfLoadResult {
    EndOfFile,
    EndOfElf,
}

impl Elf {
    fn load(f: &mut impl io::BufRead)
            -> Result<(ElfLoadResult, Elf), ElfLoadError> {
        let mut foods = Vec::<u32>::new();
        let mut s = String::new();

        loop {
            s.clear();

            match f.read_line(&mut s) {
                Ok(0) => return Ok((ElfLoadResult::EndOfFile, Elf { foods })),
                Ok(_) => {
                    let line = s.trim_end();

                    if line.is_empty() {
                        return Ok((ElfLoadResult::EndOfElf, Elf { foods }));
                    }

                    match line.parse::<u32>() {
                        Ok(weight) => foods.push(weight),
                        Err(..) => return Err(ElfLoadError::BadInteger)
                    }
                },
                Err(..) => return Err(ElfLoadError::IOError)
            }
        }
    }

    fn total_carrying(&self) -> u32 {
        return self.foods.iter().sum();
    }
}

fn read_elves(f: &mut impl io::BufRead) -> Result<Vec<Elf>, ElfLoadError> {
    let mut elves = Vec::<Elf>::new();

    loop {
        let (result, elf) = Elf::load(f)?;

        elves.push(elf);

        match result {
            ElfLoadResult::EndOfFile => return Ok(elves),
            ElfLoadResult::EndOfElf => (),
        }
    }
}

fn main() {
    match read_elves(&mut io::stdin().lock()) {
        Err(ElfLoadError::BadInteger) => {
            eprintln!("An invalid integer was encountered while loading the \
                       elves");
        },

        Err(ElfLoadError::IOError) => {
            eprintln!("I/O error while loading the elves");
        },

        Ok(elves) => {
            match elves.iter().map(|elf| elf.total_carrying()).max() {
                None => eprintln!("Empty list of elves"),
                Some(max_weight) => {
                    println!("Max weight {}", max_weight);
                }
            }
        }
    }
}
