//! ## Refs, Pointers et Smarts Pointers
//!
//! Deja, une reference et un pointeur sont exactement la meme chose, c'est une type de donnée qui est pour l'ordinateur un nombre qui dire "la vrai valeur est dans cette emplacement memoire".
//! Ceci dit les pointeurs et les refs du point de vue du compilateur ne sont pas les mêmes chose, un pointeur est dit "unsafe", car sont usage n'est pas protégé par rust, il est donc tres facile d'optenir des erreurs du style "segmentation fault" ce qui n'est pas possible avec les refs.
//! Ici nous n'allons pas plus parler des pointeurs car c'est un sujet qui demande une meilleur connaissance du fonctionnement du compilateur Rust, mais pour apprendre plus en profondeur le rust et aller plus long je conseille de travailler avec [rust en mode no_std](https://docs.rust-embedded.org/book/intro/no-std.html).
//!
//! Les refs fonctionnent de la meme maniere en C++ et en Rust. Les grandes differances entre les deux langages sur l'usage de ceci.
//! Le principe de ownership et la base meme du fonctionnement de Rust.
//! Pour comprendre peut-être plus simplement le systeme de ownership avec des smarts pointer et du code C++, je recommande cette video (<https://youtu.be/7EcNkr6KFy0?t=35>).
//! Pour rapidement essayer d'expliquer comment cela fonctionne, une owner variable est une variable qui va contenir la vrai valeur. Par exemple
//!
//! ```
//! let string1 = String::from("Hello");
//! let string2 = string1;
//! println!("{}", string1): // error[E0382]: borrow of moved value: `string1`
//! ```
//!
//! Rust a dans l'ordre, creer une variable string1 et y a strocker "Hello".
//! Puis, il a cree une nouvelle variable string2 et y a copier la valeur de string1 **ET** a vidé string1.
//! C'est ce que l'on appelle move une valeur.
//! Si je veux corriger mon programme alors je vais faire :
//!
//! ```
//! let string1 = String::from("Hello");
//! let string2 = &string1;
//! println!("{}", string1): // ok
//! ```
//!
//! Le & va créé une reference vers string1, string2 va être capable de travailer uniquement avec par lecture (&mut pour pouvoir modifier l'objet de base. A noté que rust autorise un cast `&mut T => &T` mais pas l'inverse).
//!
//! ### Les smarts pointers
//!
//! En C++ il y a unique_ptr, shared_ptr. Pour le rust avec l'ajout du principe de mutabilité, il existe plusieur type de smart pointer qui en C++ existe en seule exemplaire.
//! - unique_ptr à pour équivalent en rust [Box](https://doc.rust-lang.org/std/boxed/struct.Box.html) (avec des différances notable niveau implementation).
//! - shared_ptr à pour équivalent en rust [Rc](https://doc.rust-lang.org/std/rc/struct.Rc.html) (pour Ref counter).
//!
//! Dans les 2 cas, la valeur stocké n'est plus capable d'être modifier, ce qui est problematique si l'on souhaite modifier la valeur à l'interieure, alors rust met à disposition [RefCell](https://doc.rust-lang.org/core/cell/struct.RefCell.html) (ou Cell mais utilisable dans des cas trop precis pour en parler ici).
//! RefCell est un équivalant à Box mais avec la capacité de recupérer une réf mutable.
//!
//! RefCell a 2 methods important que sont `borrow` et `borrow_mut`, je vous laisse voir leurs utiliter dans la documentation.
//!
//! /!\ Ici nous ne parlerons pas des smarts pointers thread safe car tout ce qui a été dit pour le moment n'est pas utilisable dans plusieur thread.
//!
//! Dans cette exo il va falloir faire une link list dans le but de travailler les smarts pointers.

struct Node {
    value: i32,
    // next: Option<...>
}

struct List {
    // head: Option<...>,
    // tail: Option<...>,
    len: usize,
}

impl List {
    // NOTE: il est possible de créer une méthode avec le même nom qu'un attribut QUE si l'attribut et privé
    fn len(&self) -> usize {
        self.len
    }
}

pub fn main() {}

#[test]
fn link_list_empty() {
    let list = List::new();

    assert_eq!(list.len(), 0);
}

#[test]
fn link_list_push() {
    let mut list = List::new();
    list.push(1);

    assert_eq!(list.len(), 1);
    assert_eq!(list.next(), Some(1));
    assert_eq!(list.len(), 0);
    assert_eq!(list.next(), None);
}

#[test]
fn link_list_loop() {
    let mut results = vec![1, 2, 3, 4].into_iter();
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);

    assert_eq!(list.len(), 4);

    while let Some(n) = list.next() {
        assert_eq!(results.next(), Some(n));
    }
}

// si pour faire next tu as utilise le trait Iterator, bravo, ce test va passer sans que tu n'es rien a faire.
// pour rust si tu implementes Iterator, le boucle for est un sucre syntaxique a la boucle while du test d'avant
#[test]
fn link_list_loop_for() {
    let mut results = vec![1, 2, 3, 4].into_iter();
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);

    assert_eq!(list.len(), 4);

    for n in list {
        assert_eq!(results.next(), Some(n));
    }
}
