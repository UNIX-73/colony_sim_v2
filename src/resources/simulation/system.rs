use super::step::StepUnit;

pub trait SimulationSystem {
    fn frequency() -> StepUnit;
    fn priority() -> Option<u32>;
    fn run();
}
