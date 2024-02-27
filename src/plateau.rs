pub enum enum_Piece
{
    Pion,
    Cavalier,
    Fou,
    Tour,
    Dame,
    Roi
}

use enum_Piece::*;

pub struct struct_Jeton
{
    pub Piece    : enum_Piece,
    pub is_white : bool,
}

pub struct struct_Plateau
{
    pub Position : [[Option<usize>; 8]; 8],
    pub Armee    : [struct_Jeton; 32],
    pub Turn     : bool,
}

impl struct_Plateau
{
    pub fn new_set() -> Self
    {
        struct_Plateau
        {
            Position : 
            [
                [Some(24), Some(26), Some(28), Some(31), Some(30), Some(29), Some(27), Some(25 )],
                [Some(16), Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), Some(23)],
                [None    , None    , None    , None    , None    , None    , None    , None    ],
                [None    , None    , None    , None    , None    , None    , None    , None    ],
                [None    , None    , None    , None    , None    , None    , None    , None    ],
                [None    , None    , None    , None    , None    , None    , None    , None    ],
                [Some(0 ), Some(1 ), Some(2 ), Some(3 ), Some(4 ), Some(5 ), Some(6 ), Some(7 )],
                [Some(8 ), Some(10), Some(12), Some(15), Some(14), Some(13), Some(11), Some(9 )],
            ],
            
            Armee : 
            [
                struct_Jeton{Piece : Pion, is_white     : true }, //0
                struct_Jeton{Piece : Pion, is_white     : true },
                struct_Jeton{Piece : Pion, is_white     : true },
                struct_Jeton{Piece : Pion, is_white     : true },
                struct_Jeton{Piece : Pion, is_white     : true },
                struct_Jeton{Piece : Pion, is_white     : true }, //5
                struct_Jeton{Piece : Pion, is_white     : true },
                struct_Jeton{Piece : Pion, is_white     : true },

                struct_Jeton{Piece : Tour, is_white     : true },
                struct_Jeton{Piece : Tour, is_white     : true },
                struct_Jeton{Piece : Cavalier, is_white : true},
                struct_Jeton{Piece : Cavalier, is_white : true},
                struct_Jeton{Piece : Fou , is_white     : true }, //10
                struct_Jeton{Piece : Fou , is_white     : true },
                struct_Jeton{Piece : Dame, is_white     : true},
                struct_Jeton{Piece : Roi , is_white     : true}, //15 -!!!- Roi Blanc -!!!-


                struct_Jeton{Piece : Pion, is_white     : false},
                struct_Jeton{Piece : Pion, is_white     : false},
                struct_Jeton{Piece : Pion, is_white     : false},
                struct_Jeton{Piece : Pion, is_white     : false},
                struct_Jeton{Piece : Pion, is_white     : false}, //20
                struct_Jeton{Piece : Pion, is_white     : false},
                struct_Jeton{Piece : Pion, is_white     : false},

                struct_Jeton{Piece : Pion, is_white     : false},
                struct_Jeton{Piece : Tour, is_white     : false},
                struct_Jeton{Piece : Tour, is_white     : false}, //25
                struct_Jeton{Piece : Cavalier, is_white : false},
                struct_Jeton{Piece : Cavalier, is_white : false},
                struct_Jeton{Piece : Fou , is_white     : false},
                struct_Jeton{Piece : Fou , is_white     : false},
                struct_Jeton{Piece : Dame, is_white     : false}, //30
                struct_Jeton{Piece : Roi , is_white     : false}, //31 -!!!- Roi Noir -!!!-
            ],

            Turn : true
        }
    }

    pub fn wih(&self, tu : (usize, usize)) -> Option<usize> {self.Position[tu.1][tu.0].clone()}

    pub fn print_pg(&self)
    {

        println!("  |  A  |  B  |  C  |  D  |  E  |  F  |  G  |  H  |");

        let mut point : u8 = 48;

        for i in self.Position
        {
            
            println!("--+-----+-----+-----+-----+-----+-----+-----+-----+");

            let mut temp_print_str : String = String::new();

                point += 1;
                temp_print_str.push(point as char);
                temp_print_str.push(' ');
                temp_print_str.push('|');

            
            for j in i
            {

                match j
                {
                    Some(n) => {
                        if   self.Armee[n].is_white {temp_print_str.push(' '); temp_print_str.push(' ')}
                        else                        {temp_print_str.push(' '); temp_print_str.push('#')}

                        match self.Armee[n].Piece
                        {
                            Pion     => temp_print_str.push('P'),
                            Fou      => temp_print_str.push('F'),
                            Cavalier => temp_print_str.push('C'),
                            Tour     => temp_print_str.push('T'),
                            Dame     => temp_print_str.push('D'),
                            Roi      => temp_print_str.push('R'),
                        }

                        if   self.Armee[n].is_white {temp_print_str.push(' '); temp_print_str.push(' ')}
                        else                        {temp_print_str.push('#'); temp_print_str.push(' ')}
                    }

                    None => {
                        temp_print_str.push(' ');
                        temp_print_str.push(' ');
                        temp_print_str.push(' ');
                        temp_print_str.push(' ');
                        temp_print_str.push(' ');
                    }
                }

                temp_print_str.push('|');
            }
            println!("{}", temp_print_str);
        }

        println!("--+-----+-----+-----+-----+-----+-----+-----+-----+");
    }

    pub fn check_the_move(&self, to_move_coo : (usize, usize), moved_to : (usize, usize)) -> bool
    {
        true
    }
}


