with import <nixpkgs> { }; 

runCommand "dummy" {
    buildInputs = [ perl cargo rustup clang openssl slurm pkgconfig ];
    LIBCLANG_PATH="${llvmPackages.libclang}/lib"; 
} ""
