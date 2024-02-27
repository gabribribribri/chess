const PAT_FOU : [(isize, isize); 4] = [
    ( 1,  1),
    ( 1, -1),
    (-1,  1),
    (-1, -1)
];

const PAT_TOUR : [(isize, isize); 4] = [
    (0, 0),
    (0, 1),
    (1, 0),
    (1, 1),
];

const PION_MOVE_PAT : [(isize, isize); 4] = [
    (0, -1),
    (0, -2)
];

const PION_EAT_PAT  : [(isize, isize); 4] = [
    (-1, -1),
    ( 1, -1)
];

const ROI_PAT : [(isize, isize); 4] = [

];