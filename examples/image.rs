fn main(){
    let input = vec![];
    let module = parse(input);
    let executor = Executor::new(module);
    let result = executor.exec(param{
        func_name: "add",
        params: vec![1, 2],
    })
}
