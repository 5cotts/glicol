use pest::Parser;
use pest::error::Error;
use pest_derive::*;
// use pest::iterators::Pair;
// use pest::error::ErrorVariant;
use hashbrown::HashMap;

use glicol_macros::{one_para_number_or_ref, two_numbers};
use glicol_synth::GlicolPara;

#[derive(Parser)]
#[grammar = "glicol.pest"]
pub struct GlicolParser;

pub fn get_num(para: GlicolPara) -> f32 {
    match para {
        GlicolPara::Number(v) => v,
        _ => 0.0
    }
}

/// index, (vector of chain name, vector of parameter list)
pub type GlicolAst = HashMap<String, (Vec<String>, Vec<Vec<GlicolPara>>)>;

pub fn get_ast(code: &str) -> Result<GlicolAst, Error<Rule>> {
    let mut block = match GlicolParser::parse(Rule::block, code) {
        Ok(v) => v,
        Err(e) => { return Err(e) }
    };
    // this can be a comment though, but we call it a line
    let lines = block.next().unwrap();
    let mut ast = HashMap::new();
    for line in lines.into_inner() {
        match line.as_rule() {
            Rule::line => {
                println!("each line {:?}", line.as_str());
                let mut key = "";
                let mut chain_node_names = vec![];
                let mut chain_paras = vec![];
                for line_component in line.into_inner() {
                    match line_component.as_rule() {
                        Rule::reference => {
                            println!("ref {:?}", line_component.as_str());
                            key = line_component.as_str();
                        },
                        Rule::chain => {
                            println!("chain {:?}", line_component.as_str());
                            let chain = line_component;
                            for node_pair in chain.into_inner() {
                                let node = node_pair.into_inner().next().unwrap();
                                match node.as_rule() {
                                    Rule::delayn =>  one_para_number_or_ref!("delayn"),
                                    Rule::delayms =>  one_para_number_or_ref!("delayms"),
                                    Rule::imp =>  one_para_number_or_ref!("imp"),
                                    Rule::tri =>  one_para_number_or_ref!("tri"),
                                    Rule::squ => one_para_number_or_ref!("squ"),
                                    Rule::saw => one_para_number_or_ref!("saw"),
                                    Rule::onepole => one_para_number_or_ref!("onepole"),
                                    Rule::sin => one_para_number_or_ref!("sin"),
                                    Rule::mul => one_para_number_or_ref!("mul"),
                                    Rule::add => one_para_number_or_ref!("add"),
                                    Rule::seq => {
                                        let mut event = Vec::<(f32, GlicolPara)>::new();
                                        println!("node {:?}", node.as_str());
                                        let paras = node.into_inner().next().unwrap();
                                        println!("paras {:?}", paras.as_str());
                                        chain_node_names.push("seq");
                                        // to do, more than a symbol
                                        // should be an event that contains time and note
                                        // GlicolPara::Symbol(paras.as_str())
                                        let compounds: Vec<_> = paras.into_inner().collect();
                                        // one bar will firstly be divided here
                                        let compounds_num = compounds.len();
                                        // println!("{:?}", );
                                        for (i, compound) in compounds.into_iter().enumerate() {
                                            let relative_time_base = i as f32 /compounds_num as f32;
                                            let elements: Vec<_> = compound.into_inner().collect();
                                            let elements_n = elements.len();

                                            for (j, element) in elements.into_iter().enumerate() {
                                                let relative_time_sub = 1./ compounds_num as f32 * j as f32 / elements_n as f32;
                                                let e = element.into_inner().next().unwrap();
                                                let time = relative_time_sub + relative_time_base;
                                                match e.as_rule() {
                                                    Rule::integer => {
                                                        event.push( (time, GlicolPara::Number(e.as_str().parse::<f32>().unwrap()) ));
                                                        println!("int {:?}", e.as_str());
                                                    },
                                                    Rule::rest => {
                                                        println!("rest {:?}", e.as_str());
                                                        // event.push( (time , GlicolPara::Number(0.0) ));
                                                    },
                                                    Rule::note_ref => {
                                                        println!("ref {:?}", e.as_str());
                                                        event.push( (time, GlicolPara::Reference(e.as_str().to_owned()) ));
                                                    },
                                                    _=> unimplemented!()
                                                }
                                            }
                                        }
                                        chain_paras.push(vec![GlicolPara::Sequence(event)]);
                                    },
                                    Rule::choose => {
                                        println!("node {:?}", node.as_str());
                                        let paras: Vec<_> = node.into_inner().map(|x|x.as_str().parse::<f32>().unwrap()).collect();
                                        println!("paras {:?}", paras);
                                        chain_node_names.push("choose");
                                        chain_paras.push(vec![GlicolPara::NumberList(paras)]);
                                    },
                                    Rule::mix => {
                                        println!("node {:?}", node.as_str());
                                        let paras: Vec<_> = node.into_inner().map(|x| GlicolPara::Reference(x.as_str().to_owned()) ).collect();
                                        println!("paras {:?}", paras);
                                        chain_node_names.push("mix");
                                        chain_paras.push(paras);
                                    },
                                    Rule::sp => {
                                        println!("node {:?}", node.as_str());
                                        let paras = node.into_inner().next().unwrap();
                                        println!("paras {:?}", paras.as_str());
                                        chain_node_names.push("sp");
                                        chain_paras.push(vec![GlicolPara::SampleSymbol(paras.as_str().to_owned())]);
                                    },
                                    Rule::speed => one_para_number_or_ref!("speed"),
                                    Rule::constsig => one_para_number_or_ref!("constsig"),
                                    Rule::adc => one_para_number_or_ref!("adc"),
                                    Rule::bd => one_para_number_or_ref!("bd"),
                                    Rule::sn => one_para_number_or_ref!("sn"),
                                    Rule::hh => one_para_number_or_ref!("hh"),
                                    Rule::sawsynth => two_numbers!("sawsynth"),
                                    Rule::squsynth => two_numbers!("squsynth"),
                                    Rule::trisynth => two_numbers!("trisynth"),
                                    Rule::lpf => {
                                        println!("node {:?}", node.as_str());
                                        let mut iter = node.into_inner();
                                        let p1 = iter.next().unwrap();
                                        let p2 = iter.next().unwrap();
                                        chain_node_names.push("lpf");
                                        chain_paras.push(vec![
                                            match p1.as_rule() {
                                                Rule::number => 
                                                    GlicolPara::Number(p1.as_str().parse::<f32>().unwrap())
                                                ,
                                                Rule::reference => 
                                                    GlicolPara::Reference(p1.as_str().to_owned())
                                                ,
                                                _ => unimplemented!()
                                            },
                                            GlicolPara::Number(p2.as_str().parse::<f32>().unwrap())
                                        ]);
                                        // println!("chain_paras, {:?}", chain_paras);
                                    },
                                    Rule::balance => {
                                        println!("node {:?}", node.as_str());
                                        let mut iter = node.into_inner();
                                        let p1 = iter.next().unwrap();
                                        let p2 = iter.next().unwrap();
                                        chain_node_names.push("balance");
                                        chain_paras.push(vec![
                                            GlicolPara::Reference(p1.as_str().to_owned()),
                                            GlicolPara::Reference(p2.as_str().to_owned()),
                                        ]);
                                        // println!("chain_paras, {:?}", chain_paras);
                                    },
                                    Rule::rhpf => {
                                        println!("node {:?}", node.as_str());
                                        let mut iter = node.into_inner();
                                        let p1 = iter.next().unwrap();
                                        let p2 = iter.next().unwrap();
                                        chain_node_names.push("rhpf");
                                        chain_paras.push(vec![
                                            match p1.as_rule() {
                                                Rule::number => 
                                                    GlicolPara::Number(p1.as_str().parse::<f32>().unwrap())
                                                ,
                                                Rule::reference => 
                                                    GlicolPara::Reference(p1.as_str().to_owned())
                                                ,
                                                _ => unimplemented!()
                                            },
                                            GlicolPara::Number(p2.as_str().parse::<f32>().unwrap())
                                        ]);
                                        // println!("chain_paras, {:?}", chain_paras);
                                    },
                                    Rule::apfmsgain => {
                                        println!("node {:?}", node.as_str());
                                        let mut iter = node.into_inner();
                                        let p1 = iter.next().unwrap();
                                        let p2 = iter.next().unwrap();
                                        chain_node_names.push("apfmsgain");
                                        chain_paras.push(vec![
                                            match p1.as_rule() {
                                                Rule::number => 
                                                    GlicolPara::Number(p1.as_str().parse::<f32>().unwrap())
                                                ,
                                                Rule::reference => 
                                                    GlicolPara::Reference(p1.as_str().to_owned())
                                                ,
                                                _ => unimplemented!()
                                            },
                                            GlicolPara::Number(p2.as_str().parse::<f32>().unwrap())
                                        ]);
                                        // println!("chain_paras, {:?}", chain_paras);
                                    },
                                    Rule::envperc => {
                                        println!("node {:?}", node.as_str());
                                        let mut iter = node.into_inner();
                                        let p1 = iter.next().unwrap();
                                        let p2 = iter.next().unwrap();
                                        chain_node_names.push("envperc");
                                        chain_paras.push(vec![
                                            GlicolPara::Number(p1.as_str().parse::<f32>().unwrap()),
                                            GlicolPara::Number(p2.as_str().parse::<f32>().unwrap())
                                        ]);
                                        // println!("chain_paras, {:?}", chain_paras);
                                    },
                                    Rule::plate => one_para_number_or_ref!("plate"),
                                    Rule::get => one_para_number_or_ref!("get"),
                                    Rule::noise => one_para_number_or_ref!("noise"),
                                    Rule::meta => {
                                        println!("node {:?}", node.as_str());
                                        let paras = node.into_inner().next().unwrap();
                                        println!("paras {:?}", paras.as_str());
                                        chain_node_names.push("meta");
                                        chain_paras.push(vec![GlicolPara::Symbol(paras.as_str().to_owned())]);
                                    },
                                    _ => unimplemented!()
                                }
                            }
                            // println!("chain.into_inner(); {:?}", chain.into_inner());
                        },
                        _ => {}
                    }
                }
                ast.insert(
                    key.to_owned(), 
                    (
                        chain_node_names.iter_mut().map(|x|x.to_owned()).collect::<Vec<String>>(), 
                        chain_paras
                    )
                );
            },
            _ => {},
        };
    }
    Ok(ast)
}