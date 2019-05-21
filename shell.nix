with import <nixpkgs> { }; 

runCommand "dummy" {
    buildInputs = [ cargo clang openssl slurm pkgconfig ];
    LIBCLANG_PATH="${llvmPackages.libclang}/lib"; 
} ""
