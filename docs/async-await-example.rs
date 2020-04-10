use futures::executor::block_on;

type Song = String;

async fn learn_song() -> Song {
    println!("learn_song()");
    return "Jingle Bells".to_string();
}

async fn sing_song(song: Song) {
    println!("sing_song(\"{}\")", song);
}

async fn dance() {
    println!("dance()");
}

async fn learn_and_sing() {
    // Wait until the song has been learned before singing it.
    // We use `.await` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();
    
    // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.
    println!("async_main() before futures::join");

    // From here we needs f1 and f2 results
    futures::join!(f1, f2);
}

fn main() {
    block_on(async_main());
}
