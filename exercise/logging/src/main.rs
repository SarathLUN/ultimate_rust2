// START IN lib.rs!!!

use frogger::Frog;

// You did #1-#6 in lib.rs already, right?
//
// 7. Update Cargo.toml to add the `env_logger` dependency => done

fn main() {
    // 8. Initialize env_logger using the init() function at the top level of the library
    env_logger::init();

    // 9. Run this program with `cargo run` and take a look at the default output.
    // output below:

    // ➜  logging git:(l022-exercise-logging) ✗ cargo run
    //    Compiling frogger v0.1.0 (/Users/sarath.lun/Public/ultimate_rust2/exercise/logging)
    //     Finished dev [unoptimized + debuginfo] target(s) in 5.66s
    //      Running `target/debug/frogger`
    // [2024-04-10T15:25:56Z ERROR frogger] the frog is already asleep

    // - Now run it again with an explicit log level, like `RUST_LOG=info cargo run`
    // output below:

    // ➜  logging git:(l022-exercise-logging) ✗ RUST_LOG=info cargo run
    //     Finished dev [unoptimized + debuginfo] target(s) in 0.01s
    //      Running `target/debug/frogger`
    // [2024-04-10T15:26:46Z INFO  frogger] A Frog has hopped, 4 energy is left.
    // [2024-04-10T15:26:46Z INFO  frogger] A Frog has hopped, 3 energy is left.
    // [2024-04-10T15:26:46Z INFO  frogger] A Frog has hopped, 2 energy is left.
    // [2024-04-10T15:26:46Z INFO  frogger] A Frog has hopped, 1 energy is left.
    // [2024-04-10T15:26:46Z INFO  frogger] A Frog has hopped, 0 energy is left.
    // [2024-04-10T15:26:46Z WARN  frogger] the frog will go to sleep since he ran out of energy.
    // [2024-04-10T15:26:46Z ERROR frogger] the frog is already asleep

    // - or `RUST_LOG=trace cargo run`
    // output below:

    // ➜  logging git:(l022-exercise-logging) ✗ RUST_LOG=trace cargo run
    //     Finished dev [unoptimized + debuginfo] target(s) in 0.00s
    //      Running `target/debug/frogger`
    // [2024-04-10T15:27:08Z DEBUG frogger] A new Frog has been created
    // [2024-04-10T15:27:08Z TRACE frogger] default value was generated
    // [2024-04-10T15:27:08Z INFO  frogger] A Frog has hopped, 4 energy is left.
    // [2024-04-10T15:27:08Z INFO  frogger] A Frog has hopped, 3 energy is left.
    // [2024-04-10T15:27:08Z INFO  frogger] A Frog has hopped, 2 energy is left.
    // [2024-04-10T15:27:08Z INFO  frogger] A Frog has hopped, 1 energy is left.
    // [2024-04-10T15:27:08Z INFO  frogger] A Frog has hopped, 0 energy is left.
    // [2024-04-10T15:27:08Z WARN  frogger] the frog will go to sleep since he ran out of energy.
    // [2024-04-10T15:27:08Z ERROR frogger] the frog is already asleep

    let mut skippy = Frog::new();
    skippy.hop();
    skippy.hop();
    skippy.hop();
    skippy.hop();
    skippy.hop();
    skippy.sleep();

    // Challenge: Go back to lib.rs and set the `target: ` argument for each logging call to be the
    // path to the function.  For example, `Frog::new`
    // output below:

    // ➜  logging git:(l022-exercise-logging) ✗ RUST_LOG=trace cargo run
    //    Compiling frogger v0.1.0 (/Users/sarath.lun/Public/ultimate_rust2/exercise/logging)
    //     Finished dev [unoptimized + debuginfo] target(s) in 5.59s
    //      Running `target/debug/frogger`
    // [2024-04-10T15:37:47Z DEBUG Frog::new] A new Frog has been created
    // [2024-04-10T15:37:47Z TRACE Frog::default] default value was generated
    // [2024-04-10T15:37:47Z INFO  Frog::hop] A Frog has hopped, 4 energy is left.
    // [2024-04-10T15:37:47Z INFO  Frog::hop] A Frog has hopped, 3 energy is left.
    // [2024-04-10T15:37:47Z INFO  Frog::hop] A Frog has hopped, 2 energy is left.
    // [2024-04-10T15:37:47Z INFO  Frog::hop] A Frog has hopped, 1 energy is left.
    // [2024-04-10T15:37:47Z INFO  Frog::hop] A Frog has hopped, 0 energy is left.
    // [2024-04-10T15:37:47Z WARN  Frog::hop] the frog will go to sleep since he ran out of energy.
    // [2024-04-10T15:37:47Z ERROR Frog::sleep] the frog is already asleep
}
