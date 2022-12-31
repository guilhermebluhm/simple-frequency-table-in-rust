use std::collections::HashMap;

fn main() {
    
    let v = geracao_frequencia_vetor(&mut vec![0,0,1,2,5,1,6,1,1,9,9,9,9,1,2,5,1,6]);
    println!("{:?}", v);

}

fn geracao_frequencia_vetor(vet: &mut Vec<i32>) -> HashMap<i32, u32>{

    let mut v:HashMap<i32, u32> = HashMap::new();

    for i in vet.iter(){

        let mut freq: &mut u32 = v.entry(*i).or_insert(0);
        *freq += 1;

    }

    v

}
