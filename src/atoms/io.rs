use crate::atoms::atoms::Atoms;
use na::{Matrix3xX, Vector3};
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter};
use std::path::Path;

static ATOMIC_MASSES: [(&str, f64); 118] = [
    ("H", 1.008),
    ("He", 4.002602),
    ("Li", 6.94),
    ("Be", 9.012182),
    ("B", 10.81),
    ("C", 12.011),
    ("N", 14.007),
    ("O", 15.999),
    ("F", 18.9984032),
    ("Ne", 20.1797),
    ("Na", 22.98976928),
    ("Mg", 24.305),
    ("Al", 26.9815386),
    ("Si", 28.085),
    ("P", 30.973762),
    ("S", 32.06),
    ("Cl", 35.45),
    ("Ar", 39.948),
    ("K", 39.0983),
    ("Ca", 40.078),
    ("Sc", 44.955912),
    ("Ti", 47.867),
    ("V", 50.9415),
    ("Cr", 51.9961),
    ("Mn", 54.938045),
    ("Fe", 55.845),
    ("Co", 58.933195),
    ("Ni", 58.6934),
    ("Cu", 63.546),
    ("Zn", 65.38),
    ("Ga", 69.723),
    ("Ge", 72.63),
    ("As", 74.9216),
    ("Se", 78.96),
    ("Br", 79.904),
    ("Kr", 83.798),
    ("Rb", 85.4678),
    ("Sr", 87.62),
    ("Y", 88.90585),
    ("Zr", 91.224),
    ("Nb", 92.90638),
    ("Mo", 95.96),
    ("Tc", 97.0),
    ("Ru", 101.07),
    ("Rh", 102.9055),
    ("Pd", 106.42),
    ("Ag", 107.8682),
    ("Cd", 112.411),
    ("In", 114.818),
    ("Sn", 118.71),
    ("Sb", 121.76),
    ("Te", 127.6),
    ("I", 126.90447),
    ("Xe", 131.293),
    ("Cs", 132.9054519),
    ("Ba", 137.327),
    ("La", 138.90547),
    ("Ce", 140.116),
    ("Pr", 140.90765),
    ("Nd", 144.242),
    ("Pm", 145.0),
    ("Sm", 150.36),
    ("Eu", 151.964),
    ("Gd", 157.25),
    ("Tb", 158.92535),
    ("Dy", 162.5),
    ("Ho", 164.93032),
    ("Er", 167.259),
    ("Tm", 168.93421),
    ("Yb", 173.054),
    ("Lu", 174.9668),
    ("Hf", 178.49),
    ("Ta", 180.94788),
    ("W", 183.84),
    ("Re", 186.207),
    ("Os", 190.23),
    ("Ir", 192.217),
    ("Pt", 195.084),
    ("Au", 196.966569),
    ("Hg", 200.592),
    ("Tl", 204.38),
    ("Pb", 207.2),
    ("Bi", 208.9804),
    ("Po", 209.0),
    ("At", 210.0),
    ("Rn", 222.0),
    ("Fr", 223.0),
    ("Ra", 226.0),
    ("Ac", 227.0),
    ("Th", 232.03806),
    ("Pa", 231.03588),
    ("U", 238.02891),
    ("Np", 237.0),
    ("Pu", 244.0),
    ("Am", 243.0),
    ("Cm", 247.0),
    ("Bk", 247.0),
    ("Cf", 251.0),
    ("Es", 252.0),
    ("Fm", 257.0),
    ("Md", 258.0),
    ("No", 259.0),
    ("Lr", 262.0),
    ("Rf", 267.0),
    ("Db", 270.0),
    ("Sg", 271.0),
    ("Bh", 270.0),
    ("Hs", 277.0),
    ("Mt", 276.0),
    ("Ds", 281.0),
    ("Rg", 282.0),
    ("Cn", 285.0),
    ("Uut", 285.0),
    ("Fl", 289.0),
    ("Mc", 289.0),
    ("Lv", 293.0),
    ("Ts", 294.0),
    ("Og", 294.0),
];

pub fn get_atomic_mass(name: &str) -> f64 {
    for (symbol, mass) in ATOMIC_MASSES.iter() {
        if name == *symbol {
            return *mass;
        }
    }
    return 0.0;
}

pub fn get_atomic_str(atom_mass: f64) -> String {
    for (symbol, mass) in ATOMIC_MASSES.iter() {
        if atom_mass == *mass {
            return String::from(*symbol);
        }
    }
    return String::from("fk");
}

pub fn p<T: fmt::Display>(what_to_print: T) -> () {
    println!("{}", what_to_print)
}

pub fn d<T: fmt::Debug>(what_to_print: T) -> () {
    println!("{:?}", what_to_print)
}

pub fn append_xyz(system: &Atoms, buffer: &mut BufWriter<File>) {
    write!(buffer, "{}\n\n", system.num_atoms());
    let atom_symbol = get_atomic_str(system.mass);
    let atom_iter = system
        .positions
        .column_iter()
        .zip(system.velocities.column_iter());
    for (p, v) in atom_iter {
        writeln!(
            buffer,
            " {}\t{:?}\t{:?}\t{:?}\t\t{:?}\t{:?}\t{:?}",
            atom_symbol, p[0], p[1], p[2], v[0], v[1], v[2]
        )
        .unwrap()
    }
}

impl Atoms {
    pub fn read(file_path: &String) -> Atoms {
        let f = File::open(file_path).unwrap();
        let f = BufReader::new(f);
        let lines = f.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

        let natoms = lines[0]
            .trim()
            .parse::<usize>()
            .expect("Could not parse integer");

        let mut new = Atoms::new(natoms);
        let mut name;
        for i in 0..natoms {
            let splitted = lines[i + 2].split_whitespace().collect::<Vec<_>>();
            name = splitted[0];

            new.positions.set_column(
                i,
                &Vector3::new(
                    splitted[1].parse::<f64>().expect("Could not parse float"),
                    splitted[2].parse::<f64>().expect("Could not parse float"),
                    splitted[3].parse::<f64>().expect("Could not parse float"),
                ),
            );
            if splitted.len() == 7 {
                new.velocities.set_column(
                    i,
                    &Vector3::new(
                        splitted[4].parse::<f64>().expect("Could not parse float"),
                        splitted[5].parse::<f64>().expect("Could not parse float"),
                        splitted[6].parse::<f64>().expect("Could not parse float"),
                    ),
                );
            } else {
                new.velocities.set_column(i, &Vector3::zeros());
            }
            new.forces.set_column(i, &Vector3::zeros());
            new.mass = get_atomic_mass(name)
        }
        new
    }

    pub fn write(&self, file_path: &str) {
        let path = Path::new(file_path);
        let mut f: BufWriter<File>;
        if path.exists() {
            f = BufWriter::new(File::open(path).expect("Unable to open file"));
        } else {
            f = BufWriter::new(File::create(path).expect("Unable to create file"));
        }
        write!(f, "{}\n\n", self.num_atoms());

        let atom_symbol = get_atomic_str(self.mass);
        let atom_iter = self
            .positions
            .column_iter()
            .zip(self.velocities.column_iter());
        for (p, v) in atom_iter {
            writeln!(
                f,
                " {}\t{:?}\t{:?}\t{:?}\t\t{:?}\t{:?}\t{:?}",
                atom_symbol, p[0], p[1], p[2], v[0], v[1], v[2]
            )
            .unwrap()
        }
    }
}
