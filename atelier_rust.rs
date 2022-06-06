// Contenu : générer nombre aléatoire, comparaison de variables (cmp), utilisation match, cast variable str en int, inpu/output, loop (boucle)


use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let nombre_secret = rand::thread_rng().gen_range(1..101);

    loop {

    println!("Choisis un nombre entre 1 et 101 !");

    let mut supposition = String::new();

    io::stdin()
        .read_line(&mut supposition)
        .expect("Échec de la lecture de l'entrée utilisateur");

    // version avec except qui fait crasher : let supposition: u32 = supposition.trim().parse().expect("Veuillez entrer un nombre !");
    // version avec un catch qui permet de continuer la boucle :

    let supposition: u32 = match supposition.trim().parse() {
        Ok(nombre) => nombre,
        Err(_) => {
            println!("Un nombre je t'ai dis !");
            continue;}
    };

    println!("Votre nombre : {}", supposition);

    match supposition.cmp(&nombre_secret) {
        Ordering::Less => println!("C'est plus !"),
        Ordering::Greater => println!("C'est moins !"),
        Ordering::Equal => {
            println!("Tu as gagné !");
            break;
        }
    }
    }
}


// Contenu : comportement de modification d'une variable, hors et dans une portée intérieure

fn main() {
    let x = 5;

    println!("La valeur de x est : {}", x);           // value : 5

    let x = x + 1;

    println!("La valeur de x est : {}", x);        // value : 6

    {
        let x = x * 2;
        println!("La valeur de x dans la portée interne est : {}", x);           // value : 12
    }

    println!("La valeur de x est : {}", x);      // value : 6
    
}


// Contenu : len d'une str

fn main() {
    let espaces = "   ";
    let espaces = espaces.len();
    println!("{}", espaces);
}


// Contenu : variable flottant

fn main() {
    let x = 2.2; // f64
    println!("{}", x);
    let y: f32 = 3.1; // f32
    println!("{}", y);
}


// Contenu : addition, soustraction, multiplication : toujours créer variable let 

fn main() {
    // addition
    let somme = 5 + 10;

    // soustraction
    let difference = 95.5 - 4.3;

    // multiplication
    let produit = 4 * 30;



// Contenu : boolean : en minuscule

fn main() {
    let t = true;

    let f: bool = false; // avec une annotation de type explicite
}


// Contenu : Tuples 

fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);             // déclaration des types dans le tuple puis déclaration des valeurs
}

//

// Pour décomposer un tuple, via destructuration ;

fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("La valeur de y est : {}", y);
}

//

// Pour décomposer un tuple, via recherche intra ;

fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let cinq_cents = x.0;

    let six_virgule_quatre = x.1;

    let un = x.2;
}

// Le tuple sans aucune valeur, (), est un type spécial qui a une seule et unique valeur, qui s'écrit aussi ()

// Contenu : Tableau ou Array

// Contrairement aux tuples, chaque élément d'un tableau doit être du même type. 
// Contrairement aux tableaux de certains autres langages, les tableaux de Rust ont une taille fixe.
// les tableaux s'avèrent utiles lorsque vous savez que le nombre d'éléments n'aura pas besoin de changer.

//ex : 

let mois = ["Janvier", "Février", "Mars", "Avril", "Mai", "Juin", "Juillet",
            "Août", "Septembre", "Octobre", "Novembre", "Décembre"];

------------------------

//Pour "cadrer" la déclaration d'un tableau :

let a: [i32; 5] = [1, 2, 3, 4, 5];                 // dans ce tableau il y aura 5 valeur d'entier signées 32

------------------------

//Pour déclarer un tableau au contenu identique mais redondant :

let a = [3; 5];               // il y aura 5 éléments de valeur 3, ce code égale donc let a = [3, 3, 3, 3, 3];

------------------------

//Pour explorer un tableau :        // équivalent à l'exploration de tuples en python

fn main() {
    let a = [1, 2, 3, 4, 5];

    let premier = a[0];
    let second = a[1];
}


// Contenu : déclaration de fonction qui appelle une autre fonction,

// Il faut toujours déclarer les types des arguments de fonction lors de création d'une fonction

fn main() {
    une_autre_fonction(5);
}

