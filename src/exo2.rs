//! ## Les Traits
//!
//! Dans cet exercice, nous allons travailler les traits et essayer de comprendre plus en profondeur comment ça marche, le but principal étant de faire un `StringSplitter`. Mais avant de commencer à coder, expliquons au moins ce que c'est pour être sûr de comprendre de quoi on parle.
//! ```
//! trait Hello {
//!     fn hi();
//! }
//! ```
//! Le code juste au-dessus est un trait, c'est un type de donnée très particulier, il est similaire dans l'idée à une interface dans d'autre langage.
//! Un trait défini des méthodes qui vont devoir être implémentées par une structure ou un enum.
//! ```
//! struct MyHello {
//!     a: u8
//! }
//!
//! impl Hello for MyHello {
//!     fn hi() {
//!         println!("Hello")
//!     }
//! }
//! ```
//! Ici la structure implémente la méthode `hi` du trait `Hello`, il est important de savoir qu'un trait peut très bien prendre des génériques (comme le trait `From<T>`) et des lifetimes (exo pas encore fait, mais arrive très vite) en paramètre.
//! ```
//! trait HelloGenerique<T> {
//!     fn hi_generic() -> T;
//! }
//! ```
//! Il est aussi possible de voir cette syntaxe
//! ```
//! trait HelloTyped {
//!     type Item;
//!
//!     fn hi_typed() -> Self::Item;
//! }
//! ```
//! Pour comprendre plus en profondeur, il est obligatoire de connaitre le principe du [name mangling](https://fr.wikipedia.org/wiki/D%C3%A9coration_de_nom).
//! Pour des besoins de simplicité, je vais simplement raccourcir et expliquer que pour un même struct, il est possible si `T` est différant, d'implémenter autant de `HelloGenerique` que l'on souhaite, mais que pour la version avec les types (`HelloTyped` dans l'exemple) il n'est possible pour un même struct d'implémenter 2 fois `HelloTyped` même si `Item` change.
//! Pour nous, il va nous falloir utiliser le trait [Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html).
//!
//! A savoir qu'il est aussi possible de donnée une implementation par defaut a une methode :
//!
//! ```
//! struct MyHello {}
//!
//! trait HelloWithDefault {
//!     fn hi_default() {
//!         println!("par default");
//!     }
//! }
//!
//! impl HelloWithDefault for MyHello {
//!     // tu peux ne pas implementer hi_default, il va utiliser la methode dans le trait
//! }
//! ```
//!
//! (On est loins d'avoir aborder tout avec les traits mais pour le moment il n'est pas obligatoire dans savoir plus)

struct StringSplitter {
    remainder: Option<String>,
    delimiter: String,
}

impl StringSplitter {
    fn new(needle: String, delimiter: String) -> Self {
        Self {
            remainder: Some(needle),
            delimiter,
        }
    }
}

// Implementation de Iterator, IL EST INTERDIT d'UTIILISER LA METHODE split.
// Pour se faire il vous faut utiliser la methode find et get
// https://doc.rust-lang.org/std/string/struct.String.html#method.find
// https://doc.rust-lang.org/std/string/struct.String.html#method.get
// ici...

/// La fonction main est mises a disposition pour faire des testes de fonctionnement avec un `cargo run`
/// le but est de reussir a faire fonctionner tous les testes avec un `cargo test`
pub fn main() {}

#[test]
fn test_string_splitter() {
    let splitter = StringSplitter::new("a c d".to_string(), " ".to_string());
    let result: Vec<String> = splitter.collect(); // NOTE: collect est une fonction de "type casting" entre 2 Iterables, elle est automatiquement implementer par Iterator il est donc inutile de l'implementer soit même

    assert_eq!(
        result,
        vec!["a".to_string(), "c".to_string(), "d".to_string()]
    )
}

#[test]
fn test_string_splitter2() {
    let splitter = StringSplitter::new("a c ".to_string(), " ".to_string());
    let result: Vec<String> = splitter.collect();

    assert_eq!(
        result,
        vec!["a".to_string(), "c".to_string(), "".to_string()]
    )
}
