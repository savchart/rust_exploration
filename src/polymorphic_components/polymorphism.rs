// static dispatch
trait WalkableStatic {
    fn walk(&self);
}

struct CatStatic;

struct DogStatic;

impl WalkableStatic for CatStatic {
    fn walk(&self) {
        println!("Static cat walking");
    }
}

impl WalkableStatic for DogStatic {
    fn walk(&self) {
        println!("Static dog walking");
    }
}

fn generic_static<T: WalkableStatic>(animal: T) {
    animal.walk();
}

pub fn static_dispatch() {
    let cat = CatStatic;
    let dog = DogStatic;

    generic_static(cat);
    generic_static(dog);
}

// dynamic dispatch
trait WalkableDynamic {
    fn walk(&self);
}

struct CatDynamic;

struct DogDynamic;

impl WalkableDynamic for CatDynamic {
    fn walk(&self) {
        println!("Dynamic cat walking");
    }
}

impl WalkableDynamic for DogDynamic {
    fn walk(&self) {
        println!("Dynamic dog walking");
    }
}

fn generic_dynamic(animal: &dyn WalkableDynamic) {
    animal.walk();
}

pub fn dynamic_dispatch() {
    let cat = CatDynamic;
    let dog = DogDynamic;

    generic_dynamic(&cat);
    generic_dynamic(&dog);
}

// enum matching
enum Animal {
    Cat,
    Dog,
}

impl Animal {
    fn walk(&self) {
        match self {
            Animal::Cat => println!("Enum cat walking"),
            Animal::Dog => println!("Enum dog walking"),
        }
    }
}

pub fn enum_dispatch() {
    let cat = Animal::Cat;
    let dog = Animal::Dog;

    cat.walk();
    dog.walk();
}


// tests
#[cfg(test)]
mod polymorphism_tests {
    use super::*;

    #[test]
    fn test_static_dispatch() {
        static_dispatch();
    }

    #[test]
    fn test_dynamic_dispatch() {
        dynamic_dispatch();
    }

    #[test]
    fn test_enum_dispatch() {
        enum_dispatch();
    }
}
