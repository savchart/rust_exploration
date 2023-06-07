mod polymorphic_components{
    pub mod polymorphism;
}

use polymorphic_components::polymorphism::{static_dispatch, dynamic_dispatch, enum_dispatch};

fn main() {
    static_dispatch();
    dynamic_dispatch();
    enum_dispatch();
}
