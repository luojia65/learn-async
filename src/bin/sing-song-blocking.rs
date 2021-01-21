use futures::executor::block_on;

#[derive(Debug)]
struct Song;

async fn learn_song() -> Song { 
    println!("Learn!");
    Song 
}

async fn sing_song(song: Song) { 
    println!("Sing {:?}!", song);
}

async fn dance() { 
    println!("Dance!")
}

fn main() {
    let song = block_on(learn_song());
    block_on(sing_song(song));
    block_on(dance());
}
