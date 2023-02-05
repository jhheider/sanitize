use crate::clap;

#[test]
fn test_command_setup() {
    args::setup().debug_assert();
}
