struct Sense {}
impl Sense {
    fn sense(&self) -> bool {
        true
    }
    fn filter(&self) -> bool {
        true
    }
    fn store(&self) -> bool {
        true
    }
    fn access(&self) -> bool {
        true
    }
}

struct Catch {}
impl Catch {
    fn catch(&self) -> bool {
        true
    }
}

struct Consume {}
impl Consume {
    fn consume(&self) -> bool {
        true
    }
}

struct Digest {}
impl Digest {
    fn breakDown(&self) -> bool {
        true
    }
    fn transportEnergy(&self) -> bool {
        true
    }
    fn useEnergy(&self) -> bool {
        true
    }
}

struct Replicate {}
impl Replicate {
    fn replicate(&self) -> bool {
        true
    }
}

struct Defend {}
impl Defend {
    fn defend(&self) -> bool {
        true
    }
}

struct Move {}
impl Move {
    fn move_(&self) -> bool {
        true
    }
}

struct Communicate {}
impl Communicate {
    fn communicate(&self) -> bool {
        true
    }
}

struct Life {
    sense: Sense,
    catch: Catch,
    consume: Consume,
    digest: Digest,
    replicate: Replicate,
    defend: Defend,
    move_: Move,
    communicate: Communicate
}

// Path: main.rs
fn main() {
    println!("in main");
    let life = Life {
        sense: Sense {},
        catch: Catch {},
        consume: Consume {},
        digest: Digest {},
        replicate: Replicate {},
        defend: Defend {},
        move_: Move {},
        communicate: Communicate {},
    };
}