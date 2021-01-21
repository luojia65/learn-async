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

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();
    futures::join!(f1, f2);
}

fn main() {
    block_on(async_main())
}
