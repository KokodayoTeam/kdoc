pub trait Cloner {
    fn clone_dir();
}
pub trait FileMapper {
    fn from_json(path: &str) -> Self;
    fn do_map();
}

struct DefaultMapper {}
impl FileMapper for DefaultMapper {
    fn from_json(path: &str) -> Self {
        DefaultMapper {}
    }

    fn do_map() {
        todo!()
    }
}

struct DefaultCloner{}
impl Cloner for DefaultCloner {
    fn clone_dir() {
        todo!()
    }
}

fn main() {
    //copy dir

    //read rules of filemapper

    //traverse the dir and do map-job
}
