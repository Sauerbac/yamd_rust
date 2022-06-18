use crate::atoms::atoms::Atoms;
use na::Matrix3xX;

impl Atoms {
    pub fn verlet1(mut self, timestep: f64) -> Atoms {
        self.velocities = self.velocities + 0.5 * &self.forces * timestep / self.mass;
        self.positions = self.positions + &self.velocities * timestep;
        self
    }
    pub fn verlet2(mut self, timestep: f64) -> Atoms {
        // self.velocities = &self.velocities
        //     + Matrix3xX::from_element(self.num_atoms(), 1.0 + 0.5 * timestep / self.mass);
        self.velocities = &self.velocities + 0.5 * &self.forces * timestep / self.mass;
        self
    }
    pub fn verlet1_backup(&mut self, timestep: f64) -> () {
        self.velocities = self.velocities.to_owned() + 0.5 * &self.forces * timestep / self.mass;
        self.positions = self.positions.to_owned() + &self.velocities * timestep;
    }
    pub fn verlet2_backup(&mut self, timestep: f64) -> () {
        self.velocities = self.velocities.to_owned()
            + Matrix3xX::from_element(self.num_atoms(), 1.0 + 0.5 * timestep / self.mass)
    }
}
