
use std::collections::HashMap;
fn main() {
    let numbers=  vec![54, 29, 75, 100];
    let mut numeros = vec![90,28,45,98,10,10,10,10,10,34,34,34,78,159,8,50, 50, 50 ];

    print!("The mean of {:?} is ",&numbers);
    println!("{}",mean(numbers));
    numeros.sort();

    println!("Median is {:?}", median(&numeros));
    let mapa = mkmap(numeros);
    println!("The Hash Map is {:?}", mapa);
    println!("Mode is {:?}", mode(&mapa));
}

fn mean(numbers: Vec<usize>) -> usize{

    let len= numbers.len();
    let mut sum = 0; 

    for i in &numbers{
        sum += i;
        
    }
    let a = sum/len;
    a
}

fn median(num:&Vec<usize>) -> usize{

    let len = num.len();
    let med ;
    
   if len%2 ==0 {
    let mid = len/2;
    med = (num[mid] + num[mid+1])/2;
        
    }
    else{
        let mid = (len-1)/2;
        med = num[mid];
    }
    med
}

fn mkmap(num:Vec<usize>) -> HashMap<usize,i32>{
    let mut map = HashMap::new();
    for n in num{
        let count = map.entry(n).or_insert(0);
        *count +=1;
    }
    let mapa = map;
    mapa
}
fn mode<K, V>(mapa: &HashMap<K,V>) -> Option<&K>
where 
V:Ord,
{
    mapa
    .iter()
    .max_by(|a,b| a.1.cmp(&b.1))
    .map(|(k,_v)|k)
}

