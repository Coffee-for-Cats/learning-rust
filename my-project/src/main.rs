//This will use another module (file), on the same directory.
mod garden;
//This uses another module, but this module is a directory!
//so, inside the directory, It will use the "mod.rs" as the module,
//and I can use many paths (files) conected on mod.rs as a module.
//This gets hello from garden, so I can use it just as "hello()".
use garden2::hello;
//To use the above, I need to import the whole module:
mod garden2;

fn main() {
    hello();
    garden::hello();
}
