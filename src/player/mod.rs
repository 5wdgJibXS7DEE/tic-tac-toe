pub mod ai;
pub mod human;

pub trait Play {
    fn get_symbol(&self) -> char;
    fn play(&mut self) -> u16;
}
