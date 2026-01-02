pub mod core;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn run_app() {
        use crate::core::managers::windowing::manager::WindowConfiguration;
        use crate::core::context::Context;

        fn setup(context: &mut Context) {}
        fn update(context: &mut Context) {}
        
        pollster::block_on(crate::core::managers::windowing::manager::initialize_application(
            Some(WindowConfiguration::default()), setup, update
        ))
    }
}
