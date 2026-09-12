use std::{env, fs, process};
use vak::{analyze_with_source, generate_llvm, parse_source};

fn usage() {
    println!("Vāk compiler 0.1.0\n");
    println!("USAGE:");
    println!("  vak check <file.vak>                 Parse and type-check a source file");
    println!("  vak emit <file.vak> [output.ll]      Write validated LLVM IR");
    println!("  vak build <file.vak> [output]        Build a native executable with Clang");
    println!("  vak help                             Show this help");
    println!("  vak version                          Show the compiler version");
    println!("\nEXAMPLES:");
    println!("  vak check examples/hello.vak");
    println!("  vak build examples/strings.vak /tmp/vak-strings");
    println!("  vak emit examples/native.vak /tmp/native.ll");
    println!("\nExit codes: 0 success, 1 source/semantic/codegen failure, 2 CLI or tool failure.");
}

fn usage_error(message: &str) -> ! {
    eprintln!("error: {message}\n");
    usage();
    process::exit(2);
}

fn main() {
    let mut args = env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "help".into());
    if matches!(command.as_str(), "help" | "--help" | "-h") {
        usage();
        return;
    }
    if command == "version" {
        println!("vak 0.1.0 (LLVM backend)");
        return;
    }
    if !matches!(command.as_str(), "check" | "emit" | "build") {
        usage_error(&format!("unknown command `{command}`"));
    }
    let path = args.next().unwrap_or_else(|| {
        usage_error("missing source file");
    });
    let output_path = args.next();
    let source = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: cannot read {path}: {e}");
            process::exit(2);
        }
    };
    let program = match parse_source(&source) {
        Ok(program) => program,
        Err(errors) => {
            for e in errors {
                eprintln!(
                    "error{}: {path}:{}:{}: {}",
                    e.code.map(|code| format!("[{code}]")).unwrap_or_default(),
                    e.span.line,
                    e.span.column,
                    e.message
                );
            }
            process::exit(1);
        }
    };
    if let Err(errors) = analyze_with_source(&program, &source) {
        for e in errors {
            eprintln!(
                "error{}: {path}:{}:{}: {}",
                e.code.map(|code| format!("[{code}]")).unwrap_or_default(),
                e.span.line,
                e.span.column,
                e.message
            );
        }
        process::exit(1);
    }
    if command == "check" {
        println!(
            "checked semantics: {} top-level item(s)",
            program.items.len()
        );
        return;
    }
    let ir = match generate_llvm(&program) {
        Ok(ir) => ir,
        Err(error) => {
            eprintln!(
                "error: {path}: LLVM code generation failed: {}",
                error.message
            );
            process::exit(1);
        }
    };
    if command == "emit" {
        let output = output_path.unwrap_or_else(|| format!("{path}.ll"));
        if let Err(error) = fs::write(&output, ir) {
            eprintln!("error: cannot write {output}: {error}");
            process::exit(2);
        }
        println!("wrote LLVM IR: {output}");
        return;
    }
    let output = output_path.unwrap_or_else(|| "a.out".into());
    let ir_path = format!("{output}.ll");
    if let Err(error) = fs::write(&ir_path, ir) {
        eprintln!("error: cannot write {ir_path}: {error}");
        process::exit(2);
    }
    match process::Command::new("clang")
        .args([&ir_path, "-o", &output])
        .status()
    {
        Ok(status) if status.success() => {
            let _ = fs::remove_file(&ir_path);
            println!("built native executable: {output}");
        }
        Ok(status) => {
            eprintln!("error: clang failed with status {status}");
            process::exit(1);
        }
        Err(error) => {
            eprintln!("error: cannot run clang: {error}");
            process::exit(2);
        }
    }
}
