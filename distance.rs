// *************************************************************************************
use std::io;

pub fn Fonction_Distance ()
{
//    distance = Durée * vitesse;
    println!("\n*** Fonction_Distance ***\n");
    println! ("\nEntrez une durée:");
    let mut DuréeSaisie = String::new();
    io::stdin()
          .read_line(&mut DuréeSaisie)
          .expect("Échec de la lecture de l'entrée utilisateur");
    let DuréeSaisie: f32 = DuréeSaisie.trim().parse().expect("Veuillez entrer un nombre !");
    println!("\nVotre durée: {}", DuréeSaisie); 

    println! ("\nEntrez une vitesse:");
    let mut VitesseSaisie = String::new();
    io::stdin()
          .read_line(&mut VitesseSaisie)
          .expect("Échec de la lecture de l'entrée utilisateur");
    let VitesseSaisie: f32 = VitesseSaisie.trim().parse().expect("Veuillez entrer un nombre !");
    println!("\nVotre vitesse: {}", VitesseSaisie); 

let distance =VitesseSaisie*DuréeSaisie;
println!("\n\t\t La distance est de {}", distance);
}
