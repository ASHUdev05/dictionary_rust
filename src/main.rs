use std::io;

const ROW:usize = 2;
const COL:usize = 3;
fn meaning(word: &str) -> &str {    
    let dict: [[&str; COL]; ROW] = [
        ["ambigue","abduce","aberrated"],
        ["An ambiguous statement or expression.","To draw or take away.","Characterized by defects, abnormality, or deviation from the usual, typical, or expected course."],
    ];
    for i in 0..1 {
        for j in 0..COL {
            if dict[i][j] == word {
                return dict[i+1][j];
            }
        }
    }
        return "Word doesnt exists!";
}
fn main() {
    println!("Enter a word:");
    let mut word = String::new();
    io::stdin().read_line(&mut word).expect("Failed to read line");
    let query:String = word.trim().to_string();
    if meaning(word.trim()) != "Word doesnt exists!" {
    println!("Meaning of {:?} is {:?}", query, meaning(word.trim()));
    }
    else {
        println!("Word doesnt exists!");
    }
}
