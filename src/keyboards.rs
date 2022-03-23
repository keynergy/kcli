use keynergy::fingers::*;
use keynergy::Keyboard;

pub fn ansi() -> Keyboard {
    Keyboard {
        name: "ANSI".to_string(),
        rowstagger: vec![0.0, 0.25, 0.75],
        colstagger: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        dimensions: [11, 3],
        keyheight: 1.0,
        fingers: vec![
            vec![LP, LR, LM, LI, LI, RI, RI, RM, RR, RP, RP],
            vec![LP, LR, LM, LI, LI, RI, RI, RM, RR, RP, RP],
            vec![LP, LR, LM, LI, LI, RI, RI, RM, RR, RP, RP],
        ],
    }
}

pub fn matrix() -> Keyboard {
    Keyboard {
        name: "Matrix".to_string(),
        rowstagger: vec![0.0, 0.0, 0.0],
        colstagger: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        dimensions: [11, 3],
        keyheight: 1.0,
        fingers: vec![
            vec![LP, LR, LM, LI, LI, RI, RI, RM, RR, RP, RP],
            vec![LP, LR, LM, LI, LI, RI, RI, RM, RR, RP, RP],
            vec![LP, LR, LM, LI, LI, RI, RI, RM, RR, RP, RP],
        ],
    }
}
