use glicol::Engine;
// use glicol::GlicolNodeInfo;
// use std::collections::HashMap;

fn main() {
    let mut engine = Engine::<8>::new();
    engine.livecoding = false;
    // engine.update(r#"o: constsig 42 >> mul 0.3"#);
    // engine.next_block();
    // engine.update(r#"o: constsig 42 >> mul ~mod; ~mod: constsig 0.9"#);
    // engine.next_block();
    // engine.update(r#"o: constsig 42 >> mul 0.3"#);
    // engine.next_block();
    // engine.update(r#"o: constsig 42 >> mul ~mod; ~mod: constsig 0.9"#);
    // engine.next_block();
   
    // engine.update(r#"o: constsig 42. >> add 0 >> mul ~mod; ~mod: constsig 0.5"#).unwrap();
    // engine.next_block();
    // engine.update(r#"o: constsig 42. >> mul ~mod; ~mod: constsig 0.5"#).unwrap();
    // engine.next_block();

    // engine.update(r#"o: imp 10 >> mul 0.1"#).unwrap();
    // engine.next_block();
    // engine.update(r#"o: saw 56"#).unwrap();
    // engine.next_block();
    // engine.update(r#"o: imp 100 >> mul 0.1"#).unwrap();
    // engine.next_block();
    // engine.update(r#"o: imp 100 >> mul ~mod
    // ~mo: sin 10 >> add 1.0"#).unwrap();
    // engine.next_block();


    // engine.update(r#"o: sin 110 >> mul 0.1"#).unwrap();
    // println!(" engine.next_block() {:?}", engine.next_block());
    // engine.update(r#"o: sin 110 >> add 0.0"#).unwrap();
    // println!(" engine.next_block() {:?}", engine.next_block());
    // engine.update(r#"o: sin 110 >> mul 0.1"#).unwrap();
    // println!(" engine.next_block() {:?}", engine.next_block());

    // engine.update(r#"o: seq ~a; ~a: choose 60"#).unwrap();
    // println!(" engine.next_block() {:?}", engine.next_block());
    // engine.update(r#"o: seq ~a; ~a: choose 70"#).unwrap();
    // println!(" engine.next_block() {:?}", engine.next_block());

    // engine.update(r#"o: sin 10"#).unwrap();
    // println!(" engine.next_block() {:?}", engine.next_block());
    // engine.update(r#"o: saw 12"#).unwrap();
    // println!(" engine.next_block() {:?}", engine.next_block());
    
    // engine.update_with_code(r#"a: constsig 10 >> lpf 300 0.1"#);
    // println!(" engine.next_block() 0 {:?}", engine.next_block().0);
    // engine.update_with_code(r#"a: constsig 10 >> lpf ~m 0.1; ~m: constsig 0.5"#);
    // println!(" engine.next_block() 1 {:?}", engine.next_block().0);
    engine.add_sample(r#"\test"#, &[0.9, 0.8, 0.7, 0.6, 0.5], 1, 44100);
    engine.update_with_code(r#"a: seq 60 >> sp \test"#);
    println!(" engine.next_block() 0 {:?}", engine.next_block().0);
    engine.update_with_code(r#"a: speed 4.0 >> seq ~b >> sp \test; ~b: choose 59 72 0"#);
    println!(" engine.next_block() 1 {:?}", engine.next_block().0);
}