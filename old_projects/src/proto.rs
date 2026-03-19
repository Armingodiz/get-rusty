// we do this in a separate file so we could use it in other files, otherwise we would have to repeat the code in every file that needs it.
// also for main and server we have to use same refrence or else it will not work(not imp error)
pub mod hello_world {
    tonic::include_proto!("helloworld");
}