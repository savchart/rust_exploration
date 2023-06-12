struct SomeData {
    name: String,
    data: u32,
}

fn bumpalo_arena() {
    let mut arena = bumpalo::Bump::new();
    for _ in 0..10 {
        let data1: &mut SomeData = arena.alloc(SomeData {
            name: String::from("data1"),
            data: 1,
        });
        let data2: &mut SomeData = arena.alloc(SomeData {
            name: String::from("data2"),
            data: 2,
        });
        arena.reset();
    }
}

fn typedarena_example() {
    for _ in 0..10 {
        let arena: typed_arena::Arena<SomeData> = typed_arena::Arena::new();
        let data1: &mut SomeData = arena.alloc(SomeData {
            name: String::from("data1"),
            data: 1,
        });
        let data2: &mut SomeData = arena.alloc(SomeData {
            name: String::from("data2"),
            data: 2,
        });
    }
}

#[cfg(test)]
mod arena_tests {
    use super::*;

    #[test]
    fn test_bumpalo_arena() {
        bumpalo_arena();
    }

    #[test]
    fn test_typedarena_example() {
        typedarena_example();
    }
}