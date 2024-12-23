struct Book{
    title : String,
    author : String,
    year : i32,
    is_checkedout : bool,
}

impl Book{
    fn checkout(&mut self) -> bool{
        self.is_checkedout
    }

    fn return_book(&mut self) -> bool{
        self.is_checkedout = false;
    }
    fn get_age(&mut self){
        let current_year = 2024;
        let age = current_year - self.year;
        println!("This book is {} years old", age);
        

    }

}

impl Book{
    fn new(title : String, author : String, year : i32) -> Book{
        Book{
            title,
            author,
            year,
            is_checkedout : false,
        }
}
}
impl Book {
    fn compare_book( &self, other : &Book) -> bool{
        self.year > other.year
    }

    
}