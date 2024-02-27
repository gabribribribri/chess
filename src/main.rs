mod plateau;
mod other;
use {plateau::*, other::*};

fn main()
{
    let mut pg = struct_Plateau::new_set();

    

    loop
    {
        println!("\n"); //sauter deux lignes
        pg.print_pg();
        println!();

        println!("Entrez la case du pion que vous voulez bouger : ");
        let coo_of_pion = get_usable_user_input();
        
        let pion_a_dep : usize = match pg.wih(coo_of_pion)
        {
            Some(n) => {
                if pg.Armee[n].is_white == pg.Turn
                {
                    n
                }
                else
                {
                    println!("\nCe pion ne t'appartient pas.");
                    continue;
                }
            },
            None => {println!("\nCette case est vide.");continue;}
        };

    
    }
}