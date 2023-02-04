// Ceci est un enum Option uniquement pour les i32, pas tres reutilisable.
enum OptionI32 {
    Some(i32),
    None,
}

// un enum peut prendre des generiques comme des lifetimes.
enum Option<Type> {
    Some(Type),
    None,
}

// il est important de savoir qu'un enum peut aussi implementer c'est propre methode comme tous les types de donnée
// cette implementation est faite plus pour montrer, elle n'a pas d'autre interer que ca
impl<Type> Option<Type> {
    // c'est trois implementation de unwrap marche de la même manière
    fn unwrap(self) -> Type {
        // le match est une expression, càd qu'il donne un resultat et que ce resultat peut être stocker directement dans une variable
        match self {
            // Note: Self == Option<Type>
            Self::Some(value) => value,
            Self::None => panic!("attempt to Some not None"),
        }
        // let value = match self {
        //     Self::Some(value) => value,
        //     Self::None => panic!("attempt to Some not None"),
        // };
        // return value
    }

    fn unwrap2(self) -> Type {
        // le if let est comme un if en rust mais il ne peut être utiliser uniquement pour une valeur à la fois,
        // surtout utiliser dans le cadre ou on souhaite ne prendre en charge qu'un seule cas
        // elle est utile pour faire sortir une valeur contenu dans membre de l'enum, c'est equivalent a une destructuration
        if let Self::Some(value) = self {
            value
        } else {
            panic!("attempt to Some not None")
        }
    }

    fn unwrap3(self) -> Type {
        // cette syntaxe est arrivé vers la 1.65 pour plus d'info sur le let else je vous conseil d'aller lire https://rust-lang.github.io/rfcs/3137-let-else.html
        // pour faire simple, let expression = resultat else { code de sortie de la function }
        // elle est utile dans les même cas que le if let mais pers de rendre le code plus claire et de retirer une indentation
        let Self::Some(value) = self else {
            panic!("attempt to Some not None")
        };
        value
    }

    // &self et &mut self permet de ne pas faire libérer un objet après la fin de l'exécution de la méthode
    fn is_some(&self) -> bool {
        matches!(self, Self::Some(_))
        // c'est une macro qui donne
        // match self {
        //     Self::Some(_) => true,
        //     _ => false,
        // }
        // le _ une variable qui va avoir toutes les autres valeurs de self, il est possible de l'appeler banane ou kangourou;
        // la particularité du _ c'est que le rust ne nous lèsera pas l'utiliser, une variable _ est une variable non utilisable
    }
}
