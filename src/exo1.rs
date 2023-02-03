//! ## Les Enums
//!
//! En rust, il existe plusieurs manières de représenter une donnée, tout comme en C/C++.
//! En C il y a:
//!  - struct
//!  - enum
//!  - union
//!
//! Et pour le rust c'est pareille, il y a exactement les memes type de donner.
//! Il peut donc arriver en C de devoir ecrire ce genre de chose:
//! ```
//! enum DiscordStatus {
//!     Playing,
//!     Custom,
//!     Listening,
//!     No
//! };
//! ```
//! Hors, en Rust comment on l'ecrit ?
//! ```
//! enum DiscordStatus {
//!     Playing,
//!     Custom,
//!     Listening,
//!     No
//! }
//! ```
//! Aucune difference ! Elle est belle la vie !
//! Bon complexifion un peu les choses, en C en premier.
//! ```
//! enum DiscordStatusState {
//!     Playing,
//!     Custom,
//!     Listening,
//!     No
//! };
//!
//! typedef struct {
//!     enum DiscordStatusState state;
//!     char* value
//! } DiscordStatus;
//! ```
//! ```
//! enum DiscordStatusState {
//!     Playing,
//!     Custom,
//!     Listening,
//!     No
//! }
//!
//! struct DiscordStatus {
//!     state: DiscordStatusState,
//!     value: String
//! }
//! ```
//! Dans ce cas les deux codes reste identique mais si l'on decide par exemple de prendre une string si l'état est Custom ou Listening et un objet quelconque si l'état est Playing et rien pour No.
//! Avec le C se qui va changer c'est le `char*` en `void*` mais comment fait-on pour le rust ?
//! En rust, les enums ont une particularité, ils ont le même usage que les unions et les enum de C, mais pourquoi ? Déjà il est conseillé d'utiliser les unions uniquement dans le cadre des [FFI](https://doc.rust-lang.org/nightly/std/ffi/index.html).
//! Les enums ont la possibilité d'avoir des valeurs specifiques selon l'état, avec l'example d'au dessus ca donne :
//! ```
//! struct Quelconque;
//!
//! enum DiscordStatus {
//!     Playing(Quelconque),
//!     Custom(String),
//!     Listening(String), // exemple lien vers spotify
//!     No
//! }
//! ```
//! Une quesion qui vous vient surement c'est pourquoi ça ? La réponse est Rust est un language strictement typé en plus de vouloir permettre une safety au niveau de la mémoire.
//!
//! Ok pour comprendre un peu plus en detail, allez dans le fichier `enum.rs`

enum Types {}

impl Types {
    // fn parse_once(s: String) -> Types {}

    // fn parse_many(s: String) -> Vec<Types> {}
}

/// La fonction main est mises a disposition pour faire des testes de fonctionnement avec un `cargo run`
/// le but est de reussir a faire fonctionner tous les testes avec un `cargo test`
pub fn main() {}

#[test]
fn type_parsing_once() {
    let parsed = Types::parse_once("123".to_string());
    assert_eq!(parsed, Types::Number(123i32));

    let parsed = Types::parse_once("a123".to_string());
    assert_eq!(parsed, Types::Word("a123"));

    let parsed = Types::parse_once("123a".to_string());
    assert_eq!(parsed, Types::Word("123a"));

    let parsed = Types::parse_once("azerty".to_string());
    assert_eq!(parsed, Types::Word("azerty".to_string()));

    let parsed = Types::parse_once("azerty 3".to_string());
    assert_eq!(parsed, Types::Word("azerty 3".to_string()));
}

#[test]
fn type_parsing_many() {
    let parsed = Types::parse_many("123 456".to_string());
    assert_eq!(parsed, vec![Types::Number(123), Types::Number(456)]);

    let parsed = Types::parse_many("azerty 456".to_string());
    assert_eq!(
        parsed,
        vec![Types::Word("azerty".to_string()), Types::Number(456)]
    );

    let parsed = Types::parse_many("123 azerty".to_string());
    assert_eq!(
        parsed,
        vec![Types::Number(123), Types::Word("azerty".to_string())]
    );

    let parsed = Types::parse_many("azerty azerty2".to_string());
    assert_eq!(
        parsed,
        vec![
            Types::Word("azerty".to_string()),
            Types::Word("azerty2".to_string())
        ]
    );

    let parsed = Types::parse_many("a b c".to_string());
    assert_eq!(
        parsed,
        vec![
            Types::Word("a".to_string()),
            Types::Word("b".to_string())
            Types::Word("c".to_string())
        ]
    );

    // OPTIONEL !
    // let parsed = Types::parse_many("a   b  c".to_string());
    // assert_eq!(
    //     parsed,
    //     vec![
    //         Types::Word("a".to_string()),
    //         Types::Word("b".to_string())
    //         Types::Word("c".to_string())
    //     ]
    // );

    // let parsed = Types::parse_many("'a ' b c".to_string());
    // assert_eq!(
    //     parsed,
    //     vec![
    //         Types::Word("a ".to_string()),
    //         Types::Word("b".to_string())
    //         Types::Word("c".to_string())
    //     ]
    // );
}
