use std::fs::File;
use std::io::Write;
// we use the same types as the rest of the library.
use rand::{Rng, weak_rng, SeedableRng};
use color::NUM_COLORS;
use piece::NUM_PIECES;
use square::NUM_SQUARES;
use castle_rights::NUM_CASTLE_RIGHTS;
use file::NUM_FILES;

// write the ZOBRIEST_* arrays to a file.  I don't generate it, because its just
// a bunch of random u64s
pub fn write_zobrist(f: &mut File) {
    let mut rng = weak_rng();
    rng.reseed([0xDEADBEEF, 0xBEEFDEAD, 0xABCDEFAB, 0x12345678]);

    write!(f, "const SIDE_TO_MOVE: u64 = {};\n\n", rng.next_u64()).unwrap();

    write!(f, "const ZOBRIST_PIECES: [[[u64; NUM_SQUARES]; NUM_PIECES]; NUM_COLORS] = [[[\n").unwrap();
    for i in 0..NUM_COLORS {
        for j in 0..NUM_PIECES {
            for _ in 0..NUM_SQUARES {
                write!(f, "    {},\n", rng.next_u64()).unwrap();
            }
            if j != NUM_PIECES - 1 {
                write!(f, "   ], [\n").unwrap();
            }
        }
        if i != NUM_COLORS - 1 {
            write!(f, "  ]], [[\n").unwrap();
        }
    }
    write!(f, "]]];\n\n").unwrap();

    write!(f, "const ZOBRIST_CASTLES: [[u64; NUM_CASTLE_RIGHTS]; NUM_COLORS] = [[\n").unwrap();
    for i in 0..NUM_COLORS {
        for _ in 0..NUM_CASTLE_RIGHTS {
            write!(f, "    {},\n", rng.next_u64()).unwrap();
        }
        if i != NUM_COLORS - 1 {
            write!(f, "  ], [\n").unwrap();
        }
    }
    write!(f, "]];\n\n").unwrap();

    write!(f, "const ZOBRIST_EP: [[u64; NUM_FILES]; NUM_COLORS] = [[\n").unwrap();
    for i in 0..NUM_COLORS {
        for _ in 0..NUM_FILES {
            write!(f, "    {},\n", rng.next_u64()).unwrap();
        }
        if i != NUM_COLORS - 1 {
            write!(f, "], [\n").unwrap();
        }
    }
    write!(f, "]];\n\n").unwrap();
}
