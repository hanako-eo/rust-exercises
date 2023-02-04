pub struct Cell {}

/// La fonction main est mises a disposition pour faire des testes de fonctionnement avec un `cargo run`
/// le but est de reussir a faire fonctionner tous les testes avec un `cargo test`
pub fn main() {}

#[test]
fn set_during_get() {
    let x = Cell::<&str>::new("hello");
    let first = x.get();

    x.set("world");

    assert_eq!(first, "hello");
    assert_ne!(x.get(), first);
}

#[test]
fn update_cell() {
    let x = Cell::new("hello");
    let first = x.get();

    x.update(|s| format!("{} 1", s).as_str());

    assert_eq!(first, "hello");
    assert_ne!(x.get(), "hello 1");
}

#[test]
fn swap_value_between_cells() {
    let x = Cell::new("hello");
    let y = Cell::new("world");

    x.swap(y);

    assert_eq!(x.get(), "world");
    assert_eq!(y.get(), "hello");
}

#[test]
fn unwrap_cell() {
    let a = Cell::new("hello");
    let b = a.into_inner();
    let c: &str = a.into();

    assert_eq!(a.get(), b);
    assert_eq!(b, c);

    let x = Cell::new(2);
    let y = x.into_inner();
    let z: i32 = x.into();

    assert_eq!(x.get(), y);
    assert_eq!(y, z);
}

#[test]
fn transmute_cell() {
    let a = Cell::new(1);
    let b = a.get();
    let c = unsafe { core::mem::transmute::<Cell<i32>, i32>(a) };

    assert_eq!(b, c);
}

// ----------------------------------------------------------------

// Niveau 1: avec UnsafeCell
// Niveau 2: usage des raw pointers (pas de UnsafeCell du core)
// si jamais pour les raw pointers https://doc.rust-lang.org/std/primitive.pointer.html
