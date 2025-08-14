use std::fmt::format;
use std::fs;
use std::error::Error;
use std::ops::IndexMut;

use serde_yaml::{from_value, Value};

use super::*;
use super::super::debug::*;

pub struct Lang {
    forms: Value,
    roots: Value,
}

fn get_type<T>(keys: Vec<&str>, value: &Value) -> Result<T, String>
where T: serde::de::DeserializeOwned {
    // {dbg!(&keys);
    // dbg!(value);}
    let mut spot = value;
    for i in 0..(keys.len()-1) {
        spot = &spot[keys[i]];
    };
    let t: T = from_value(spot[keys[keys.len()-1]].clone())
        .ctx(format!("Invalid parameter {} in value", keys.join("/")))?;
    Ok(t)
}

impl Keyword {
    pub fn render(&self, lang: &Lang) -> Result<String, String> {
        Ok(match &self {
            Keyword::Adjective(root) => {
                Keyword::format_base("adjective", root, lang)
                    .ctx(format!("Error formatting adjective with base `{}`", root))?
            },
            Keyword::Nominative(root, plural) => {
                format!(
                    "{}{}",
                    Keyword::format_base("nominative", root, lang)
                        .ctx(format!("`Error formatting nominative with base {}`", root))?,
                    if *plural {
                        let ending: String = get_type(vec!["noun_plurality_suffix"], &lang.forms)
                            .ctx("Error loading plurality ending, check forms.yaml")?;
                        ending
                    } else {
                        "".to_string()
                    }
                )
            },
            Keyword::Verbal(root, form) => {
                format!(
                    "{}{}",
                    Keyword::format_base("verbal", root, lang)
                    .ctx(format!("`Error formatting verbal with base {}`", root))?, 
                    {
                        let form_keys = match form {
                            VerbForm::TensePresent => vec!["verb_tense_endings", "present"],
                            VerbForm::TenseFuture => vec!["verb_tense_endings", "future"],
                            VerbForm::TensePast => vec!["verb_tense_endings", "past"],
                            VerbForm::Infinitive => vec!["verb_infinitive_suffix"],
                        };
                        let ending: String = get_type(form_keys.clone(), &lang.forms)
                            .ctx(format!("`Error getting verb ending with modifiers {}`", form_keys.join(" and ")))?;
                        ending
                    }
                )
            },
            Keyword::VerbalAdjective(root) => {Keyword::format_base("verbal_adjective", root, lang).ctx(format!("`Error formatting verbal adjective with base {}`", root))?},
            Keyword::Prepositional(root) => {Keyword::format_base("prepositional", root, lang).ctx(format!("`Error formatting prepositional with base {}`", root))?},
            Keyword::AdjectAdjective(root) => {Keyword::format_base("adject_adjective", root, lang).ctx(format!("`Error formatting adject adjective with base {}`", root))?},

            Keyword::CompletiveAspect => get_type(vec!["verb_particles", "aspect", "completive"], &lang.forms).ctx("Error getting completive aspect marker")?,
            Keyword::ProgressiveAspect => get_type(vec!["verb_particles", "aspect", "progressive"], &lang.forms).ctx("Error getting progressive aspect marker")?,
            Keyword::HabitualAspect => get_type(vec!["verb_particles", "aspect", "habitual"], &lang.forms).ctx("Error getting habitual aspect marker")?,
            Keyword::PerfectAspect => get_type(vec!["verb_particles", "aspect", "perfect"], &lang.forms).ctx("Error getting perfect aspect marker")?,

            Keyword::DefiniteArticle(deixis) => get_type(vec!["article", "definite", deixis.as_str()], &lang.forms).ctx(format!("`Error getting definite article with deixis {:?}`", deixis))?, 
            Keyword::IndefiniteArticle(deixis) => get_type(vec!["article", "indefinite", deixis.as_str()], &lang.forms).ctx(format!("`Error getting indefinite article with deixis {:?}`", deixis))?, 
            Keyword::DeicticSpatialNoun(deixis) => get_type(vec!["deictic_nouns", "spatial", deixis.as_str()], &lang.forms).ctx(format!("`Error getting spatial noun with deixis {:?}`", deixis))?, 
            Keyword::DeicticTemporalNoun(deixis) => get_type(vec!["deictic_nouns", "temporal", deixis.as_str()], &lang.forms).ctx(format!("`Error getting temporal noun with deixis {:?}`", deixis))?, 
        })
    }

    fn format_base(of_type: &str, root: &str, lang: &Lang) -> Result<String, String> {
        let cons: String = get_type(vec![root], &lang.roots).ctx("error")?;
        let mold: String = get_type(vec![
            "root_form",
            cons.len().to_string().as_str(),
            of_type
        ], &lang.forms).ctx("error")?; 
        let cons = cons.as_str(); let mold = mold.as_str();
        
        Keyword::format_patterns(cons, mold)  
    }

    fn format_patterns(cons_pattern: &str, mold_pattern: &str) -> Result<String, String> {
        let mut out_string: Vec<char> = mold_pattern.chars().collect();
        for i in 0..cons_pattern.len() {
            let out_string_str: String = out_string.iter().cloned().collect();
            let replace = match out_string_str.find("-") {
                Some(u) => u,
                None => return Err("Pattern is missing one or more indicators for consonant patters (denoted by `-`)!".to_string())
            };
            out_string[replace] = match cons_pattern.chars().nth(i) {
                Some(c) => c,
                None => return Err("An unknown error has occured".to_string())
            };
        }

        let out: String = out_string.into_iter().collect();
        Ok(out)
    }
}

pub fn render_keywords(keywords: Vec<Keyword>, lang: &Lang) -> Result<String, String> {
    let mut out_string = String::new();
    for keyword in keywords {
        let rendered = keyword.render(lang)?;
        out_string.push_str(&rendered.as_str());
        out_string.push(' ');
    }
    Ok(out_string)
}

impl Lang {
    pub fn load(static_path: &str) -> Result<Lang, Box<dyn Error>> {
        let forms_str: String = fs::read_to_string(format!("{}/forms.yaml", static_path)).ctx(format!("Failed to load forms.yaml from {}", static_path))?; 
        let roots_str: String = fs::read_to_string(format!("{}/roots.yaml", static_path)).ctx(format!("Failed to load roots.yaml  from {}", static_path))?; 
        
        let forms: Value = serde_yaml::from_str(&forms_str).ctx("Failed to parse forms.yaml")?; 
        let roots: Value = serde_yaml::from_str(&roots_str).ctx("Failed to parse roots.yaml")?; 

        Ok(Lang {
            forms,
            roots,
        })
    }

    pub fn render(&self, text: &str) -> Result<String, String> {
        let object = to_object(text)?;
        Ok(render_keywords(object, self)?)
    }
}