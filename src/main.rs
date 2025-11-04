//use rand::Rng;
use clap::{Command,ArgAction,Arg};
fn main() {
    let matches = Command::new("rps")
        .version("0.1.0")
        .author("Mohammad")
        .about("Command-line rock paper scissors. Play by inputing your choices (you can play multiple rounds in one session by typing your choices, for example:\nrps p -> to play one round  and choose paper.\nrps rps -> to play three rounds and choose rock, paper, scissors, in this sequence.) ")
        .arg(
            Arg::new("choices")
                .value_name("CHOICES")
                .help("Input your choices for \'r\'ock, \'p\'aper, or \'s\'cissors.")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Prints warnings of unrecognized characters. They are normally ignored without warnings.")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("Only prints the result screen.")
                .action(ArgAction::SetTrue),
        )
        .get_matches();
    let verbose:bool=matches.get_flag("verbose");
    let quiet:bool = matches.get_flag("quiet");
    
    // CANNOT HAVE VERBOSE AND QUIET AT THE SAME TIME
    if verbose && quiet{
        eprintln!("------------------------------------------------------------------------\n\
                   | ERROR! CANNOT HAVE \'verbose\' AND \'quiet\' AT THE SAME TIME. ABORTING. |\n\
                   ------------------------------------------------------------------------");
        std::process::exit(26);
    }

    let input_vector:Vec<String> = matches.get_many("choices").unwrap().cloned().collect();
    let input = input_vector.join("");
    let mut wins = 0;
    let mut losses = 0;
    let mut draws = 0;
    let v_input = validate_input(&input,verbose);
    for player_choice in v_input.chars(){
        let cpu_choice = get_computer_choice();
        let result = compare_choices(player_choice,cpu_choice); 
        if result == 'w' {
            wins += 1;
        }
        else if result == 'l'{
            losses += 1;
        }else if result == 'd'{
            draws += 1;
        }else{
            eprintln!("ERROR OCCURED IN MAIN, UNEXPECTER result VALUE");
            std::process::exit(27);
        }
        if !quiet {
            print_result(player_choice,cpu_choice,result);
        }
    }
    print_score(wins,losses,draws,quiet);
}


fn print_score(wins:usize,losses:usize,draws:usize, _quiet:bool){
    println!("------------GAME OVER-------------");
    println!("\ttotal  wins:\t{wins}\n\ttotal losses:\t{losses}\n\ttotal  draws:\t{draws}");
}

fn validate_input(input:&str,verbose:bool) -> String{
    let input = &input.trim().to_lowercase();
    let mut v_input = String::new();
    'outer: for character in input.chars() {
        if character != 'r'&& character !='p' && character !='s'{
            if verbose{
                eprintln!("WARNING:UNRECOGNIZED CHARACTER \'{character}\', IGNORING IT.");
            }
            continue 'outer;
        }
        v_input.push_str(&character.to_string());
    }
    if v_input.is_empty(){
    
        eprintln!("ERROR, NO VALID INPUT FOUND.\nPLEASE INPUT \'r\' OR \'p\' OR \'s\'");
        std::process::exit(28);
    }
    v_input
}



fn print_result(player:char,cpu:char, result:char){
    println!("PLAYER HAS CHOSEN: {player}, CPU HAS CHOSEN: {cpu}"); 
    if result == 'w'{
        println!("\tplayer won!\n");
    }else if result == 'l'{
        println!("\tplayer lost!\n");
    }else if result == 'd'{
        println!("\tit\'s a draw!\n");
    }else{
        eprintln!("ERROR IN FUNCTION print_result(result:char) THE FUNCTION CANNOT EXCEPT ANY VALUE EXCEPT (\'w\', \'l\', OR \'d\'\nIT IS CASE SENSITIVE!");
        std::process::exit(29);
    }
}

fn get_computer_choice() -> char{
    let rng = rand::random_range(0..=2);
    if rng == 0{
        return 'r';
    } else if rng == 1{
        return 'p';
    } else {
        return 's';
    }
}
// returns 'w' if player won. 'l' if player lost. 'd' if it's a draw
fn compare_choices(player:char,cpu:char) -> char{
    if player == 'r'{
        if cpu == 'r'{
            return 'd';
        }else if cpu == 'p'{
            return 'l';
        }else if cpu == 's'{
            return 'w';
        }
    }else if player == 'p'{
        if cpu == 'r'{
            return 'w';
        }else if cpu == 'p'{
            return 'd';
        }else if cpu == 's'{
            return 'l';
        }
    }else if player == 's'{
        if cpu == 'r'{
            return 'l';
        }else if cpu =='p'{
            return 'w';
        }else if cpu == 's'{
            return 'd'
        }
    }
    eprintln!("ERROR IN FUNCTION comparechoices(player:char,cpu:char)-> char COULD NOT COMPARE CHOICES.",);
    std::process::exit(30);
}
// TEST FUNCTION FOR DEVOLEPMENT REASONS.  
/* fn test () {
 let test_choices = ['r','p','s'];
  for testa in test_choices{
        for testb in test_choices{
            let result = compare_choices(testa,testb);
            print_result(testa, testb, result);
        }
        println!();
    }
    println!("---------\ntest finished.");
}*/
