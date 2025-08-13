use std::fmt::format;
use std::fs;
use std::error::Error;
use std::ops::IndexMut;

use serde_yaml::{from_value, Value};

use super::*;

pub struct Lang {
    forms: Value,
    roots: Value,
}

fn get_type<T>(keys: Vec<&str>, value: &Value) -> Result<T, serde_yaml::Error>
where T: serde::de::DeserializeOwned {
    // {dbg!(&keys);
    // dbg!(value);}
    let mut spot = value;
    for i in 0..(keys.len()-1) {
        spot = &spot[keys[i]];
    };
    let t: T = from_value(spot[keys[keys.len()-1]].clone())?; /* ? /* ? */ */
    Ok(t)
}

impl Keyword {
    pub fn render(&self, lang: &Lang) -> Result<String, serde_yaml::Error> {
        Ok(match &self {
            Keyword::Adjective(root) => {
                Keyword::format_base("adjective", root, lang)? /* ? */
            },
            Keyword::Nominative(root, plural) => {
                format!(
                    "{}{}",
                    Keyword::format_base("nominative", root, lang)?, /* ? /* ? */ */
                    if *plural {
                        let ending: String = get_type(vec!["noun_plurality_suffix"], &lang.forms)?; /* ? /* ? */ */
                        ending
                    } else {
                        "".to_string()
                    }
                )
            },
            Keyword::Verbal(root, form) => {
                format!(
                    "{}{}",
                    Keyword::format_base("verbal", root, lang)?, /* ? /* ? */ */
                    {
                        let form_keys = match form {
                            VerbForm::TensePresent => vec!["verb_tense_endings", "present"],
                            VerbForm::TenseFuture => vec!["verb_tense_endings", "future"],
                            VerbForm::TensePast => vec!["verb_tense_endings", "past"],
                            VerbForm::Infinitive => vec!["verb_infinitive_suffix"],
                        };
                        let ending: String = get_type(form_keys, &lang.forms)?; /* ? /* ? */ */
                        ending
                    }
                )
            },
            Keyword::VerbalAdjective(root) => {Keyword::format_base("verbal_adjective", root, lang)? /* ? */},
            Keyword::Prepositional(root) => {Keyword::format_base("prepositional", root, lang)? /* ? */},
            Keyword::AdjectAdjective(root) => {Keyword::format_base("adject_adjective", root, lang)? /* ? */},

            Keyword::CompletiveAspect => get_type(vec!["verb_particles", "aspect", "completive"], &lang.forms)?,
            Keyword::ProgressiveAspect => get_type(vec!["verb_particles", "aspect", "progressive"], &lang.forms)?,
            Keyword::HabitualAspect => get_type(vec!["verb_particles", "aspect", "habitual"], &lang.forms)?,
            Keyword::PerfectAspect => get_type(vec!["verb_particles", "aspect", "perfect"], &lang.forms)?,

            Keyword::DefiniteArticle(deixis) => get_type(vec!["article", "definite", deixis.as_str()], &lang.forms)?, /* ? /* ? */ */
            Keyword::IndefiniteArticle(deixis) => get_type(vec!["article", "indefinite", deixis.as_str()], &lang.forms)?, /* ? /* ? */ */
            Keyword::DeicticSpatialNoun(deixis) => get_type(vec!["deictic_nouns", "spatial", deixis.as_str()], &lang.forms)?, /* ? /* ? */ */
            Keyword::DeicticTemporalNoun(deixis) => get_type(vec!["deictic_nouns", "temporal", deixis.as_str()], &lang.forms)?, /* ? /* ? */ */
        })
    }

    fn format_base(of_type: &str, root: &str, lang: &Lang) -> Result<String, serde_yaml::Error> {
        let cons: String = get_type(vec![root], &lang.roots)?; /* ? /* ? */ */
        let mold: String = get_type(vec![
            "root_form",
            cons.len().to_string().as_str(), // format!("{}", cons.len()).as_str(), // idk which is faster
            of_type
        ], &lang.forms)?; /* ? /* ? */ */
        let cons = cons.as_str(); let mold = mold.as_str();

        Ok(Keyword::format_patterns(cons, mold))    
    }

    fn format_patterns(cons_pattern: &str, mold_pattern: &str) -> String {
        let mut out_string: Vec<char> = mold_pattern.chars().collect();
        for i in 0..cons_pattern.len() {
            let out_string_str: String = out_string.iter().cloned().collect();
            let replace = match out_string_str.find("-") {
                Some(u) => u,
                None => panic!("pattern is missing one or more indicators for consonant patters (denoted by `-`)!")
            };
            out_string[replace] = match cons_pattern.chars().nth(i) {
                Some(c) => c,
                None => panic!("the code has a bug, this should never go wrong based on inputs")
            };
        }

        let out: String = out_string.into_iter().collect();
        out
    }
}

pub fn render_keywords(keywords: Vec<Keyword>, lang: &Lang) -> String {
    let mut out_string = String::new();
    for keyword in keywords {
        let rendered = match keyword.render(lang) {
            Ok(s) => s,
            Err(e) => panic!("{}", e)
        };
        out_string.push_str(&rendered.as_str());
        out_string.push(' ');
    }
    out_string
}

impl Lang {
    pub fn load(static_path: &str) -> Result<Lang, Box<dyn Error>> {
        let forms_str: String = fs::read_to_string(format!("{}/forms.yaml", static_path))?; /* ? /* ? */ */
        let roots_str: String = fs::read_to_string(format!("{}/roots.yaml", static_path))?; /* ? /* ? */ */
        
        let forms: Value = serde_yaml::from_str(&forms_str)?; /* ? /* ? */ */
        let roots: Value = serde_yaml::from_str(&roots_str)?; /* ? /* ? */ */

        Ok(Lang {
            forms,
            roots,
        })
    }

    pub fn render(&self, text: &str) -> String {
        let object = to_object(text);
        render_keywords(object, self)
    }
}