fn une_autre_fonction(x: i32) {
    println!("La valeur de x est : {}", x);
}

-----------------------------------------------------------------------------------

// Contenu : Instruction / Expression de fonction, différences 

// une fonction main qui contient une instruction : 

fn main() {
    let y = 6;
}

// Les expressions peuvent faire partie d'une instruction : 
// dans la fonction ci-dessus, le 6 dans l'instruction let y = 6; est une expression qui s'évalue à la valeur 6.

//

// L'appel de fonction est aussi une expression. 
// L'appel de macro est également une expression.
// Un nouveau bloc de portée que nous créons avec des accolades est une expression, ex de tout cela ;

fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("La valeur de y est : {}", y);
}

main()

//

//Définir le retour de fonction dès la création de la fonction 

fn cinq() -> i32 {
    5                                  // indiquer via -> que la fonction renvoie un entier
}

fn main() {
    let x = cinq();

    println!("La valeur de x est : {}", x);
}

////////////////////////////////////////////////////////////////////////////

Contenu : condition if sur comparaison de variable

fn main() {
    let nombre = 6;

    if nombre < 5 {
        println!("La condition est vérifiée");
    } else {
        println!("La condition n'est pas vérifiée");
    }
}

//

Contenu : exemple condition if true booléen

fn main() {
    let condition = true;

    if condition {
        println!("La condition est {}", condition);
    }
}

//

Contenu : else if

fn main() {
    let nombre = 6;

    if nombre % 4 == 0 {
        println!("Le nombre est divisible par 4");
    } else if nombre % 3 == 0 {
        println!("Le nombre est divisible par 3");
    } else if nombre % 2 == 0 {
        println!("Le nombre est divisible par 2");
    } else {
        println!("Le nombre n'est pas divisible par 4, 3 ou 2");
    }
}

//

Contenu : variable-fonction-condition-if

fn main() {

    let condition = true;
    
    let nombre = if condition { 5 } else { 6 };

    println!("La valeur du nombre est : {}", nombre);
    
}

//////////////////////////////////////////////////////////////////

Contenu : les boucles loop, while, for

// loop

fn main() {
    loop {
        println!("À nouveau !");
    }
}

// compteur simple avec loop

fn main() {
    let mut compteur = 0;

    let resultat = loop {

        compteur += 1;
        println!("état du compteur : {}", compteur);

        if compteur == 10 {
            break compteur * 2;
        }
    };

    println!("Le résultat est {}", resultat);
}


// boucles imbriquées : compteur initial + compteur secondaire : incrémentation + loop

fn main() {
    let mut compteur = 0;
    'increment: loop {
        println!("boucle 1, compteur stock = {}", compteur);
        let mut restant = 10;

        loop {
            println!("dans la 2ème boucle, compteur restant = {}", restant);
            if restant == 5 {
                println!("sortie 2ème boucle sur compteur boucle 2");
                break; // break de la loop 
            }
            if compteur == 2 {
                println!("sortie 2ème boucle sur compteur boucle 1");
                break 'increment; // break de l'incrémentation
            }

            restant -= 1; // else final implicite
        }

        println!("fin boucle 2, compteur fin boucle 1 = {}", compteur);

        compteur += 1;

        println!("compteur fin boucle 1 incrémenté = {}", compteur);

    }
    println!("Fin du compteur 1 = {}", compteur);
}

// boucle while avec compteur

fn main() {
    let mut nombre = 3;

    while nombre != 0 {
        println!("{} !", nombre);

        nombre -= 1;
    }

    println!("DÉCOLLAGE !!!");
}

// boucle while itération sur indice de liste-tableau-array

fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut indice = 0;

    while indice < 5 {
        println!("La valeur est : {}", a[indice]);
        indice += 1;
        println!("L'indice est {}", indice);
    }
}

// boucle for itération sur liste

fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("La valeur est : {}", element);
    }
}

// boucle for qui itère à l'envers, (ici de 3 à 1), utilisation de REV

fn main() {
    for nombre in (1..4).rev() {
        println!("{} !", nombre);
    }
    println!("DÉCOLLAGE !!!");
}

// Les fonctions

// exemple fonction qui prend string en argument et renvoie un string


fn prend_et_rend(texte: String) -> String {
  texte
}

