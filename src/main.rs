use std::fs::File;
use object_rewrite::Rewriter;

fn main() {
        let path = "main_cpp_clang_03_with_inlined_3s";
        let file = File::open(path).unwrap();
        let file = unsafe { memmap2::Mmap::map(&file).unwrap() };
        Rewriter::read(&*file).unwrap();
}
