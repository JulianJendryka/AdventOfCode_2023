use super::hash::HashCreator;

#[derive(Clone, Debug)]
pub struct Lense {
    pub do_insert: bool,
    pub hash: u128,
    pub label: String,
    pub focal_length: u8,
}

impl Lense {
    pub fn create(line: String) -> Lense {
        match line.contains("-") {
            true => Lense::create_removing_lense(line),
            false => Lense::create_adding_lense(line),
        }
    }

    pub fn create_removing_lense(line: String) -> Lense {
        let mut split = line.split("-");

        let label = split.next().unwrap().to_string();
        Lense {
            do_insert: false,
            hash: HashCreator::get_hash(&label),
            label,
            focal_length: 0,
        }
    }

    pub fn create_adding_lense(line: String) -> Lense {
        let mut split = line.split("=");

        let label = split.next().unwrap().to_string();
        let focal_length: u8 = split
            .next()
            .unwrap()
            .to_string()
            .trim()
            .parse()
            .expect("please give me correct focal length!");

        Lense {
            do_insert: true,
            hash: HashCreator::get_hash(&label),
            label,
            focal_length,
        }
    }

    pub fn print(&self) {
        print!(" [{0} {1}]", self.label, self.focal_length)
    }
}
