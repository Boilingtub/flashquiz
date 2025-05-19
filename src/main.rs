use rand::Rng;
use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::thread::sleep;
use std::time::Duration;

static STANDARDSLOWPRINTTIME: u64 = 5;

fn slow_print(instr: &str, time: u64) {
    for c in instr.chars() {
        print!("{}", c);
        let _ = io::stdout().flush();
        sleep(Duration::from_millis(time));
    }
}

fn usr_input(prompt: &str) -> String {
    slow_print(format!("{}", prompt).as_str(), STANDARDSLOWPRINTTIME);
    let _ = io::stdout().flush();

    let mut instr = String::new();
    io::stdin()
        .read_line(&mut instr)
        .expect("Failed to read input");

    instr
}

fn decompose_quiz(quizstr: &str) -> (Vec<&str>, Vec<&str>) {
    let mut questions: Vec<&str> = vec![];
    let mut awnsers: Vec<&str> = vec![];

    let mut firstindex = 0;
    let mut lastindex = 0;
    let quizcharcount = quizstr.chars().count() - 2;

    while lastindex < quizcharcount {
        match quizstr[firstindex..].find("T:[") {
            Some(v) => firstindex += v + 2,
            None => {
                println!("first index failed , firstindex = {}", firstindex);
                break;
            }
        }
        match quizstr[firstindex..].find("}]") {
            Some(v) => lastindex = firstindex + v + 2,
            None => {
                println!("last index failed , lastindex = {}", lastindex);
                break;
            }
        }

        let substr: &str = &quizstr[firstindex..lastindex];

        match substr.find("[Q:{") {
            Some(v) => {
                let questionfirstindex = v + 4;
                match substr.find("},A:{") {
                    Some(questionlastindex) => {
                        questions.push(&substr[questionfirstindex..questionlastindex]);
                    }
                    None => {
                        println!("question last index failed");
                        break;
                    }
                };
            }
            None => {
                println!("question first index failed");
                break;
            }
        }
        match substr.find("},A:{") {
            Some(v) => {
                let awnserfirstindex = v + 5;
                match substr.find("}]") {
                    Some(awnserlastindex) => {
                        awnsers.push(&substr[awnserfirstindex..awnserlastindex])
                    }
                    None => {
                        //println!("{}",substr);
                        println!("\nawnser last index failed\n");
                        break;
                    }
                }
            }
            None => {
                println!("\nawnser first index failed\n");
                break;
            }
        }
        //println!("\n Quizstr.len() ={:?} \n firstindex={:?} \n lastindex={:?} \n substr={:?}"
        //       ,quizcharcount,firstindex,lastindex , substr);
        //usr_input("break");
    }
    return (questions, awnsers);
}

fn genrandomnumarr(lownum: u32, highnum: u32) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut randarr: Vec<u32> = (lownum..highnum).collect();

    let mut i = lownum;
    while i < highnum {
        let randnum = rng.gen_range(lownum..highnum);
        if randnum != i.try_into().unwrap() {
            randarr.swap(
                i.try_into().unwrap(),
                <u32 as TryInto<usize>>::try_into(randnum).unwrap(),
            );
        }
        i = i + 1;
    }

    return randarr;
}

fn startquiz(questions: Vec<&str>, awnsers: Vec<&str>) {
    let quizorderarr: Vec<u32> = genrandomnumarr(0, questions.len().try_into().unwrap());

    for x in quizorderarr.iter() {
        let v = *x as usize;
        slow_print(
            format!("\n{}\n", questions[v]).as_str(),
            STANDARDSLOWPRINTTIME,
        );
        let usr_awnser = usr_input("\nYour Awnser >>");

        if usr_awnser.trim() == "stop!".to_string().trim() {
            break;
        }

        if usr_awnser.trim() == awnsers[v].trim() {
            slow_print("CORRECT!!!\n", STANDARDSLOWPRINTTIME);
        }

        slow_print(
            format!("\nCorrect awnser: -> {}\n", awnsers[v]).as_str(),
            STANDARDSLOWPRINTTIME,
        );
        slow_print("--------------------\n", STANDARDSLOWPRINTTIME);
        usr_input("");
    }
}

fn list_quizfiles(dir_path: &str) -> Result<Vec<std::path::PathBuf>, io::Error> {
    use std::path::Path;
    let path = Path::new(dir_path);

    let path_str: &str = match path.is_dir() {
        true => dir_path,
        false => {
            println!("Provided path does not exist !\ndefaulting to \"./\"");
            "./"
        }
    };

    Ok(fs::read_dir(path_str)?
        .filter_map(|res| res.ok())
        .map(|f| f.path())
        .filter_map(|path| {
            if path.extension().map_or(false, |ext| ext == "qz") {
                Some(path)
            } else {
                None
            }
        })
        .collect())
}
fn selectquizfile(dir_path: &str) -> String {
    let availablequizez = list_quizfiles(dir_path).unwrap();
    if availablequizez.len() == 0 {
        println!("No Quizzes found");
        return "".to_string();
    } else {
        let mut selectedquiz = "";
        while selectedquiz.len() == 0 {
            for (i, q) in availablequizez.iter().enumerate() {
                slow_print(
                    format!("{}.) {}\n", i, q.as_path().display()).as_str(),
                    STANDARDSLOWPRINTTIME,
                );
            }

            let selectval = usr_input("Select available quiz by number -> ")
                .trim()
                .parse::<u32>();
            match selectval {
                Ok(v) => {
                    if v < availablequizez.len().try_into().unwrap() {
                        selectedquiz = availablequizez
                            [<u32 as TryInto<usize>>::try_into(v).unwrap()]
                        .as_path()
                        .to_str()
                        .unwrap();
                    } else {
                        println!("No Such quiz exists");
                    }
                }
                Err(v) => {
                    println!("You did not enter a valid number.");
                    println!("You entered \"{}\"", v)
                }
            }
        }

        return selectedquiz.to_owned();
    }
}

fn main() {
    slow_print("Welcome to flashquiz !\n", STANDARDSLOWPRINTTIME);
    let args: Vec<String> = env::args().collect();
    let quizfilepath;

    let path = if args.len() > 1 { &args[1] } else { "./" };

    quizfilepath = selectquizfile(path);
    if quizfilepath.len() == 0 {
        return;
    }

    let quizfile = fs::read_to_string(quizfilepath).expect("Quiz file opened");
    let (questions, awnsers): (Vec<&str>, Vec<&str>) = decompose_quiz(&quizfile);

    usr_input("Stop the quiz at any time by typing \"stop!\"");
    usr_input("Press Enter to start quiz...");
    startquiz(questions, awnsers);

    slow_print("END OF QUIZ", STANDARDSLOWPRINTTIME);
}
