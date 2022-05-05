#![feature(test)]
extern crate test;
use emojito;
use test::Bencher;

#[bench]
fn empty(b: &mut Bencher) {
    b.iter(|| {
        let content = include_str!("tweets.json");
        emojito::find_emoji(&content);
    });
}
