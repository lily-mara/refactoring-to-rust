use std::env;
use std::path::PathBuf;
use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    params, VmBuilder, WasmVal
};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut args: Vec<String> = env::args().skip(1).collect();
    args.reverse();
    let target = args.pop().unwrap_or("paper_search".to_string());
    let filename = format!("{}.wasm", target);
    let wasm_file: PathBuf = ["..", "target", "wasm32-wasi", "debug", filename.as_str()].iter().collect();
    
    // create a config with the `wasi` option enabled
    let config = ConfigBuilder::new(CommonConfigOptions::default())
        .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
        .build()?;
    assert!(config.wasi_enabled());

    // create a VM with the config
    let term = args.pop().unwrap_or("type".to_string());
    let mut vm = VmBuilder::new().with_config(config).build()?; 

    vm.wasi_module_mut()
        .expect("Not found wasi module")
        .initialize(None, None, None);
    let m = vm.clone()
        .register_module_from_file(target.to_string(), &wasm_file)?;
    let env_instance = m.named_module(target.to_string())?;

    let exec = vm.executor(); 

    let mut memory = env_instance.memory("memory")?; 
    let allocate = env_instance.func("allocate")?; 
    let search = env_instance.func("memory_search")?; 

    let term_len: i32 = term.len() as i32;
    let iptr = allocate.run(exec, params!(term_len))?[0].to_i32();
    let uptr: u32 = iptr as u32; 
    memory.write(term, uptr);

    let iresptr = search.run(exec, params!(iptr))?[0].to_i32();
    let uresptr:  u32 = iresptr as u32; 
    let val = memory.read_string(uresptr, 1024)?; 
    let val = val.trim_matches(char::from(0));

    println!("{:?}", val);
    Ok(())
}