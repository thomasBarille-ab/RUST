//Prendre possesion 
fn main() {
    let s = String::from("hello");  // s rentre dans la portée.
  
    prendre_possession(s);  // La valeur de s est déplacée dans la fonction…
                            // … et n'est plus en vigueur à partir d'ici
  
    let x = 5;              // x rentre dans la portée.
  
    creer_copie(x);         // x va être déplacée dans la fonction,
                            // mais i32 est Copy, donc on peut
                            // utiliser x ensuite.
  
  } // Ici, x sort de la portée, puis ensuite s. Mais puisque la valeur de s a
  // été déplacée, il ne se passe rien de spécial.
  
fn prendre_possession(texte: String) { // texte rentre dans la portée.
    println!("{}", texte);
} // Ici, texte sort de la portée et `drop` est appelé. La mémoire est libérée.

fn creer_copie(entier: i32) { // entier rentre dans la portée.
    println!("{}", entier);
} // Ici, entier sort de la portée. Il ne se passe rien de spécial.

//Donner possession
fn main() {
    let s1 = donne_possession();     // donne_possession déplace sa valeur de
                                        // retour dans s1

    let s2 = String::from("hello");  // s2 rentre dans la portée

    let s3 = prend_et_rend(s2);      // s2 est déplacée dans
                                        // prend_et_rend, qui elle aussi
                                        // déplace sa valeur de retour dans s3.
} // Ici, s3 sort de la portée et est éliminée. s2 a été déplacée donc il ne se
// passe rien. s1 sort aussi de la portée et est éliminée.

fn donne_possession() -> String {      // donne_possession va déplacer sa
                                        // valeur de retour dans la
                                        // fonction qui l'appelle.

    let texte = String::from("yours");   // texte rentre dans la portée.

    texte                                // texte est retournée et
                                            // est déplacée vers le code qui
                                            // l'appelle.
}

// Cette fonction va prendre une String et en retourne aussi une.
fn prend_et_rend(texte: String) -> String { // texte rentre dans la portée.

    texte  // texte est retournée et déplacée vers le code qui l'appelle.
}
  
  