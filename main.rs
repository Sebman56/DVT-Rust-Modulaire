mod dure;
mod vitesse;
mod distance;


use std::io;
fn main (){
    menu();
}

fn menu() {
    println!("\n\n\t Salut Séb !!! \n");
    println!("\n\n\t Programme DVT \n");
    println!(" \t Vous cherchez :");
    println!(" Une distance --- tapez: 1");
    println!(" Une vitesse  --- tapez: 2");
    println!(" Une durée    --- tapez: 3");
    println!(" \n\n\t Choix:");

    let mut choix = String::new();

    io::stdin()
        .read_line(&mut choix)
        .expect(" Echec de l'entrée utilisateur");

    let choix: u32 = choix.trim().parse().expect("Veuillez entrer un nombre !");
    let retour_choix: &u32 = &choix;

    retour_menu(retour_choix);
}

fn retour_menu (retour_choix: &u32) {
        println!("\n\t Vous avez choisi: {}\n\n\t", retour_choix);
     
        if *retour_choix == 1 {
            println!("Vous chercher une distance");
            distance::Fonction_Distance ();    
        }
        if *retour_choix == 2 {
            println!("Vous chercher une vitesse");
            vitesse::Fonction_Vitesse ();
        }
        if *retour_choix == 3 {
            println!("Vous chercher une durée");
            dure::Fonction_Durée ();
        } 
        if *retour_choix == 0 || *retour_choix > 3  {
            println!("Je ne sais pas ce que vous cherchez !!!");
        }            
}
