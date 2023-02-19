#[cfg(feature = "fake")]
fn main() {
    use domain::episode::Episode;
    use fake::{Fake, Faker};

    let episodes = (0..10).map(|_| Faker.fake::<Episode>()).collect::<Vec<_>>();
    println!("{episodes:?}");
}

#[cfg(not(feature = "fake"))]
fn main() {}
