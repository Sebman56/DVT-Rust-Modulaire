// *************************************************************************************
use std::io;

pub fn Fonction_Durée ()
{
        println!("\n*** Fonction_Durée ***\n");

        println! ("\nEntrez une distance:");
        let mut DistanceSaisie =String::new();
        io::stdin()
              .read_line(&mut DistanceSaisie)
              .expect("Échec de la lecture de l'entrée utilisateur");
         
         let DistanceSaisie: f32  = DistanceSaisie.trim().parse().expect("Veuillez entrer un nombre !");
         println!("\nVotre distance: {}", DistanceSaisie);


         println! ("\nEntrez une vitesse:");
         let mut VitesseSaisie = String::new();
         io::stdin()
               .read_line(&mut VitesseSaisie)
               .expect("Échec de la lecture de l'entrée utilisateur");
          
         let VitesseSaisie: f32 = VitesseSaisie.trim().parse().expect("Veuillez entrer un nombre !");
         println!("\nVotre vitesse: {}", VitesseSaisie); 
        let Durée = DistanceSaisie / VitesseSaisie;
        println!("\n\t\t La durée est de {}",Durée);
}

