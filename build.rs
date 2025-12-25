use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Put the memory.x linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // Compile TivaWare USB device C files
    compile_tivaware_usb();

    // Only re-run if memory.x changes
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=build.rs");
}

fn compile_tivaware_usb() {
    let tivaware_path = "TivaWare_C_Series-2.2.0.295";
    
    // Define include paths
    // Note: C files use #include "inc/hw_ints.h", so the include path
    // should be the TivaWare root directory, not the inc subdirectory
    let tivaware_root = tivaware_path;
    
    // Create compiler with common settings
    let mut build = cc::Build::new();
    build
        .compiler("arm-none-eabi-gcc")
        .flag("-mcpu=cortex-m4")
        .flag("-mthumb")
        .flag("-mfloat-abi=hard")
        .flag("-mfpu=fpv4-sp-d16")
        .flag("-ffunction-sections")
        .flag("-fdata-sections")
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-Wno-unused-parameter")
        .flag("-Os")
        .include(tivaware_root)  // Root path so "inc/hw_ints.h" resolves correctly
        .define("TARGET_IS_TM4C123_RB1", None)
        .define("PART_TM4C123GH6PM", None)
        .define("gcc", None);  // Define compiler type like TivaWare does
    
    // Compile USB device core files
    build
        .file(format!("{}/usblib/device/usbdenum.c", tivaware_path))
        .file(format!("{}/usblib/device/usbdhandler.c", tivaware_path))
        .file(format!("{}/usblib/device/usbdconfig.c", tivaware_path))
        .file(format!("{}/usblib/device/usbdcdesc.c", tivaware_path));
    
    // Compile USB CDC serial class (for serial port functionality)
    build.file(format!("{}/usblib/device/usbdcdc.c", tivaware_path));
    
    // Compile usblib core files
    build
        .file(format!("{}/usblib/usbmode.c", tivaware_path))
        .file(format!("{}/usblib/usbulpi.c", tivaware_path))
        .file(format!("{}/usblib/usbtick.c", tivaware_path))
        .file(format!("{}/usblib/usbbuffer.c", tivaware_path))
        .file(format!("{}/usblib/usbringbuf.c", tivaware_path))
        .file(format!("{}/usblib/usbdesc.c", tivaware_path))
        .file(format!("{}/usblib/usbdma.c", tivaware_path));  // DMA support for USB
    
    // Compile driverlib USB functions
    build.file(format!("{}/driverlib/usb.c", tivaware_path));
    
    // Compile additional driverlib files needed by USB code
    // These provide sysctl, interrupt, gpio, cpu, fpu, systick functions that USB code calls
    build
        .file(format!("{}/driverlib/sysctl.c", tivaware_path))
        .file(format!("{}/driverlib/interrupt.c", tivaware_path))
        .file(format!("{}/driverlib/gpio.c", tivaware_path))
        .file(format!("{}/driverlib/cpu.c", tivaware_path))
        .file(format!("{}/driverlib/fpu.c", tivaware_path))
        .file(format!("{}/driverlib/systick.c", tivaware_path));
    
    // Compile and link
    build.compile("tivaware_usb");
    
    // Tell cargo to link the library
    println!("cargo:rustc-link-lib=static=tivaware_usb");
    
    // Rebuild if any of these files change
    println!("cargo:rerun-if-changed={}/usblib/device/usbdenum.c", tivaware_path);
    println!("cargo:rerun-if-changed={}/usblib/device/usbdhandler.c", tivaware_path);
    println!("cargo:rerun-if-changed={}/usblib/device/usbdconfig.c", tivaware_path);
    println!("cargo:rerun-if-changed={}/usblib/device/usbdcdesc.c", tivaware_path);
    println!("cargo:rerun-if-changed={}/usblib/device/usbdcdc.c", tivaware_path);
    println!("cargo:rerun-if-changed={}/usblib/usbmode.c", tivaware_path);
    println!("cargo:rerun-if-changed={}/usblib/usbulpi.c", tivaware_path);
    println!("cargo:rerun-if-changed={}/usblib/usbtick.c", tivaware_path);
    println!("cargo:rerun-if-changed={}/usblib/usbbuffer.c", tivaware_path);
    println!("cargo:rerun-if-changed={}/usblib/usbringbuf.c", tivaware_path);
    println!("cargo:rerun-if-changed={}/usblib/usbdesc.c", tivaware_path);
    println!("cargo:rerun-if-changed={}/usblib/usbdma.c", tivaware_path);
    println!("cargo:rerun-if-changed={}/driverlib/usb.c", tivaware_path);
    println!("cargo:rerun-if-changed={}/driverlib/sysctl.c", tivaware_path);
    println!("cargo:rerun-if-changed={}/driverlib/interrupt.c", tivaware_path);
    println!("cargo:rerun-if-changed={}/driverlib/gpio.c", tivaware_path);
    println!("cargo:rerun-if-changed={}/driverlib/cpu.c", tivaware_path);
    println!("cargo:rerun-if-changed={}/driverlib/fpu.c", tivaware_path);
    println!("cargo:rerun-if-changed={}/driverlib/systick.c", tivaware_path);
}

