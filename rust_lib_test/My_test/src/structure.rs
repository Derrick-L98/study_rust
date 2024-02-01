use std::vec;

pub struct Structure {
    vec: Vec<i32>,
}

impl Structure {
    pub async fn new() -> Self {
        Self { vec: Vec::new() }
    }
    pub async fn push(&mut self) {
        self.vec.push(1);
        self.vec.push(2);
        self.vec.push(3);
        self.vec.push(4);
        self.vec.push(5);
        println!("{:#?}", self.vec);
        self.vec.reverse();
        println!("{:#?}", self.vec);
        // self.
    }
    pub async fn show(&self) {
        println!("{:#?}", self.vec);
    }
}
