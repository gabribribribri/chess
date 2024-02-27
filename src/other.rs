pub fn get_usable_user_input() -> (usize, usize)
{
    loop
    {
        let x_us : usize;
        let y_us : usize;
        let mut da_s : String = String::new();

        match std::io::stdin().read_line(&mut da_s) {
            Ok (_)  => (),
            Err(_)  => {println!("Il y a eu une erreur, veuillez réessayer"); continue;}
        }

        let da_chars = da_s.trim().chars();

        if da_chars.clone().count() != 2
        {
            println!("Vous devez entrer 2 caractères");
            continue;
        }

        let da_two_chars : Vec<char> = da_chars.collect();

        x_us = match da_two_chars[0]
        {
            'a' => 0, 'A' => 0,
            'b' => 1, 'B' => 1,
            'c' => 2, 'C' => 2,
            'd' => 3, 'D' => 3,
            'e' => 4, 'E' => 4,
            'f' => 5, 'F' => 5,
            'g' => 6, 'G' => 6,
            'h' => 7, 'H' => 7,
            _   => {println!("Coordonnées non valides"); continue}
        };

        y_us = match da_two_chars[1]
        {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _   => {println!("Coordonnées non valides"); continue}
        };

        return (x_us, y_us);
    }
}