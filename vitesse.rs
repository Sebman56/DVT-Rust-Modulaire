
// *************************************************************************************
use std::io;

pub fn Fonction_Vitesse ()
{
    println!("\n*** Fonction_Vitesse ***\n");   

    println! ("\nEntrez une distance:");
    let mut DistanceSaisie =String::new();
    io::stdin()
          .read_line(&mut DistanceSaisie)
          .expect("Échec de la lecture de l'entrée utilisateur");
     
     let DistanceSaisie: f32  = DistanceSaisie.trim().parse().expect("Veuillez entrer un nombre !");
     println!("\nVotre distance: {}", DistanceSaisie);
 
    
    println! ("\nEntrez une durée:");
    let mut DuréeSaisie = String::new();
    io::stdin()
          .read_line(&mut DuréeSaisie)
          .expect("Échec de la lecture de l'entrée utilisateur");
     
    let DuréeSaisie: f32 = DuréeSaisie.trim().parse().expect("Veuillez entrer un nombre !");
    println!("\nVotre durée: {}", DuréeSaisie); 
    
    let vitesse = (DistanceSaisie / DuréeSaisie) ;
    println!("\n La vitesse est de {}", vitesse);
}
    
