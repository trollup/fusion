use powdr::riscv::continuations::{
    bootloader::default_input, /*rust_continuations, rust_continuations_dry_run,*/
};
use powdr::riscv::{compile_rust, Runtime};
use powdr::GoldilocksField;
use powdr::Pipeline;

use fusion_state::State;

use std::path::Path;
use std::time::Instant;

type Proof = ();

pub fn prove(tx: &fusion_api::Tx, pre_state: &State, post_state: &State) -> Result<Proof, String> {
    env_logger::init();

    let output_path = Path::new("output");

    let (asm_file_path, asm_contents) = compile_rust::<GoldilocksField>(
        //"./fusion-powdr-verifier",
        "/home/leo/devel/fusion/fusion-powdr-verifier",
        // TODO change "output"
        output_path,
        true,
        &Runtime::base().with_poseidon(),
        //true,
        false,
    )
    .ok_or_else(|| vec!["could not compile rust".to_string()])
    .unwrap();

    log::debug!("powdr-asm code:\n{asm_contents}");

    let mut pipeline = Pipeline::<GoldilocksField>::default()
        .from_asm_string(asm_contents.clone(), Some(asm_file_path.clone()))
        // TODO change "output"
        .with_output(output_path.into(), true)
        .add_data(42, pre_state)
        .add_data(43, tx)
        .add_data(44, &post_state.root());

    log::info!("Running powdr-riscv executor in fast mode...");
    let start = Instant::now();

    let program = pipeline.compute_analyzed_asm().unwrap().clone();
    let initial_memory = powdr::riscv::continuations::load_initial_memory(&program);
    let (trace, _mem) = powdr::riscv_executor::execute_ast::<GoldilocksField>(
        &program,
        initial_memory,
        pipeline.data_callback().unwrap(),
        &default_input(&[]),
        usize::MAX,
        powdr::riscv_executor::ExecMode::Fast,
    );

    let duration = start.elapsed();
    log::info!("Fast executor took: {:?}", duration);
    log::info!("Trace length: {}", trace.len);

    Ok(())
}
