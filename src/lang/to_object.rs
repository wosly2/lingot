use std::vec;

use super::super::debug::*;

#[derive(Debug)]
pub enum VerbForm {
    TensePresent,
    TenseFuture,
    TensePast,
    Infinitive
}

#[derive(Debug)]
pub enum Deixis {
    NonSpatial,
    Proximal,
    Immediate,
    Distal
}

#[derive(Debug)]
pub enum Keyword {
    // first string in these spots is pretty much just the root
    Adjective(String),
    Nominative(String, bool), // bool plural
    Verbal(String, VerbForm),
    VerbalAdjective(String),
    Prepositional(String),
    AdjectAdjective(String),

    // the non-root-inflected ones
    CompletiveAspect,
    ProgressiveAspect,
    HabitualAspect,
    PerfectAspect,

    DefiniteArticle(Deixis),
    IndefiniteArticle(Deixis),

    DeicticSpatialNoun(Deixis),
    DeicticTemporalNoun(Deixis),
}

pub fn keyword_from_string(keyword: &str, parameters: Vec<String>) -> Result<Keyword, String> {
    fn deixis_from_string(deixis_string: String, allow_nonspatial: bool) -> Result<Deixis, String> {
        Ok(match deixis_string.as_str() {
            "nspac" => if allow_nonspatial {Deixis::NonSpatial} else {return Err("you may not use non spatial deixis here".to_string())},
            "prox" => Deixis::Proximal,
            "imm" => Deixis::Immediate,
            "dist" => Deixis::Distal,
            _ => return Err(format!("no such valid deixis {} in this context", deixis_string))
        })
    }
    // error handling
    if !vec!["adj","nom","verb","vadj","prep","aadj","aspComp","aspProg","aspHabt","aspPerf","artDef","artIndef","dNounSpac","dNounTemp"]
        .contains(&keyword) { return Err(format!("Unkown keyword {}", keyword))}
    if match keyword {
        "adj" => 1,
        "nom" => 2,
        "verb" => 2,
        "vadj" => 1,
        "prep" => 1,
        "aadj" => 1,
        "aspComp" => 0,
        "aspProg" => 0,
        "aspHabt" => 0,
        "aspPerf" => 0,
        "artDef" => 1,
        "artIndef" => 1,
        "dNounSpac" => 1,
        "dNounTemp" => 1,
        _ => 1000,
    } != parameters.len() as i32 {
        return Err("Invalid length of keyword parameters".to_string());
    }
    // match conversion
    Ok(match keyword {
        "adj" => Keyword::Adjective(parameters[0].clone()),
        "nom" => Keyword::Nominative(parameters[0].clone(), {
            if parameters[1] == "1" {true} else {false}
        }),
        "verb" => Keyword::Verbal(parameters[0].clone(), {
            match parameters[1].as_str() {
                "pres" => VerbForm::TensePresent,
                "fut" => VerbForm::TenseFuture,
                "past" => VerbForm::TensePast,
                "inf" => VerbForm::Infinitive,
                _ => return Err(format!("Unknown parameter {} for verb form", parameters[1]))
            }
        }),
        "vadj" => Keyword::VerbalAdjective(parameters[0].clone()),
        "prep" => Keyword::Prepositional(parameters[0].clone()),
        "aadj" => Keyword::AdjectAdjective(parameters[0].clone()),

        "aspComp" => Keyword::CompletiveAspect,
        "aspProg" => Keyword::ProgressiveAspect,
        "aspHabt" => Keyword::HabitualAspect,
        "aspPerf" => Keyword::PerfectAspect,

        "artDef" => Keyword::DefiniteArticle(deixis_from_string(parameters[0].clone(), true)?),
        "artIndef" => Keyword::IndefiniteArticle(deixis_from_string(parameters[0].clone(), true)?),
        "dNounSpac" => Keyword::DeicticSpatialNoun(deixis_from_string(parameters[0].clone(), false)?),
        "dNounTemp" => Keyword::DeicticTemporalNoun(deixis_from_string(parameters[0].clone(), false)?),

        
        _ => return Err(format!("Unkown keyword {}", keyword))
    })
}

impl Deixis {
    pub fn as_str(&self) -> &str {
        match self {
            Deixis::NonSpatial => "non_spatial",
            Deixis::Proximal => "proximal",
            Deixis::Immediate => "immediate",
            Deixis::Distal => "distal",
        }
    }
}

pub fn to_object(text: &str) -> Result<Vec<Keyword>, String> {
    const DEBUG_PRINT: bool = false;

    let mut cleaned = String::new();

    for line in text.lines() {
        let no_comment = match line.find(";") {
            Some(i) => {
                &line[0..i]
            },
            None => line
        };

        cleaned.push_str(format!(" {}", no_comment).as_str());
    }

    let mut reading_keyword = false;
    let mut reading_parameters = false;
    let mut reading_parameter = false;
    let mut last_char_was_space = false;

    let mut word = String::new();
    let mut keyword = String::new();
    let mut parameters: Vec<String> = vec![];

    let mut objects: Vec<Keyword> = vec![];

    const SYMBOL_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

    for char in cleaned.chars() {
        // symbol creation
        if SYMBOL_CHARS.contains(char) {
            if !reading_keyword {reading_keyword = true; word.clear()}
            else if reading_parameters && !reading_parameter {reading_parameter = true; word.clear()};

            word.push(char);
        }

        // starting parameter
        if char == '{' {
            if reading_keyword {
                if DEBUG_PRINT {println!("keyword {}", word);};
                keyword = word.clone();
                parameters = vec![]; // clear our parameters now
                reading_parameters = true; word.clear();
            } else {
                return Err("Invalid syntax: parameters have no keyword body".to_string())
            }
        }

        // closing parameter
        if char == '}' {
            if !reading_keyword || !reading_parameters || !reading_parameter {
                return Err("Invalid syntax, closing bracket on no parameter body".to_string())
            } else {
                if DEBUG_PRINT {println!("  parameter {}", word);};
                parameters.push(word.clone());

                // push keyword and parameters
                if DEBUG_PRINT {println!("pushing, keyword={}, parameters={:?}", keyword, parameters);}
                objects.push(keyword_from_string(keyword.clone().as_str(), parameters.clone())?);
                parameters = vec![];

                reading_keyword = false; reading_parameters = false; reading_parameter = false; word.clear();
            }
        }
        
        // spaces
        if char == ' ' && !last_char_was_space {
            if reading_keyword {
                if !reading_parameters {
                    if DEBUG_PRINT {println!("keyword {}", word);}

                    // push with no arguments
                    if DEBUG_PRINT {println!("pushing, keyword={}, NO PARAMETERS", keyword);}
                    objects.push(keyword_from_string(word.clone().as_str(), vec![])?);
                    
                    reading_parameters = false; word.clear()
                } else {
                    if reading_parameter {
                        if DEBUG_PRINT {println!("  parameter {}", word);}

                        // add parameter
                        parameters.push(word.clone());

                        reading_parameter = false; word.clear();
                    }
                }
            }

            last_char_was_space = true
        } else if char == ' ' {
            last_char_was_space = true
        } else {
            last_char_was_space = false
        }
    };

    Ok(objects)
}